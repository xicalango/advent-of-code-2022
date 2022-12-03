use std::cmp::Ordering;
use std::str::FromStr;

use crate::{Error, Scored};

#[derive(Debug, Eq, PartialEq)]
pub enum OpponentChoice {
    A,
    B,
    C,
}

#[derive(Debug, Eq, PartialEq)]
pub enum MyChoice {
    X,
    Y,
    Z,
}

#[derive(Debug, Eq, PartialEq, Ord, Copy, Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        Some(match *self {
            RPS::Rock => if *other == RPS::Scissors { Ordering::Greater } else { Ordering::Less },
            RPS::Paper => if *other == RPS::Rock { Ordering::Greater } else { Ordering::Less },
            RPS::Scissors => if *other == RPS::Paper { Ordering::Greater } else { Ordering::Less },
        })
    }
}

impl From<&OpponentChoice> for RPS {
    fn from(choice: &OpponentChoice) -> Self {
        match *choice {
            OpponentChoice::A => RPS::Rock,
            OpponentChoice::B => RPS::Paper,
            OpponentChoice::C => RPS::Scissors,
        }
    }
}

impl FromStr for OpponentChoice {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(OpponentChoice::A),
            "B" => Ok(OpponentChoice::B),
            "C" => Ok(OpponentChoice::C),
            _ => Err(Error(format!("invalid choice: {}", s)))
        }
    }
}

impl FromStr for MyChoice {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MyChoice::X),
            "Y" => Ok(MyChoice::Y),
            "Z" => Ok(MyChoice::Z),
            _ => Err(Error(format!("invalid choice: {}", s)))
        }
    }
}

impl RPS {
    pub fn get_draw_choice(&self) -> RPS {
        *self
    }

    pub fn get_losing_choice(&self) -> RPS {
        match *self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    pub fn get_winning_choice(&self) -> RPS {
        match *self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

impl Scored for RPS {
    fn get_score(&self) -> u32 {
        match *self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

#[derive(Debug)]
pub struct StrategyGuide(Vec<(OpponentChoice, MyChoice)>);

#[derive(Debug)]
pub struct Game(Vec<(RPS, RPS)>);

impl FromStr for StrategyGuide {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut result = Vec::new();

        for line in s.lines() {
            let (opponent_choice, my_choice) = line.trim_end().split_once(" ").ok_or(Error("invalid line".to_string()))?;

            let opponent_choice: OpponentChoice = opponent_choice.parse()?;
            let my_choice: MyChoice = my_choice.parse()?;

            result.push( (opponent_choice, my_choice) );
        }

        Ok(StrategyGuide(result))
    }
}

impl Scored for (RPS, RPS) {
    fn get_score(&self) -> u32 {
        let (opponent, my) = self;

        let winner_score = if my == opponent {
            3
        } else if my > opponent {
            6
        } else {
            0
        };

        my.get_score() + winner_score
    }
}

pub trait Evaluator {
    fn evaluate_strategy(strategy: &(OpponentChoice, MyChoice)) -> (RPS, RPS);
}

pub struct BasicEvaluator;
pub struct AdvancedEvaluator;

impl Evaluator for BasicEvaluator {
    fn evaluate_strategy(strategy: &(OpponentChoice, MyChoice)) -> (RPS, RPS) {
        let (op_choice, my_choice) = strategy;
        let op_choice: RPS = op_choice.into();
        let my_choice: RPS = match *my_choice {
            MyChoice::X => RPS::Rock,
            MyChoice::Y => RPS::Paper,
            MyChoice::Z => RPS::Scissors,
        };

        (op_choice, my_choice)
    }
}

impl Evaluator for AdvancedEvaluator {
    fn evaluate_strategy(strategy: &(OpponentChoice, MyChoice)) -> (RPS, RPS) {
        let (op_choice, my_choice) = strategy;
        let op_choice: RPS = op_choice.into();
        let my_choice = match *my_choice {
            MyChoice::X => op_choice.get_losing_choice(),
            MyChoice::Y => op_choice.get_draw_choice(),
            MyChoice::Z => op_choice.get_winning_choice(),
        };

        (op_choice, my_choice)
    }
}


impl StrategyGuide {
    pub fn evaluate_game<E: Evaluator>(&self) -> Game {
        let StrategyGuide(guide) = &self;
        let game_data = guide.iter().map(|v| E::evaluate_strategy(v)).collect();
        Game(game_data)
    }
}

impl Scored for Game {
    fn get_score(&self) -> u32 {
        let Game(scores) = &self;
        scores.iter().map(|v| v.get_score()).sum::<u32>()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_conversions() {
        let oc: OpponentChoice = "A".parse().unwrap();
        let rps: RPS = (&oc).into();

        assert_eq!(RPS::Rock, rps);
    }

    #[test]
    fn test_invalid_conversion() {
        let result : Result<MyChoice, Error> = "i".parse();
        assert!(result.is_err());
        assert_eq!("invalid choice: i", result.unwrap_err().0)
    }


    #[test]
    fn test_read_guide() {
        let guide_source = include_str!("../res/day2-guide_example.txt");
        let guide: StrategyGuide = guide_source.parse().unwrap();
        println!("{:#?}", guide);
    }

    #[test]
    fn test_basic_total_score() {
        let guide_source = include_str!("../res/day2-guide_example.txt");
        let guide: StrategyGuide = guide_source.parse().unwrap();
        let game = guide.evaluate_game::<BasicEvaluator>();

        println!("basic game: {:#?}", game);

        let total_score = game.get_score();

        println!("total score: {}", total_score);
    }

    #[test]
    fn test_advanced_total_score() {
        let guide_source = include_str!("../res/day2-guide_example.txt");
        let guide: StrategyGuide = guide_source.parse().unwrap();
        let game = guide.evaluate_game::<AdvancedEvaluator>();

        println!("advanced game: {:#?}", game);

        let total_score = game.get_score();

        println!("total score: {}", total_score);
    }

}
