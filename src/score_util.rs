use std::collections::HashMap;

use crate::{app::DieFace, model::roll::{Roll, RollType}};

pub fn calc_score(roll: Roll, faces: &Vec<DieFace>) -> u32 {
    match roll.roll_type {
        RollType::Ones => calc_score_for_num_type(1, faces),
        RollType::Twos => calc_score_for_num_type(2, faces),
        RollType::Threes => calc_score_for_num_type(3, faces),
        RollType::Fours => calc_score_for_num_type(4, faces),
        RollType::Fives => calc_score_for_num_type(5, faces),
        RollType::Sixes => calc_score_for_num_type(6, faces),
        RollType::ThreeOfAKind => calc_score_for_x_of_a_kind(3, faces),
        RollType::FourOfAKind => calc_score_for_x_of_a_kind(4, faces),
        RollType::FullHouse => calc_score_for_full_house(faces),
        RollType::SmallStraight => calc_score_for_straight(4, faces),
        RollType::LargeStraight => calc_score_for_straight(5, faces),
        RollType::Chance => calc_score_for_chance(faces),
        RollType::Yahtzee => calc_score_for_yahtzee(roll, faces),
    }
}

fn calc_score_for_num_type(num: u8, faces: &Vec<DieFace>) -> u32 {
    faces
        .iter()
        .filter(|f| f.value == num)
        .fold(0u32, |sum, f| sum + f.value as u32)
}

fn calc_score_for_x_of_a_kind(num: u8, faces: &Vec<DieFace>) -> u32 {
    let counts = face_counts(faces);
    let applies = counts.iter().any(|(_, count)| *count >= num);

    match applies {
        true => face_total(faces),
        false => 0,
    }
}

fn calc_score_for_full_house(faces: &Vec<DieFace>) -> u32 {
    let counts = face_counts(faces);
    if counts.iter().any(|(_, count)| *count == 2) && counts.iter().any(|(_, count)| *count == 3) {
        25
    } else {
        0
    }
}

fn calc_score_for_straight(count: u8, faces: &Vec<DieFace>) -> u32 {
    let mut sorted = faces.clone();
    sorted.sort_by(|a, b| a.value.cmp(&b.value));
    
    let mut max_run = 1;
    let mut current_run = 1;
    let mut last_value = sorted[0].value;
    
    for i in 1..sorted.len() {
        if sorted[i].value == last_value + 1 {
            current_run += 1;
            max_run = max_run.max(current_run);
        } else if sorted[i].value != last_value {
            current_run = 1;
        }
        last_value = sorted[i].value;
    }

    if max_run >= count {
        match count {
            4 => 30,
            5 => 40,
            _ => 0
        }
    } else {
        0
    }
}

fn calc_score_for_chance(faces: &Vec<DieFace>) -> u32 {
    face_total(faces)
}

fn calc_score_for_yahtzee(roll: Roll, faces: &Vec<DieFace>) -> u32 {
    if is_yahtzee(faces) {
        let current = roll.score.unwrap_or(0);
        if current > 0 {
            current + 100
        } else {
            50
        }
    } else {
        roll.score.unwrap_or(0)
    }
}

pub fn is_yahtzee(faces: &Vec<DieFace>) -> bool {
    face_counts(faces).iter().any(|(_, count)| *count == 5)
}

fn face_counts(faces: &Vec<DieFace>) -> HashMap<u8, u8> {
    faces.iter().fold(HashMap::new(), |mut map, f| {
        *map.entry(f.value).or_insert(0) += 1;
        map
    })
}

fn face_total(faces: &Vec<DieFace>) -> u32 {
    faces.iter().fold(0u32, |tot, f| tot + f.value as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_straight() {
        let dice = vec![
            DieFace::new(1),
            DieFace::new(2),
            DieFace::new(3),
            DieFace::new(4),
            DieFace::new(6),
        ];
        assert_eq!(calc_score_for_straight(4, &dice), 30);
    }

    #[test]
    fn test_small_straight_unordered() {
        let dice = vec![
            DieFace::new(3),
            DieFace::new(4),
            DieFace::new(5),
            DieFace::new(6),
            DieFace::new(1),
        ];
        assert_eq!(calc_score_for_straight(4, &dice), 30);
    }

    #[test]
    fn test_large_straight_low() {
        let dice = vec![
            DieFace::new(1),
            DieFace::new(2),
            DieFace::new(3),
            DieFace::new(4),
            DieFace::new(5),
        ];
        assert_eq!(calc_score_for_straight(5, &dice), 40);
    }

    #[test]
    fn test_large_straight_high() {
        let dice = vec![
            DieFace::new(2),
            DieFace::new(3),
            DieFace::new(4),
            DieFace::new(5),
            DieFace::new(6),
        ];
        assert_eq!(calc_score_for_straight(5, &dice), 40);
    }

    #[test]
    fn test_no_straight() {
        let dice = vec![
            DieFace::new(1),
            DieFace::new(3),
            DieFace::new(3),
            DieFace::new(4),
            DieFace::new(6),
        ];
        assert_eq!(calc_score_for_straight(4, &dice), 0);
        assert_eq!(calc_score_for_straight(5, &dice), 0);
    }

    #[test]
    fn test_straight_with_duplicates() {
        let dice = vec![
            DieFace::new(1),
            DieFace::new(2),
            DieFace::new(2),
            DieFace::new(3),
            DieFace::new(4),
        ];
        assert_eq!(calc_score_for_straight(4, &dice), 30);
    }
}
