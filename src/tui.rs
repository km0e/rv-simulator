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
    cycle: usize,
}

impl App {
    pub fn new(sm: Rv32i) -> Self {
        Self {
            simulator: sm,
            exit: false,
            cycle: 0,
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
    fn render_seps(&self, chunk: Rect, buffer: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(chunk);
        let signals = [
            self.simulator.if_id.inout(),
            self.simulator.id_ex.inout(),
            self.simulator.ex_mem.inout(),
            self.simulator.mem_wb.inout(),
        ];
        chunks
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
                Widget::render(table, *chunk, buffer);
            });
    }
    fn render_asm(&self, chunk: Rect, buffer: &mut Buffer) {
        let rows = self
            .simulator
            .asm
            .read(chunk.height as usize)
            .into_iter()
            .map(|asm| Row::new(vec![asm.stage.to_string(), asm.asm]))
            .collect::<Vec<_>>();
        let table = Table::new(rows, [Constraint::Length(10), Constraint::Percentage(95)])
            .block(
                Block::default()
                    .title(" ASM ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .header(Row::new(vec!["Stage", "Instruction"]))
            .column_spacing(1);
        Widget::render(table, chunk, buffer);
    }
    fn render_stage(&self, chunk: Rect, buffer: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunk);
        let rows = self
            .simulator
            .ex
            .output()
            .into_iter()
            .map(|(name, value)| Row::new(vec![name, format!("{:x}", value)]))
            .collect::<Vec<_>>();
        let table = Table::new(rows, [Constraint::Length(10), Constraint::Percentage(95)])
            .block(
                Block::default()
                    .title(" EX ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::NONE),
            )
            .column_spacing(1);
        Widget::render(table, chunks[0], buffer);

        let rows = self
            .simulator
            .hazard
            .output()
            .into_iter()
            .map(|(name, value)| Row::new(vec![name, format!("{:x}", value)]))
            .collect::<Vec<_>>();
        let table = Table::new(rows, [Constraint::Length(10), Constraint::Percentage(95)])
            .block(
                Block::default()
                    .title(" Hazard ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::NONE),
            )
            .column_spacing(1);
        Widget::render(table, chunks[1], buffer);
    }
    fn render_frame(&self, frame: &mut Frame) {
        let chunck = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(24),
                Constraint::Length(8),
                Constraint::Fill(1),
            ])
            .split(frame.size());
        self.render_seps(chunck[0], frame.buffer_mut());
        self.render_stage(chunck[1], frame.buffer_mut());
        self.render_asm(chunck[2], frame.buffer_mut());
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
        self.cycle += 1;
    }
    fn prec_cycle(&mut self) {
        self.simulator = self.simulator.reset();
        self.cycle -= 1;
        for _ in 0..self.cycle {
            self.simulator.rasing_edge();
            self.simulator.falling_edge();
        }
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
