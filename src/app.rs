use rand::Rng;
use std::io;
use std::time::{Duration, Instant};

use ratatui::widgets::Paragraph;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

use crate::components::dice::Dice;
use crate::score_util::calc_score;
use crate::theme::Theme;
use crate::{components::roll_slots::RollSlots, event, model::roll::AllRolls};

const HEIGHT: u8 = 18;
const WIDTH: u8 = 58;

pub struct App {
    pub exit: bool,
    pub rolls: AllRolls,
    pub dice_faces: Vec<DieFace>,
    pub roll_count: u8,
}

#[derive(Clone, Copy)]
pub struct DieFace {
    pub value: u8,
    pub held: bool,
    pub rolling_until: Option<Instant>,
}

impl DieFace {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            held: false,
            rolling_until: None,
        }
    }

    pub fn is_rolling(self) -> bool {
        self.rolling_until.is_some()
    }
}

impl App {
    pub fn new() -> App {
        App {
            exit: false,
            rolls: AllRolls::new(),
            roll_count: 0,
            dice_faces: vec![
                DieFace::new(1),
                DieFace::new(2),
                DieFace::new(3),
                DieFace::new(4),
                DieFace::new(5),
            ],
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| draw(self, frame))?;
            event::handle_events(self)?;
        }
        Ok(())
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn start_roll(&mut self) {
        if self.roll_count < 3 {
            self.rolls.clear_selection();
            self.roll_count += 1;
            for die in &mut self.dice_faces {
                if !die.held {
                    let duration = rand::rng().random_range(500..=1000);
                    die.rolling_until = Some(Instant::now() + Duration::from_millis(duration));
                }
            }
            if self.roll_count == 3 {
                self.rolls.select_next();
            }
        }
    }

    pub fn update_dice_animation(&mut self) {
        for die in &mut self.dice_faces {
            if die.rolling_until.map_or(false, |i| i > Instant::now()) {
                die.value = rand::rng().random_range(1..=6);
            } else {
                die.rolling_until = None;
            }
        }
    }

    pub fn is_rolling(&self) -> bool {
        self.dice_faces.iter().any(|face| face.is_rolling())
    }

    fn render_main(&self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(21), Constraint::Length(1)])
            .split(area);

        let title = Line::from(" YAHTZEE ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .fg(Theme::BORDER)
            .border_set(border::THICK);

        let inner = block.inner(area);
        block.render(main_layout[0], buf);
        self.render_footer(main_layout[1], buf);

        let sections = Layout::default() //slot section and bottom section with dice and score
            .direction(Direction::Vertical)
            .spacing(1)
            .constraints([Constraint::Length(9), Constraint::Length(5)])
            .split(inner);

        self.render_slots(sections[0], buf);

        if self.is_game_over() {
            self.render_game_over(sections[1], buf);
        } else {
            self.render_dice_and_score(sections[1], buf);
        }
    }

    fn render_game_over(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from("Game Over").centered())
            .fg(Theme::ACCENT)
            .border_set(border::DOUBLE);
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Create vertical centering layout
        let vertical_center = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Min(1),
            ])
            .split(inner_area);

        Paragraph::new(vec![Line::from(vec![
            "Score: ".fg(Theme::TEXT),
            format!("{}", self.total_score()).fg(Theme::PRIMARY),
        ])])
        .centered()
        .render(vertical_center[1], buf);
    }

    fn render_slots(&self, area: Rect, buf: &mut Buffer) {
        let roll_slots = RollSlots {
            rolls: self.rolls,
            faces: &self.dice_faces,
            roll_count: self.roll_count,
        };
        roll_slots.render(area, buf);
    }

    fn render_dice_and_score(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([45, 11])
            .split(area);

        let dice = Dice::new(self.dice_faces.clone());
        dice.render(layout[0], buf);

        let filled = "●";
        let empty = "○";
        Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                "Roll: ".fg(Theme::TEXT),
                if self.roll_count > 0 { filled } else { empty }
                    .fg(Theme::PRIMARY)
                    .bold(),
                if self.roll_count > 1 { filled } else { empty }
                    .fg(Theme::PRIMARY)
                    .bold(),
                if self.roll_count > 2 { filled } else { empty }
                    .fg(Theme::PRIMARY)
                    .bold(),
            ]),
            Line::from(vec![
                "SCORE: ".fg(Theme::TEXT),
                format!("{}", self.total_score()).fg(Theme::PRIMARY).bold(),
            ]),
        ])
        .right_aligned()
        .render(layout[1], buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let instructions = match self.is_game_over() {
            false => Line::from(vec![
                "Quit ".fg(Theme::TEXT),
                "q ".fg(Theme::SECONDARY).bold(),
                "| Roll ".fg(Theme::TEXT),
                "r ".blue().bold(),
                "| (Un)Hold ".fg(Theme::TEXT),
                "1-5 ".fg(Theme::SECONDARY).bold(),
                "| Move ".fg(Theme::TEXT),
                "arrows ".fg(Theme::SECONDARY).bold(),
                "| Select ".fg(Theme::TEXT),
                "CR".fg(Theme::SECONDARY).bold(),
            ]),
            true => Line::from(vec![
                "Quit ".fg(Theme::TEXT),
                "q ".blue().bold(),
                "| Play Again ".fg(Theme::TEXT),
                "CR".blue().bold(),
            ]),
        };

        instructions.centered().render(area, buf);
    }

    pub fn toggle_hold(&mut self, index: usize) {
        self.dice_faces[index].held = !self.dice_faces[index].held;
    }

    pub fn submit_selection(&mut self) {
        if self.rolls.selected().is_some() {
            if self.rolls.yahtzee_roll.score.is_some() {
                // if yahtzee already scored, always check for bonus yahtzee
                self.rolls.yahtzee_roll.score = Some(calc_score(self.rolls.yahtzee_roll, &self.dice_faces));
            }
            let selection = self.rolls.selected().unwrap();
            selection.score = Some(calc_score(*selection, &self.dice_faces).into());
            self.reset();
        }
    }

    pub fn total_score(&self) -> u32 {
        let total = self.rolls
            .iter()
            .fold(0, |tot, r| tot + r.score.unwrap_or(0));
        let (_, bonus) = self.rolls.bonus_status();
        return total + bonus;
    }

    pub fn reset(&mut self) {
        for f in &mut self.dice_faces {
            f.held = false
        }
        self.roll_count = 0;
        self.rolls.clear_selection();
    }

    pub fn start_over(&mut self) {
        self.reset();
        self.rolls = AllRolls::new();
    }

    pub fn is_game_over(&self) -> bool {
        self.rolls.iter().all(|r| r.score.is_some())
    }
}

pub fn draw(app: &App, frame: &mut Frame) {
    frame.render_widget(app, frame.area());
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < WIDTH.into() || area.height < HEIGHT.into() {
            Line::from("Terminal window too small".red().bold()).render(area, buf);
        } else {
            let width = area.width.min(WIDTH.into());
            let height = area.height.min(HEIGHT.into());
            let x = (area.width.saturating_sub(width)) / 2;
            let y = (area.height.saturating_sub(height)) / 2;
            let constrained_area = Rect::new(x, y, width, height);
            self.render_main(constrained_area, buf);
        }
    }
}
