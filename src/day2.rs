use std::cmp::Ordering;
use std::str::FromStr;

use crate::Error;

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

#[derive(Debug, Eq, PartialEq, Ord)]
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

impl From<OpponentChoice> for RPS {
    fn from(choice: OpponentChoice) -> Self {
        match choice {
            OpponentChoice::A => RPS::Rock,
            OpponentChoice::B => RPS::Paper,
            OpponentChoice::C => RPS::Scissors,
        }
    }
}

impl From<MyChoice> for RPS {
    fn from(choice: MyChoice) -> Self {
        match choice {
            MyChoice::X => RPS::Rock,
            MyChoice::Y => RPS::Paper,
            MyChoice::Z => RPS::Scissors,
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

pub trait Scored {
    fn get_score(&self) -> u32;
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
pub struct StrategyGuide(Vec<(RPS, RPS)>);

impl FromStr for StrategyGuide {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut result = Vec::new();

        for line in s.lines() {
            let (opponent_choice, my_choice) = line.trim_end().split_once(" ").ok_or(Error("invalid line".to_string()))?;

            let opponent_choice: OpponentChoice = opponent_choice.parse()?;
            let my_choice: MyChoice = my_choice.parse()?;

            result.push( (opponent_choice.into(), my_choice.into()) );
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

impl Scored for StrategyGuide {
    fn get_score(&self) -> u32 {
        let StrategyGuide(scores) = &self;
        scores.iter().map(|v| v.get_score()).sum::<u32>()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_conversions() {
        let oc: OpponentChoice = "A".parse().unwrap();
        let rps: RPS = oc.into();

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
    fn test_total_score() {
        let guide_source = include_str!("../res/day2-guide_example.txt");
        let guide: StrategyGuide = guide_source.parse().unwrap();

        let total_score = guide.get_score();

        println!("total score: {}", total_score);
    }

}
