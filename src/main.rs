// use rv_simulator::Builder;
// use std::io::stdin;
// fn main() {
//     let instruction_memory: Vec<u32> = vec![
//         0xe5010113, // addi x2 x2 -432
//         0x1a812623, // sw x8 428 x2
//         0x1b010413, // addi x8 x2 432
//         0xfe042623, // sw x0 -20 x8
//         0x0280006f, // jal x0 40
//         0xfec42783, // lw x15 -20 x8
//     ];
//     let asm_mem = vec![
//         "addi x2 x2 -432".to_string(),
//         "sw x8 428 x2".to_string(),
//         "addi x8 x2 432".to_string(),
//         "sw x0 -20 x8".to_string(),
//         "jal x0 40".to_string(),
//         "lw x15 -20 x8".to_string(),
//     ];
//     let rv = rv_simulator::Rv32iBuilder::new(instruction_memory, asm_mem)
//         .build()
//         .unwrap();
//     let mut count = 0;
//     loop {
//         let mut buf = String::new();
//         stdin().read_line(&mut buf).unwrap();
//         println!("Cycle: {}", count);
//         println!("Rasising edge");
//         rv.rasing_edge();
//         #[cfg(debug_assertions)]
//         println!("{}", rv.debug());
//         println!("Falling edge");
//         rv.falling_edge();
//         #[cfg(debug_assertions)]
//         println!("{}", rv.debug());
//         count += 1;
//     }
// }

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

mod tui;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        // frame.render_widget(self, frame.size());
        let chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(12),
                Constraint::Percentage(36),
                Constraint::Percentage(24),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
            ])
            .split(frame.size());
        let titles = [" IF ", " ID ", " EX ", " MEM ", " WB "]
            .into_iter()
            .map(Title::from);
        let blocks = titles
            .map(|title| Block::default().title(title).borders(Borders::ALL))
            .collect::<Vec<_>>();
        blocks
            .into_iter()
            .zip(chunk.iter())
            .for_each(|(block, chunk)| {
                frame.render_widget(block, *chunk);
            });
    }

    /// updates the application's state based on user input
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
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(12),
                Constraint::Percentage(36),
                Constraint::Percentage(24),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
            ])
            .split(area);
        let titles = [" IF ", " ID ", " EX ", " MEM ", " WB "]
            .into_iter()
            .map(Title::from);
        let blocks = titles
            .map(|title| Block::default().title(title).borders(Borders::ALL))
            .collect::<Vec<_>>();
        blocks
            .into_iter()
            .for_each(|block| block.render(chunk[0], buf));
        // let instructions = Title::from(Line::from(vec![
        //     " Decrement ".into(),
        //     "<Left>".blue().bold(),
        //     " Increment ".into(),
        //     "<Right>".blue().bold(),
        //     " Quit ".into(),
        //     "<Q> ".blue().bold(),
        // ]));
        // let block = Block::default()
        //     .title(title.alignment(Alignment::Center))
        //     .title(
        //         instructions
        //             .alignment(Alignment::Center)
        //             .position(Position::Bottom),
        //     )
        //     .borders(Borders::ALL)
        //     .border_set(border::THICK);

        // let counter_text = Text::from(vec![Line::from(vec![
        //     "Value: ".into(),
        //     self.counter.to_string().yellow(),
        // ])]);

        // Paragraph::new(counter_text)
        //     .centered()
        //     .block(block)
        //     .render(area, buf);
    }
}
