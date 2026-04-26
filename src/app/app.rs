extern crate ratatui;
extern crate crossterm;

use std::io::{self, Lines};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Borders, Paragraph, Widget}
};

#[derive(Default)]
pub struct App{
    pub counter: i32,
    exit: bool
}

struct TopWidget;

impl App {

    /// runs the application's main loop until the user quits
    pub fn run<F>(&mut self, terminal: &mut DefaultTerminal, mut update: F) -> io::Result<()> 
    where 
        F: FnMut(&mut App),
    {
        while !self.exit {
            update(self);

            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let [top, bottom] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(6), Constraint::Min(0)]) // MAGIC NUMBER HERE defines amount of lines for space for topwidget including border and title
            .areas(frame.area());

        frame.render_widget(TopWidget, top);
        frame.render_widget(self, bottom);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

        fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for TopWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Top Panel".bold())
            .borders(ratatui::widgets::Borders::BOTTOM)
            .border_set(border::DOUBLE);

        let text = vec![
            Line::from("line 1"),
            Line::from("line 2"),
            Line::from("line 3"),
            Line::from("line 4"),
        ];

        Paragraph::new(text)
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" some lines test ".bold());
        let instructions = Line::from(vec![
            "Remove Line ".into(),
            "<Left>".magenta().bold(),
            " Add Line ".into(),
            "<Right>".magenta().bold(),
            " Quit ".into(),
            "<Esc>".magenta().bold(),
        ]);
        let block = Block::default()
            .title(title.bold())
            .title_bottom(instructions.centered())
            .borders(Borders::NONE);


        let mut lines = Vec::new();
        for i in 0..self.counter {
            lines.push(Line::from(format!("{}. Line", i + 1)));
        }

        let counter_text = Text::from(lines);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

