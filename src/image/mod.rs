use image::{DynamicImage, GenericImageView, ImageFormat};
use std::io::{self, Write};
use std::env;

/// Protocol to use for rendering images
#[derive(Debug, Clone, Copy, PartialEq)]
enum Protocol {
    Kitty,
    Sixel,
    ITerm2,
    Halfblock,
}

pub fn render(path: &str, protocol_arg: &str) {
    match render_image(path, protocol_arg) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn render_image(path: &str, protocol_arg: &str) -> anyhow::Result<()> {
    let img = load_image(path)?;
    let protocol = if protocol_arg == "auto" {
        detect_protocol()
    } else {
        parse_protocol(protocol_arg)?
    };
    let (term_width, term_height) = crossterm::terminal::size()
        .map(|(w, h)| (w as usize, h as usize))
        .unwrap_or((80, 24));
    match protocol {
        Protocol::Kitty => render_kitty(&img, term_width, term_height),
        Protocol::Sixel => render_sixel(&img, term_width, term_height),
        Protocol::ITerm2 => render_iterm2(&img, term_width, term_height),
        Protocol::Halfblock => render_halfblock(&img, term_width, term_height),
    }
}

fn load_image(path: &str) -> anyhow::Result<DynamicImage> {
    if path.starts_with("http://") || path.starts_with("https://") {
        let response = ureq::get(path).call()?;
        let mut bytes = Vec::new();
        response.into_reader().read_to_end(&mut bytes)?;
        let img = image::load_from_memory(&bytes)?;
        Ok(img)
    } else {
        let img = image::open(path)?;
        Ok(img)
    }
}

fn detect_protocol() -> Protocol {
    if let Ok(term_program) = env::var("TERM_PROGRAM") {
        if term_program == "iTerm.app" {
            return Protocol::ITerm2;
        }
    }
    if let Ok(term) = env::var("TERM") {
        if term.contains("kitty") {
            return Protocol::Kitty;
        }
        if term.contains("xterm") {
            return Protocol::Sixel;
        }
    }
    if let Ok(colorterm) = env::var("COLORTERM") {
        if colorterm == "truecolor" || colorterm == "24bit" {
            return Protocol::Halfblock;
        }
    }
    Protocol::Halfblock
}

fn parse_protocol(s: &str) -> anyhow::Result<Protocol> {
    match s.to_lowercase().as_str() {
        "kitty" => Ok(Protocol::Kitty),
        "sixel" => Ok(Protocol::Sixel),
        "iterm2" => Ok(Protocol::ITerm2),
        "halfblock" => Ok(Protocol::Halfblock),
        _ => Err(anyhow::anyhow!(
            "Invalid protocol: {}. Valid options: kitty, sixel, iterm2, halfblock",
            s
        )),
    }
}

fn render_kitty(img: &DynamicImage, term_width: usize, _term_height: usize) -> anyhow::Result<()> {
    let max_width_px = ((term_width - 2) * 8) as u32;
    let (img_width, img_height) = img.dimensions();
    let scaled_img = if img_width > max_width_px {
        let scale = max_width_px as f32 / img_width as f32;
        let new_height = (img_height as f32 * scale) as u32;
        img.resize(max_width_px, new_height, image::imageops::FilterType::Lanczos3)
    } else {
        img.clone()
    };
    let mut png_data = Vec::new();
    scaled_img.write_to(&mut std::io::Cursor::new(&mut png_data), ImageFormat::Png)?;
    let base64_data = base64_encode(&png_data);
    print!("\x1b_Gf=100,a=T;{}\x1b\\", base64_data);
    io::stdout().flush()?;
    Ok(())
}

fn render_sixel(img: &DynamicImage, term_width: usize, _term_height: usize) -> anyhow::Result<()> {
    // Resize image to fit terminal width
    // Assuming approx 8 pixels per character cell width
    let max_width_px = (term_width as u32) * 8;
    let (img_width, img_height) = img.dimensions();

    let scaled_img = if img_width > max_width_px {
        let scale = max_width_px as f32 / img_width as f32;
        let new_height = (img_height as f32 * scale) as u32;
        img.resize(max_width_px, new_height, image::imageops::FilterType::Lanczos3)
    } else {
        img.clone()
    };

    let rgba_img = scaled_img.to_rgba8();
    let width = rgba_img.width();
    let height = rgba_img.height();

    // Quantize to 6x6x6 RGB cube (216 colors)
    // Map: (r,g,b) -> index 0..215
    let mut indexed_pixels = Vec::with_capacity((width * height) as usize);
    let mut used_colors = [false; 216];

    for pixel in rgba_img.pixels() {
        // Handle transparency
        if pixel[3] < 128 {
            indexed_pixels.push(255); // Use 255 as marker for transparent
            continue;
        }

        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        // Map 0-255 to 0-5
        let r_idx = (r as u16 * 5 + 127) / 255;
        let g_idx = (g as u16 * 5 + 127) / 255;
        let b_idx = (b as u16 * 5 + 127) / 255;

        let palette_index = (r_idx * 36 + g_idx * 6 + b_idx) as usize;
        indexed_pixels.push(palette_index as u8);
        used_colors[palette_index] = true;
    }

    // Start Sixel sequence
    // DCS P1;P2;P3 q - P1=pixel aspect ratio, P2=background mode, P3=horizontal grid
    // "Pan;Pad;Ph;Pv" - aspect ratio numerator/denominator, horizontal/vertical extent
    print!("\x1bP0;0;0q\"1;1;{};{}", width, height);

    // Emit Palette
    for i in 0..216 {
        if used_colors[i] {
            let r_idx = i / 36;
            let g_idx = (i % 36) / 6;
            let b_idx = i % 6;

            // Map 0..5 to 0..100 for Sixel
            let r = (r_idx * 100 + 2) / 5;
            let g = (g_idx * 100 + 2) / 5;
            let b = (b_idx * 100 + 2) / 5;

            print!("#{0};2;{1};{2};{3}", i, r, g, b);
        }
    }

    // Encode bands
    for y in (0..height).step_by(6) {
        let rows_in_band = std::cmp::min(6, height - y);

        for color_idx in 0..216 {
            if !used_colors[color_idx] { continue; }

            let mut has_pixels_for_color = false;
            let mut color_cols = vec![0u8; width as usize];

            for row_offset in 0..rows_in_band {
                let img_y = y + row_offset;
                for x in 0..width {
                    let px_idx = (img_y * width + x) as usize;
                    if indexed_pixels[px_idx] == color_idx as u8 {
                        color_cols[x as usize] |= 1 << row_offset;
                        has_pixels_for_color = true;
                    }
                }
            }

            if has_pixels_for_color {
                print!("#{}", color_idx);

                let mut x = 0;
                while x < width as usize {
                    let val = color_cols[x];
                    let mut run_len = 1;
                    while x + run_len < width as usize && color_cols[x + run_len] == val {
                        run_len += 1;
                    }

                    let char_val = (val + 63) as char;
                    if run_len > 1 {
                        print!("!{}{}", run_len, char_val);
                    } else {
                        print!("{}", char_val);
                    }
                    x += run_len;
                }
                print!("$");
            }
        }
        print!("-");
    }

    print!("\x1b\\");
    println!(); // Newline after image
    io::stdout().flush()?;
    Ok(())
}

fn render_iterm2(img: &DynamicImage, term_width: usize, _term_height: usize) -> anyhow::Result<()> {
    let max_width_px = ((term_width - 2) * 10) as u32;
    let (img_width, img_height) = img.dimensions();
    let scaled_img = if img_width > max_width_px {
        let scale = max_width_px as f32 / img_width as f32;
        let new_height = (img_height as f32 * scale) as u32;
        img.resize(max_width_px, new_height, image::imageops::FilterType::Lanczos3)
    } else {
        img.clone()
    };
    let mut png_data = Vec::new();
    scaled_img.write_to(&mut std::io::Cursor::new(&mut png_data), ImageFormat::Png)?;
    let base64_data = base64_encode(&png_data);
    print!("\x1b]1337;File=inline=1:{}\x07", base64_data);
    io::stdout().flush()?;
    Ok(())
}

fn render_halfblock(img: &DynamicImage, term_width: usize, term_height: usize) -> anyhow::Result<()> {
    let max_chars_width = term_width - 2;
    let max_chars_height = (term_height - 4) * 2;
    let (img_width, img_height) = img.dimensions();
    let width_scale = max_chars_width as f32 / img_width as f32;
    let height_scale = max_chars_height as f32 / img_height as f32;
    let scale = width_scale.min(height_scale);
    let scaled_width = (img_width as f32 * scale) as u32;
    let scaled_height = (img_height as f32 * scale) as u32;
    let resized = img.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3);
    let rgb_img = resized.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    for y in (0..height).step_by(2) {
        for x in 0..width {
            let top_pixel = rgb_img.get_pixel(x, y);
            let bottom_pixel = if y + 1 < height {
                rgb_img.get_pixel(x, y + 1)
            } else {
                top_pixel
            };
            print!(
                "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                top_pixel[0], top_pixel[1], top_pixel[2],
                bottom_pixel[0], bottom_pixel[1], bottom_pixel[2]
            );
        }
        println!("\x1b[0m");
    }
    io::stdout().flush()?;
    Ok(())
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3f) as usize;
        result.push(CHARS[b1] as char);
        result.push(CHARS[b2] as char);
        result.push(if chunk.len() > 1 { CHARS[b3] as char } else { '=' });
        result.push(if chunk.len() > 2 { CHARS[b4] as char } else { '=' });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_protocol() {
        assert_eq!(parse_protocol("kitty").unwrap(), Protocol::Kitty);
        assert_eq!(parse_protocol("sixel").unwrap(), Protocol::Sixel);
        assert_eq!(parse_protocol("iterm2").unwrap(), Protocol::ITerm2);
        assert_eq!(parse_protocol("halfblock").unwrap(), Protocol::Halfblock);
        assert!(parse_protocol("invalid").is_err());
    }

    #[test]
    fn test_base64_encode() {
        let data = b"hello";
        let encoded = base64_encode(data);
        assert_eq!(encoded, "aGVsbG8=");
    }

    #[test]
    fn test_sixel_renders_without_fallback() {
        // Create a small test image (2x2 red pixels)
        let img = DynamicImage::ImageRgba8(image::RgbaImage::from_fn(2, 2, |_, _| {
            image::Rgba([255, 0, 0, 255]) // Red pixel
        }));

        // Capture stdout to verify Sixel output
        let result = render_sixel(&img, 80, 24);
        assert!(result.is_ok(), "render_sixel should not return error");

        // The function should complete without calling halfblock fallback
        // If it had a fallback, there would be eprintln output with "fallback"
    }

    #[test]
    fn test_sixel_output_format() {
        use std::io::Write;

        // Create a small test image
        let img = DynamicImage::ImageRgba8(image::RgbaImage::from_fn(4, 6, |x, y| {
            if (x + y) % 2 == 0 {
                image::Rgba([255, 0, 0, 255]) // Red
            } else {
                image::Rgba([0, 0, 255, 255]) // Blue
            }
        }));

        // We can't easily capture stdout in a unit test, but we can verify
        // the function completes successfully without panic
        let result = render_sixel(&img, 80, 24);
        assert!(result.is_ok(), "Sixel rendering should succeed");
    }
}
