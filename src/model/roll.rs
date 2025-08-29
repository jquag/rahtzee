#[derive(Clone, Copy)]
pub enum Roll {
    Ones { score: Option<u32> },
    Twos { score: Option<u32> },
    Threes { score: Option<u32> },
    Fours { score: Option<u32> },
    Fives { score: Option<u32> },
    Sixes { score: Option<u32> },
    ThreeOfAKind { score: Option<u32> },
    FourOfAKind { score: Option<u32> },
    FullHouse { score: Option<u32> },
    SmallStraight { score: Option<u32> },
    LargeStraight { score: Option<u32> },
    Chance { score: Option<u32> },
    Yahtzee { score: Option<u32> },
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
            ones_roll: Roll::Ones { score: None },
            twos_roll: Roll::Twos { score: None },
            threes_roll: Roll::Threes { score: None },
            fours_roll: Roll::Fours { score: None },
            fives_roll: Roll::Fives { score: None },
            sixes_roll: Roll::Sixes { score: None },
            three_of_a_kind_roll: Roll::ThreeOfAKind { score: None },
            four_of_a_kind_roll: Roll::FourOfAKind { score: None },
            full_house_roll: Roll::FullHouse { score: None },
            small_straight_roll: Roll::SmallStraight { score: None },
            large_straight_roll: Roll::LargeStraight { score: None },
            chance_roll: Roll::Chance { score: None },
            yahtzee_roll: Roll::Yahtzee { score: None },
        }
    }

    pub fn bonus_status(self) -> (u32, u32) {
        let progress = self.ones_roll.score().unwrap_or(0)
            + self.twos_roll.score().unwrap_or(0)
            + self.threes_roll.score().unwrap_or(0)
            + self.fours_roll.score().unwrap_or(0)
            + self.fives_roll.score().unwrap_or(0)
            + self.sixes_roll.score().unwrap_or(0);

        (progress, if progress >= 63 { 35 } else { 0 })
    }
}

impl Roll {
    pub fn score(&self) -> Option<u32> {
        match self {
            Roll::Ones { score } |
            Roll::Twos { score } |
            Roll::Threes { score } |
            Roll::Fours { score } |
            Roll::Fives { score } |
            Roll::Sixes { score } |
            Roll::ThreeOfAKind { score } |
            Roll::FourOfAKind { score } |
            Roll::FullHouse { score } |
            Roll::SmallStraight { score } |
            Roll::LargeStraight { score } |
            Roll::Chance { score } |
            Roll::Yahtzee { score } => *score,
        }
    }
}
