use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Stylize}, text::Line, widgets::Widget};

use crate::components::die::Die;

pub struct Dice {
    pub dice: Vec<u8>,
}

impl Dice {
    pub fn new(dice: Vec<u8>) -> Self {
        Self {
            dice: dice,
        }
    }
}

impl Widget for Dice {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(5),
            ])
            .split(area);

        Line::from(vec![
            "Roll: ".white(),
            format!("{}", 1).fg(Color::Indexed(147)).bold(),
        ]).render(chunks[0], buf);
        
        // Create constraints based on number of dice
        let constraints: Vec<Constraint> = self.dice
            .iter()
            .map(|_| Constraint::Length(9))
            .collect();
        
        // Render die faces horizontally
        let dice_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(chunks[1]);
        
        for (face, area) in self.dice.iter().zip(dice_row.iter()) {
            let die = Die::new(*face);
            die.render(*area, buf);
        }
    }
}
