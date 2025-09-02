#[derive(Clone, Copy, PartialEq)]
pub enum RollType {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    ThreeOfAKind,
    FourOfAKind,
    FullHouse,
    SmallStraight,
    LargeStraight,
    Chance,
    Yahtzee,
}

#[derive(Clone, Copy)]
pub struct Roll {
    pub roll_type: RollType,
    pub score: Option<u32>,
    pub selected: bool,
}

impl Roll {
    pub fn new(roll_type: RollType) -> Roll {
        Roll {
            roll_type,
            score: None,
            selected: false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct AllRolls {
    pub ones_roll: Roll,
    pub twos_roll: Roll,
    pub threes_roll: Roll,
    pub fours_roll: Roll,
    pub fives_roll: Roll,
    pub sixes_roll: Roll,
    pub three_of_a_kind_roll: Roll,
    pub four_of_a_kind_roll: Roll,
    pub full_house_roll: Roll,
    pub small_straight_roll: Roll,
    pub large_straight_roll: Roll,
    pub chance_roll: Roll,
    pub yahtzee_roll: Roll,
}

impl AllRolls {
    pub fn new() -> AllRolls {
        AllRolls {
            ones_roll: Roll::new(RollType::Ones),
            twos_roll: Roll::new(RollType::Twos),
            threes_roll: Roll::new(RollType::Threes),
            fours_roll: Roll::new(RollType::Fours),
            fives_roll: Roll::new(RollType::Fives),
            sixes_roll: Roll::new(RollType::Sixes),
            three_of_a_kind_roll: Roll::new(RollType::ThreeOfAKind),
            four_of_a_kind_roll: Roll::new(RollType::FourOfAKind),
            full_house_roll: Roll::new(RollType::FullHouse),
            small_straight_roll: Roll::new(RollType::SmallStraight),
            large_straight_roll: Roll::new(RollType::LargeStraight),
            chance_roll: Roll::new(RollType::Chance),
            yahtzee_roll: Roll::new(RollType::Yahtzee),
        }
    }

    pub fn bonus_status(self) -> (u32, u32) {
        let progress = self.ones_roll.score.unwrap_or(0)
            + self.twos_roll.score.unwrap_or(0)
            + self.threes_roll.score.unwrap_or(0)
            + self.fours_roll.score.unwrap_or(0)
            + self.fives_roll.score.unwrap_or(0)
            + self.sixes_roll.score.unwrap_or(0);

        (progress, if progress >= 63 { 35 } else { 0 })
    }

    fn iter(&self) -> impl Iterator<Item = &Roll> {
        [
            &self.ones_roll,
            &self.twos_roll,
            &self.threes_roll,
            &self.fours_roll,
            &self.fives_roll,
            &self.sixes_roll,
            &self.three_of_a_kind_roll,
            &self.four_of_a_kind_roll,
            &self.full_house_roll,
            &self.small_straight_roll,
            &self.large_straight_roll,
            &self.chance_roll,
            &self.yahtzee_roll,
        ]
        .into_iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Roll> {
        [
            &mut self.ones_roll,
            &mut self.twos_roll,
            &mut self.threes_roll,
            &mut self.fours_roll,
            &mut self.fives_roll,
            &mut self.sixes_roll,
            &mut self.three_of_a_kind_roll,
            &mut self.four_of_a_kind_roll,
            &mut self.full_house_roll,
            &mut self.small_straight_roll,
            &mut self.large_straight_roll,
            &mut self.chance_roll,
            &mut self.yahtzee_roll,
        ]
        .into_iter()
    }

    pub fn selected(&mut self) -> Option<&Roll> {
        self.iter().find(|r| r.selected)
    }

    pub fn select_next(&mut self) {
        let mut rolls: Vec<&mut Roll> = self.iter_mut().collect();
        
        // Find current selection index
        let current_idx = rolls.iter().position(|r| r.selected);
        
        // Start from next position, or 0 if nothing selected
        let start_idx = current_idx.map_or(0, |i| (i + 1) % rolls.len());
        
        for i in 0..rolls.len() {
            rolls[i].selected = false;
        }

        for i in 0..rolls.len() {
            let idx = (start_idx + i) % rolls.len();
            if rolls[idx].score.is_none() {
                rolls[idx].selected = true;
                break;
            }
        }
    }

    pub fn select_prev(&mut self) {
        let mut rolls: Vec<&mut Roll> = self.iter_mut().collect();
        
        // Find current selection index
        let current_idx = rolls.iter().position(|r| r.selected);
        
        // Start from prev position, or 0 if nothing selected
        let start_idx = match current_idx {
            Some(0) => rolls.len() - 1,
            Some(i) => i - 1,
            None => 0
        };
        
        for i in 0..rolls.len() {
            rolls[i].selected = false;
        }

        for i in 0..rolls.len() {
            let idx = if i <= start_idx {
                start_idx - i
            } else {
                rolls.len() - (i - start_idx)
            };
            if rolls[idx].score.is_none() {
                rolls[idx].selected = true;
                break;
            }
        }
    }
}
