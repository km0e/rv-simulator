use std::io::{self, stdout, Stdout};
mod signal;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::*,
};
use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};

use crate::{abi::Control, simulator::Rv32i};

/// A type alias for the terminal type used in this application
pub type Backend = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Backend> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

#[derive(Debug)]
pub struct App {
    simulator: Rv32i,
    exit: bool,
}

impl App {
    pub fn new(sm: Rv32i) -> Self {
        Self {
            simulator: sm,
            exit: false,
        }
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Backend) -> io::Result<()> {
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
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(frame.size());
        let signals = [
            self.simulator.if_id.inout(),
            self.simulator.id_ex.inout(),
            self.simulator.ex_mem.inout(),
            self.simulator.mem_wb.inout(),
        ];
        chunk
            .iter()
            .zip(signals)
            .zip([" IF/ID ", " ID/EX ", " EX/MEM ", " MEM/WB "].iter())
            .for_each(|((chunk, signal), title)| {
                let rows = signal.into_iter().map(|(n, in_, out)| {
                    Row::new(vec![n, format!("{:x}", in_), format!("{:x}", out)])
                });
                let table = Table::new(
                    rows,
                    [
                        Constraint::Fill(1),
                        Constraint::Length(8),
                        Constraint::Length(8),
                    ],
                )
                .block(
                    Block::default()
                        .title(*title)
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL),
                )
                .header(Row::new(vec!["Name", "In", "Out"]))
                .column_spacing(1);

                frame.render_widget(table, *chunk);
                // let (names, values): (Vec<_>, Vec<_>) = signal
                //     .into_iter()
                //     .map(|(n, in_, out)| (n, (in_, out)))
                //     .unzip();
                // let chunks = Layout::default()
                //     .direction(Direction::Horizontal)
                //     .constraints([
                //         Constraint::Length(10),
                //         Constraint::Fill(1),
                //         Constraint::Fill(1),
                //     ])
                //     .split(chunks[1]);
                // let names = List::new(names.into_iter()).block(Block::default().title("Name"));
                // frame.render_widget(names, chunks[0]);

                // let (ins, outs): (Vec<_>, Vec<_>) = values.into_iter().unzip();

                // let ins = List::new(ins.into_iter().map(|in_| format!("{:x}", in_)))
                //     .block(Block::default().title("In"));
                // frame.render_widget(ins, chunks[1]);

                // let outs = List::new(outs.into_iter().map(|out_| format!("{:x}", out_)))
                //     .block(Block::default().title("Out"));
                // frame.render_widget(outs, chunks[2]);
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
            KeyCode::Left => self.prec_cycle(),
            KeyCode::Right => self.next_cycle(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn next_cycle(&mut self) {
        self.simulator.rasing_edge();
        self.simulator.falling_edge();
    }
    fn prec_cycle(&mut self) {
        // self.simulator.falling_edge();
        // self.simulator.rasing_edge();
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
    }
}
