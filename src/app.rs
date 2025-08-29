use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

use crate::{components::dice::Dice, model::roll::Roll};
use crate::{components::roll_slots::RollSlots, event, model::roll::AllRolls};

pub struct App {
    pub score: u32,
    pub exit: bool,
    pub rolls: AllRolls,
    pub current_roll_selection: Option<Roll>,
}

impl App {
    pub fn new() -> App {
        App {
            score: 0,
            exit: false,
            rolls: AllRolls::new(),
            current_roll_selection: None,
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

    fn render_main(&self, area: Rect, buf: &mut Buffer) {
        // Set max width and height
        let max_width = 58;
        let max_height = 21;

        // Calculate the constrained area (centered)
        let width = area.width.min(max_width);
        let height = area.height.min(max_height);
        let x = (area.width.saturating_sub(width)) / 2;
        let y = (area.height.saturating_sub(height)) / 2;
        let constrained_area = Rect::new(x, y, width, height);

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(21), Constraint::Length(1)])
            .split(constrained_area);

        let title = Line::from(" YAHTZEE ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .fg(Color::Indexed(50))
            .border_set(border::THICK);

        // Create inner area for content (inside the border)
        let inner = block.inner(constrained_area);

        // First render the main block
        block.render(main_layout[0], buf);

        // Split inner into top (5 rows) and bottom (rest)
        let sections = Layout::default()
            .direction(Direction::Vertical)
            .spacing(1)
            .constraints([
                Constraint::Length(9),
                Constraint::Length(6),
                Constraint::Length(1),
            ])
            .split(inner);

        let roll_slots = RollSlots { rolls: self.rolls };
        roll_slots.render(sections[0], buf);

        let dice = Dice::new(vec![1, 2, 3, 4, 5, 6]);
        dice.render(sections[1], buf);

        Line::from(vec![
            "SCORE: ".white(),
            format!("{} ", self.score).fg(Color::Indexed(147)).bold(),
        ]).right_aligned().render(sections[2], buf);

        let instructions = Line::from(vec![
            "Quit ".white(),
            "q ".blue().bold(),
            "| Roll ".white(),
            "r ".blue().bold(),
            "| (Un)Hold ".white(),
            "1-6 ".blue().bold(),
            "| Move ".white(),
            "arrows ".blue().bold(),
            "| Select ".white(),
            "CR".blue().bold(),
        ])
        .centered();
        instructions.render(main_layout[1], buf);
    }
}

pub fn draw(app: &App, frame: &mut Frame) {
    frame.render_widget(app, frame.area());
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 58 || area.height < 21 {
            Line::from("Terminal window too small".red().bold()).render(area, buf);
        } else {
            self.render_main(area, buf);
        }
    }
}
