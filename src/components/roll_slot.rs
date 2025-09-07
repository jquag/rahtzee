use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::{
    app::DieFace,
    model::roll::{Roll, RollType},
    score_util::{calc_score, is_yahtzee},
    theme::Theme,
};

pub struct RollSlot<'a> {
    pub roll: Roll,
    pub faces: &'a Vec<DieFace>,
    pub roll_count: u8,
}

impl RollSlot<'_> {
    pub fn new(roll: Roll, faces: &Vec<DieFace>, roll_count: u8) -> RollSlot<'_> {
        RollSlot {
            roll,
            faces,
            roll_count,
        }
    }
}

impl Widget for RollSlot<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let label = match self.roll.roll_type {
            RollType::Ones => num_cat_block(String::from("1")),
            RollType::Twos => num_cat_block(String::from("2")),
            RollType::Threes => num_cat_block(String::from("3")),
            RollType::Fours => num_cat_block(String::from("4")),
            RollType::Fives => num_cat_block(String::from("5")),
            RollType::Sixes => num_cat_block(String::from("6")),
            RollType::ThreeOfAKind => cat_block(String::from("3/"), String::from("Kind")),
            RollType::FourOfAKind => cat_block(String::from("4/"), String::from("Kind")),
            RollType::FullHouse => cat_block(String::from("Full"), String::from("House")),
            RollType::SmallStraight => {
                cat_block(String::from("Small"), String::from("Str8"))
            }
            RollType::LargeStraight => {
                cat_block(String::from("Large"), String::from("Str8"))
            }
            RollType::Chance => cat_block(String::from(""), String::from("Chance")),
            RollType::Yahtzee => yahtzee_block(self.roll),
        };

        let label_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        if self.roll.selected {
            let possible_score = calc_score(self.roll, self.faces);

            label.fg(Theme::ACCENT).render(label_area[0], buf);
            Line::from(possible_score.to_string())
                .centered()
                .fg(Theme::ACCENT)
                .render(label_area[1], buf);
        } else if self.roll.roll_type == RollType::Yahtzee
            && self.roll.score.unwrap_or(0) >= 50
            && self.roll_count > 0
            && !self.faces.iter().any(|face| face.is_rolling())
            && is_yahtzee(self.faces)
        {
            let possible_score = calc_score(self.roll, self.faces);

            label.fg(Theme::SECONDARY).render(label_area[0], buf);
            Line::from(possible_score.to_string())
                .centered()
                .fg(Theme::SECONDARY)
                .render(label_area[1], buf);
        } else {
            label.fg(Theme::TEXT).render(label_area[0], buf);
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
        let label = cat_block(
            String::from("Bonus"),
            format!("{}/63", self.progress),
        );
        let label_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        label.fg(Theme::TEXT).render(label_area[0], buf);
        score(Some(self.score)).render(label_area[1], buf);
    }
}

fn num_cat_block(n: String) -> Paragraph<'static> {
    let block = Block::bordered()
        .padding(Padding::horizontal(2))
        .border_set(border::ROUNDED);
    return Paragraph::new(n).block(block);
}

fn cat_block(line_1: String, line_2: String) -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from(line_1),
        Line::from(line_2),
        Line::from("───────"),
    ])
    .centered()
}

fn yahtzee_block(roll: Roll) -> Paragraph<'static> {
    match yahtzee_bonus_count_from_score(roll.score) {
        0 => cat_block(String::from(""), String::from("Yahtzee")),
        c if c <= 6 => cat_block(
            String::from("Yahtzee"),
            '★'.to_string().repeat(c.into()),
        ),
        _ => cat_block(
            String::from("Yahtzee"),
            String::from("★★★★★★+"),
        ),
    }
}

fn score(score: Option<u32>) -> Line<'static> {
    return match score {
        Some(s) => Line::from(s.to_string()).centered().fg(Theme::PRIMARY),
        None => Line::from(String::from("---"))
            .centered()
            .fg(Theme::TEXT_DIM),
    };
}

fn yahtzee_bonus_count_from_score(score: Option<u32>) -> u8 {
    match score {
        Some(s) if s >= 50 => ((s - 50) / 100) as u8,
        _ => 0,
    }
}
