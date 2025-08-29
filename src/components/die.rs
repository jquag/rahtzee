use ratatui::{
    buffer::Buffer, layout::Rect, style::Stylize, symbols::border, text::Line, widgets::{Block, Paragraph, Widget}
};

pub struct Die {
    pub face: u8,
}

impl Die {
    pub fn new(face: u8) -> Self {
        Self {
            face: face.clamp(1, 6),
        }
    }
}

impl Widget for Die {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Die faces using Unicode characters
        let die_face = match self.face {
            1 => vec!["       ", "   ●   ", "       "],
            2 => vec![" ●     ", "       ", "     ● "],
            3 => vec![" ●     ", "   ●   ", "     ● "],
            4 => vec![" ●   ● ", "       ", " ●   ● "],
            5 => vec![" ●   ● ", "   ●   ", " ●   ● "],
            6 => vec![" ●   ● ", " ●   ● ", " ●   ● "],
            _ => vec!["       ", "   ?   ", "       "],
        };

        let lines: Vec<Line> = die_face.iter().map(|&s| Line::from(s).white()).collect();

        let block = Block::bordered().border_set(border::ROUNDED).white();

        Paragraph::new(lines).block(block).render(area, buf);
    }
}
