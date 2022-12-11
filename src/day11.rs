
use std::{str::FromStr, num::ParseIntError};

use crate::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        match &parts[..] {
            ["new", "=", "old", "*", "old"] => Ok(Operation::Square),
            ["new", "=", "old", "*", v] => Ok(Operation::Mul(v.parse()?)),
            ["new", "=", "old", "+", v] => Ok(Operation::Add(v.parse()?)),
            _ => Err(Error(format!("invalid line: {}", s)))
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonkeyMeta {
    starting_items: Vec<u64>,
    op: Operation,
    div_test: u64,
    next_monkeys: (usize, usize)
}

impl Default for MonkeyMeta {
    fn default() -> Self {
        Self { starting_items: Default::default(), op: Operation::Add(0), div_test: Default::default(), next_monkeys: Default::default() }
    }
}

#[derive(Debug)]
pub struct AllMonkeyMeta(Vec<MonkeyMeta>);

impl FromStr for AllMonkeyMeta {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut result = Vec::new();
        let mut accu = MonkeyMeta::default();

        for line in s.lines().map(|l| l.trim()) {
            if line.is_empty() {
                result.push(accu);
                accu = MonkeyMeta::default();
            }

            let vars = line.split_once(":");
            if let None = vars {
                continue;
            }

            let (op, par) = vars.unwrap();

            // println!("op: .{}. par: .{}.", op, par);

            match op {
                "Starting items" => {
                    let items: Result<Vec<u64>, ParseIntError> = par.split(",").map(|v| v.trim().parse()).collect();
                    accu.starting_items = items?;
                },
                "Operation" => {
                    accu.op = par.trim().parse()?;
                },
                "Test" => {
                    accu.div_test = par.split_whitespace().last().ok_or(Error(format!("invalid line: {}", line)))?.parse()?;
                },
                "If true" => {
                    accu.next_monkeys.0 = par.split_whitespace().last().ok_or(Error(format!("invalid line: {}", line)))?.parse()?;
                },
                "If false" => {
                    accu.next_monkeys.1 = par.split_whitespace().last().ok_or(Error(format!("invalid line: {}", line)))?.parse()?;
                },
                _ => {},
            }

        }

        Ok(AllMonkeyMeta(result))
    }
}


#[derive(Debug, Default)]
pub struct MonkeyState {
    meta: MonkeyMeta,
    items: Vec<u64>,
    inspect_counter: u64,
}

impl From<MonkeyMeta> for MonkeyState {

  

}

#[cfg(test)]
mod test {

    use super::*;

    static EXAMPLE: &'static str = include_str!("../res/day11-apes_example.txt");

    #[test]
    fn test_parse_op() {

        let op1 = "new = old * 19";
        let op2 = "new = old + 19";
        let op3 = "new = old * old";
        let op_err = "new = old + old";

        let op1: Operation = op1.parse().unwrap();
        let op2: Operation = op2.parse().unwrap();
        let op3: Operation = op3.parse().unwrap();
        let op_err: Result<Operation, Error> = op_err.parse();

        assert_eq!(op1, Operation::Mul(19));
        assert_eq!(op2, Operation::Add(19));
        assert_eq!(op3, Operation::Square);
        assert!(op_err.is_err());
    }

    #[test]
    fn test_parse_monkey_meta() {
        let allmeta: AllMonkeyMeta = EXAMPLE.parse().unwrap();
        println!("{:#?}", allmeta);
    }

}
