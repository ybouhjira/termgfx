//! TermGFX Studio - Fullscreen interactive component explorer
//!
//! A TUI application that provides an IDE-like experience for exploring
//! and configuring termgfx components with live preview.

mod app;
mod layout;
mod registry;
mod storage;
mod ui;
pub mod widgets;

pub use app::run_studio;
