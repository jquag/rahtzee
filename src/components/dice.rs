use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Stylize}, text::Line, widgets::Widget};

use crate::{app::DieFace, components::die::Die, theme::Theme};

pub struct Dice {
    pub faces: Vec<DieFace>,
}

impl Dice {
    pub fn new(faces: Vec<DieFace>) -> Self {
        Self {
            faces,
        }
    }
}

impl Widget for Dice {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create constraints based on number of dice
        let constraints: Vec<Constraint> = self.faces
            .iter()
            .map(|_| Constraint::Length(9))
            .collect();
        
        // Render die faces horizontally
        let dice_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            // .spacing(1)
            .split(area);
        
        for (face, area) in self.faces.iter().zip(dice_row.iter()) {
            let die = Die::new(*face);
            die.render(*area, buf);
        }
    }
}
