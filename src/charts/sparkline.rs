const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

pub fn render(data: &str) {
    let values: Vec<f64> = data
        .split(',')
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect();

    if values.is_empty() {
        eprintln!("Error: No valid numeric values found");
        return;
    }

    if values.len() == 1 {
        print!("{}", BLOCKS[BLOCKS.len() - 1]);
        return;
    }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;

    for value in values {
        let normalized = if range == 0.0 {
            0.5
        } else {
            (value - min) / range
        };

        let index = (normalized * (BLOCKS.len() - 1) as f64).round() as usize;
        let index = index.min(BLOCKS.len() - 1);
        print!("{}", BLOCKS[index]);
    }

    println!();
}
