use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::{
    app::DieFace, model::roll::{Roll, RollType}, score_util::calc_score, theme::Theme
};

pub struct RollSlot<'a> {
    pub roll: Roll,
    pub faces: &'a Vec<DieFace>,
}

impl RollSlot<'_> {
    pub fn new(roll: Roll, faces: &Vec<DieFace>) -> RollSlot<'_> {
        RollSlot { roll, faces }
    }
}

impl Widget for RollSlot<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let color = if self.roll.selected {Theme::ACCENT} else {Theme::TEXT};
        let label = match self.roll.roll_type {
            RollType::Ones => num_cat_block(String::from("1"), color),
            RollType::Twos => num_cat_block(String::from("2"), color),
            RollType::Threes => num_cat_block(String::from("3"), color),
            RollType::Fours => num_cat_block(String::from("4"), color),
            RollType::Fives => num_cat_block(String::from("5"), color),
            RollType::Sixes => num_cat_block(String::from("6"), color),
            RollType::ThreeOfAKind => cat_block(String::from("3/"), String::from("Kind"), color),
            RollType::FourOfAKind => cat_block(String::from("4/"), String::from("Kind"), color),
            RollType::FullHouse => cat_block(String::from("Full"), String::from("House"), color),
            RollType::SmallStraight => cat_block(String::from("Small"), String::from("Str8"), color),
            RollType::LargeStraight => cat_block(String::from("Large"), String::from("Str8"), color),
            RollType::Chance => cat_block(String::from(""), String::from("Chance"), color),
            RollType::Yahtzee => cat_block(String::from(""), String::from("Yahtzee"), color),
        };

        let label_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        if self.roll.selected {
            let possible_score = calc_score(self.roll.roll_type, self.faces);

            label.fg(Theme::ACCENT).render(label_area[0], buf);
            Line::from(possible_score.to_string())
                .centered()
                .fg(Theme::ACCENT)
                .render(label_area[1], buf);
        } else {
            label.render(label_area[0], buf);
            score(self.roll.score).render(label_area[1], buf);
        }
    }
}

pub struct BonusSlot {
    pub progress: u32,
    pub score: u32,
}

impl BonusSlot {
    pub fn new(status: (u32, u32)) -> BonusSlot {
        BonusSlot {
            progress: status.0,
            score: status.1,
        }
    }
}

impl Widget for BonusSlot {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let label = cat_block(String::from("Bonus"), format!("{}/63", self.progress), Theme::TEXT);
        let label_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        label.render(label_area[0], buf);
        score(Some(self.score)).render(label_area[1], buf);
    }
}

fn num_cat_block(n: String, color: Color) -> Paragraph<'static> {
    let block = Block::bordered()
        .fg(color)
        .padding(Padding::horizontal(2))
        .border_set(border::ROUNDED);
    return Paragraph::new(n).block(block);
}

fn cat_block(line_1: String, line_2: String, color: Color) -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from(line_1),
        Line::from(line_2),
        Line::from("───────"),
    ])
    .centered()
    .fg(color)
}

fn score(score: Option<u32>) -> Line<'static> {
    return match score {
        Some(s) => Line::from(s.to_string()).centered().fg(Theme::PRIMARY),
        None => Line::from(String::from("---"))
            .centered()
            .fg(Theme::TEXT_DIM),
    };
}
