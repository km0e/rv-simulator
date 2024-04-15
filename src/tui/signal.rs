use std::{cell::RefCell, rc::Rc};

use ratatui::{
    layout::Layout,
    widgets::{List, Widget},
};

pub struct SignalList {
    pub signals: Rc<RefCell<Vec<(String, u32)>>>,
}

impl Widget for SignalList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let signals = self.signals.borrow();
        let (name, value): (Vec<_>, Vec<_>) = signals
            .iter()
            .map(|(name, value)| (name.clone(), format!("{:#x}", value)))
            .unzip();
        let chunk = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(
                [
                    ratatui::layout::Constraint::Length(3),
                    ratatui::layout::Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(area);
        List::new(name.into_iter().collect::<Vec<_>>()).render(chunk[0], buf);
        List::new(value.into_iter().collect::<Vec<_>>()).render(chunk[1], buf);
    }
}
