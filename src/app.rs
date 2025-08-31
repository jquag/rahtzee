use rand::Rng;
use std::io;
use std::time::{Duration, Instant};

use ratatui::widgets::{self, Paragraph};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

use crate::theme::Theme;
use crate::{components::dice::Dice, model::roll::Roll};
use crate::{components::roll_slots::RollSlots, event, model::roll::AllRolls};

const HEIGHT: u8 = 18;
const WIDTH: u8 = 58;

pub struct App {
    pub score: u32,
    pub exit: bool,
    pub rolls: AllRolls,
    pub current_roll_selection: Option<Roll>,
    pub dice_faces: Vec<DieFace>,
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
            score: 0,
            exit: false,
            rolls: AllRolls::new(),
            current_roll_selection: None,
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
        for die in &mut self.dice_faces {
            let duration = rand::rng().random_range(500..=1000);
            die.rolling_until = Some(Instant::now() + Duration::from_millis(duration));
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
        // Calculate the constrained area (centered)
        let width = area.width.min(WIDTH.into());
        let height = area.height.min(HEIGHT.into());
        let x = (area.width.saturating_sub(width)) / 2;
        let y = (area.height.saturating_sub(height)) / 2;
        let constrained_area = Rect::new(x, y, width, height);

        let main_layout = Layout::default() //main block and one-line footer
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(21), Constraint::Length(1)])
            .split(constrained_area);

        let title = Line::from(" YAHTZEE ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .fg(Theme::BORDER)
            .border_set(border::THICK);

        let inner = block.inner(constrained_area);
        block.render(main_layout[0], buf);
        self.render_footer(main_layout[1], buf);

        let sections = Layout::default() //slot section and bottom section with dice and score
            .direction(Direction::Vertical)
            .spacing(1)
            .constraints([
                Constraint::Length(9),
                Constraint::Length(5),
            ])
            .split(inner);

        self.render_slots(sections[0], buf);
        self.render_dice_and_score(sections[1], buf);
    }

    fn render_slots(&self, area: Rect, buf: &mut Buffer) {
        let roll_slots = RollSlots { rolls: self.rolls };
        roll_slots.render(area, buf);
    }

    fn render_dice_and_score(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([45, 11])
            .split(area);

        let dice = Dice::new(self.dice_faces.clone());
        dice.render(layout[0], buf);

        Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                "Roll: ".fg(Theme::TEXT),
                format!("{} ", 100).fg(Theme::PRIMARY).bold(),
            ]),
            Line::from(vec![
                "SCORE: ".fg(Theme::TEXT),
                format!("{} ", self.score).fg(Theme::PRIMARY).bold(),
            ]),
        ])
        .right_aligned()
        .render(layout[1], buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            "Quit ".fg(Theme::TEXT),
            "q ".blue().bold(),
            "| Roll ".fg(Theme::TEXT),
            "r ".blue().bold(),
            "| (Un)Hold ".fg(Theme::TEXT),
            "1-6 ".blue().bold(),
            "| Move ".fg(Theme::TEXT),
            "arrows ".blue().bold(),
            "| Select ".fg(Theme::TEXT),
            "CR".blue().bold(),
        ])
        .centered();
        instructions.render(area, buf);
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
            self.render_main(area, buf);
        }
    }
}
