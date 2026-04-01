#![allow(dead_code, unused)]

use std::io;

mod app;
use app::App;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal));
    Ok(())
}