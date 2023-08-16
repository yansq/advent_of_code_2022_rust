use std::result::Result;
use std::str::FromStr;

use crate::utils::read_lines;

trait Score {
    fn get_score(&self) -> u32;
}

trait Beats {
    fn beats(&self) -> Self;
}

enum HandResult {
    Win,
    Draw,
    Lose,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

impl Score for HandResult {
    fn get_score(&self) -> u32 {
        match *self {
            HandResult::Win => 6,
            HandResult::Draw => 3,
            HandResult::Lose => 0,
        }
    }
}

impl Score for Hand {
    fn get_score(&self) -> u32 {
        match *self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

fn play_hand(own_hand: Hand, opponent_hand: Hand) -> HandResult {
    let (own_beats, opponent_beats) = (own_hand.beats(), opponent_hand.beats());
    match (own_beats, opponent_beats) {
        _ if own_beats == opponent_hand => HandResult::Win,
        _ if opponent_beats == own_hand => HandResult::Lose,
        _ => HandResult::Draw,
    }
}

pub fn calculate_score() -> u32 {
    let mut score = 0;
    if let Ok(lines) = read_lines("src/day2/input.txt") {
        for line in lines.into_iter().flatten() {
            let (opponent_hand_, own_hand_) = line.split_once(' ').unwrap();
            let (own_hand, opponent_hand) = (
                Hand::from_str(own_hand_).unwrap(),
                Hand::from_str(opponent_hand_).unwrap(),
            );
            score += own_hand.get_score();
            let result = play_hand(own_hand, opponent_hand);
            score += result.get_score();
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result() {
        assert_eq!(calculate_score(), 13052);
    }
}
