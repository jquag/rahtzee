use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::{model::roll::Roll, theme::Theme};

pub struct RollSlot {
    pub roll: Roll,
}

impl RollSlot {
    pub fn new(roll: Roll) -> RollSlot {
        RollSlot { roll }
    }
}

impl Widget for RollSlot {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let label = match self.roll {
            Roll::Ones { .. } => num_cat_block(String::from("1")),
            Roll::Twos { .. } => num_cat_block(String::from("2")),
            Roll::Threes { .. } => num_cat_block(String::from("3")),
            Roll::Fours { .. } => num_cat_block(String::from("4")),
            Roll::Fives { .. } => num_cat_block(String::from("5")),
            Roll::Sixes { .. } => num_cat_block(String::from("6")),
            Roll::ThreeOfAKind { .. } => cat_block(String::from("3/"), String::from("Kind")),
            Roll::FourOfAKind { .. } => cat_block(String::from("4/"), String::from("Kind")),
            Roll::FullHouse { .. } => cat_block(String::from("Full"), String::from("House")),
            Roll::SmallStraight { .. } => cat_block(String::from("Small"), String::from("Str8")),
            Roll::LargeStraight { .. } => cat_block(String::from("Large"), String::from("Str8")),
            Roll::Chance { .. } => cat_block(String::from(""), String::from("Chance")),
            Roll::Yahtzee { .. } => cat_block(String::from(""), String::from("Yahtzee")),
        };

        let label_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        label.render(label_area[0], buf);
        score(self.roll.score()).render(label_area[1], buf);
    }
}

pub struct BonusSlot {
    pub progress: u32,
    pub score: u32,
}

impl BonusSlot {
    pub fn new(status: (u32, u32)) -> BonusSlot {
        BonusSlot { progress: status.0, score: status.1 }
    }
}

impl Widget for BonusSlot {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let label = cat_block(String::from("Bonus"), format!("{}/63", self.progress));
        let label_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        label.render(label_area[0], buf);
        score(Some(self.score)).render(label_area[1], buf);
    }
}

fn num_cat_block(n: String) -> Paragraph<'static> {
    let block = Block::bordered()
        .fg(Theme::TEXT)
        .padding(Padding::horizontal(2))
        .border_set(border::ROUNDED);
    return Paragraph::new(n).block(block);
}

fn cat_block(line_1: String, line_2: String) -> Paragraph<'static> {
    Paragraph::new(vec![Line::from(line_1), Line::from(line_2), Line::from("───────")])
        .centered()
        .fg(Theme::TEXT)
}

fn score(score: Option<u32>) -> Line<'static> {
    return match score {
        Some(s) => Line::from(s.to_string()).centered().fg(Theme::PRIMARY),
        None => Line::from(String::from("---")).centered().fg(Theme::TEXT_DIM),
    };
}
