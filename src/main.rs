#![allow(dead_code, unused)]

use std::io;

mod app;
use app::App;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| {
        let mut app = App::default();

        app.run(terminal, |app: &mut App| {
            // main loop logic
            if app.counter > 7 {
                app.counter = 2;
            }
            if app.counter < 2 {
                app.counter = 7;
            }

        })
    });
    
    Ok(())
}