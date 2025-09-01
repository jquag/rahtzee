use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::{app::DieFace, theme::Theme};

pub struct Die {
    pub face: DieFace,
}

impl Die {
    pub fn new(face: DieFace) -> Self {
        Self { face }
    }
}

impl Widget for Die {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Die faces using Unicode characters
        let die_face = match self.face.value {
            1 => vec!["       ", "   ●   ", "       "],
            2 => vec![" ●     ", "       ", "     ● "],
            3 => vec![" ●     ", "   ●   ", "     ● "],
            4 => vec![" ●   ● ", "       ", " ●   ● "],
            5 => vec![" ●   ● ", "   ●   ", " ●   ● "],
            6 => vec![" ●   ● ", " ●   ● ", " ●   ● "],
            _ => vec!["       ", "   ?   ", "       "],
        };

        let border_color = if self.face.held {
            Theme::ACCENT
        } else {
            Theme::TEXT
        };
        let color = if self.face.is_rolling() {
            Theme::TEXT_DIM
        } else if self.face.held {
            Theme::ACCENT
        } else {
            Theme::TEXT
        };
        let lines: Vec<Line> = die_face.iter().map(|&s| Line::from(s).fg(color)).collect();

        let block = Block::bordered()
            .border_set(border::ROUNDED)
            .fg(border_color);

        Paragraph::new(lines).block(block).render(area, buf);
    }
}
