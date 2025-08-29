use ratatui::{
    buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, widgets::Widget
};

use crate::{components::roll_slot::{BonusSlot, RollSlot}, model::roll::AllRolls};

pub struct RollSlots {
    pub rolls: AllRolls
}

impl Widget for RollSlots {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // First row - 5 chars tall
                Constraint::Length(4), // Second row - 5 chars tall
            ])
            .spacing(1)
            .split(area);

        // Create 7 columns for each row (7 chars wide each)
        let constraints: Vec<Constraint> = (0..7).map(|_| Constraint::Length(7)).collect();

        let top_row_slots = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints.clone())
            .spacing(1)
            .split(rows[0]);

        let bottom_row_slots = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .spacing(1) 
            .split(rows[1]);

        let one_slot = RollSlot::new(self.rolls.ones_roll);
        one_slot.render(top_row_slots[0], buf);

        let two_slot = RollSlot::new(self.rolls.twos_roll);
        two_slot.render(top_row_slots[1], buf);

        let three_slot = RollSlot::new(self.rolls.threes_roll);
        three_slot.render(top_row_slots[2], buf);

        let four_slot = RollSlot::new(self.rolls.fours_roll);
        four_slot.render(top_row_slots[3], buf);

        let five_slot = RollSlot::new(self.rolls.fives_roll);
        five_slot.render(top_row_slots[4], buf);

        let six_slot = RollSlot::new(self.rolls.sixes_roll);
        six_slot.render(top_row_slots[5], buf);

        let bonus_slot = BonusSlot::new(self.rolls.bonus_status());
        bonus_slot.render(top_row_slots[6], buf);

        let three_of_a_kind_slot = RollSlot::new(self.rolls.three_of_a_kind_roll);
        three_of_a_kind_slot.render(bottom_row_slots[0], buf);

        let four_of_a_kind_slot = RollSlot::new(self.rolls.four_of_a_kind_roll);
        four_of_a_kind_slot.render(bottom_row_slots[1], buf);

        let full_house_slot = RollSlot::new(self.rolls.full_house_roll);
        full_house_slot.render(bottom_row_slots[2], buf);

        let small_straight_slot = RollSlot::new(self.rolls.small_straight_roll);
        small_straight_slot.render(bottom_row_slots[3], buf);

        let large_straight_slot = RollSlot::new(self.rolls.large_straight_roll);
        large_straight_slot.render(bottom_row_slots[4], buf);

        let chance_slot = RollSlot::new(self.rolls.chance_roll);
        chance_slot.render(bottom_row_slots[5], buf);

        let yahtzee_slot = RollSlot::new(self.rolls.yahtzee_roll);
        yahtzee_slot.render(bottom_row_slots[6], buf);
    }
}
