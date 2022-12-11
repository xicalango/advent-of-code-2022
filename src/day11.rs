
use std::{str::FromStr, num::ParseIntError};
use std::collections::VecDeque;

use crate::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {

    pub fn eval(&self, op: &u64) -> u64 {
        println!("op: {}", op);
        match self {
            Operation::Add(v) => op + v,
            Operation::Mul(v) => op * v,
            Operation::Square => op * op,
        }
    }

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

impl MonkeyMeta {

    pub fn start_eval(&self) -> MonkeyState {
        let items = self.starting_items.clone().into();
        MonkeyState {
            meta: self,
            items,
            inspect_counter: 0,
        }
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


#[derive(Debug)]
pub struct MonkeyState<'a> {
    meta: &'a MonkeyMeta,
    items: VecDeque<u64>,
    inspect_counter: u64,
}

pub trait Worry {
    fn apply_worry(old_worry: u64) -> u64;
}

pub struct ReduceWorry;
pub struct NoWorries;

impl Worry for ReduceWorry {
    fn apply_worry(old_worry: u64) -> u64 {
        old_worry / 3
    }
}

impl Worry for NoWorries {
    fn apply_worry(old_worry: u64) -> u64 {
        old_worry
    }
}

impl<'a> MonkeyState<'a> {

    pub fn eval_one_item<W: Worry>(&mut self) -> Option<(u64, &usize)> {
        let front = self.items.pop_front();

        if let None = front {
            return None;
        }

        self.inspect_counter += 1;

        let item = front.unwrap();
        let new_worry = self.meta.op.eval(&item);
        let new_worry = W::apply_worry(new_worry);

        let next_monkey = if new_worry % self.meta.div_test == 0 {
            &self.meta.next_monkeys.0
        } else {
            &self.meta.next_monkeys.1
        };

        Some((new_worry, next_monkey))
    }

}

#[derive(Debug)]
pub struct AllMonkeys<'a>(Vec<MonkeyState<'a>>);

impl AllMonkeyMeta {

    pub fn start_eval(&self) -> AllMonkeys {
        let AllMonkeyMeta(monkey_meta) = self;

        let conv: Vec<MonkeyState> = monkey_meta.iter().map(|f| f.start_eval()).collect();

        AllMonkeys(conv)
    }

}

impl<'a> AllMonkeys<'a> {

    pub fn eval_round<W: Worry>(&mut self) {
        let AllMonkeys(monkeys) = self;

        let mut add_items: Vec<Vec<u64>> = vec![Vec::default(); monkeys.len()];

        for (i, monkey) in monkeys.iter_mut().enumerate() {

            for item in add_items[i].drain(..) {
                monkey.items.push_back(item);
            }

            loop {
                // println!("eval monkey {}: {:?}", i, monkey);
                let result = monkey.eval_one_item::<W>();
                // println!("got: {:?}", result);
                match result {
                    None => break,
                    Some((worry, next)) => {
                        add_items[*next].push(worry);
                    }
                }
            }
        }

        for (i, items) in add_items.iter_mut().enumerate() {
            for item in items.drain(..) {
                monkeys[i].items.push_back(item);
            }
        }

    }

    pub fn eval_rounds<W: Worry>(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.eval_round::<W>();
        }
    }

    pub fn find_most_active<const N: usize>(&self) -> Vec<&u64> {
        let AllMonkeys(monkeys) = self;

        assert!(N <= monkeys.len());

        let mut inspections: Vec<&u64> = monkeys.iter().map(|m| &m.inspect_counter).collect();
        inspections.sort();
        inspections.reverse();

        let _ = inspections.split_off(N);

        inspections
    }

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

    #[test]
    fn test_eval() {
        let allmeta: AllMonkeyMeta = EXAMPLE.parse().unwrap();

        let mut all_monkeys: AllMonkeys = allmeta.start_eval();

        all_monkeys.eval_rounds::<ReduceWorry>(20);

        let actives = all_monkeys.find_most_active::<2>();
        println!("actives: {:?}", actives);

        let business = actives.iter().fold(1, |a, i| a * *i);
        println!("business: {}", business);

        assert_eq!(business, 10605);
    }

    #[test]
    fn test_eval_no_worry() {
        let allmeta: AllMonkeyMeta = EXAMPLE.parse().unwrap();

        let mut all_monkeys: AllMonkeys = allmeta.start_eval();

        all_monkeys.eval_rounds::<NoWorries>(10_000);

        let actives = all_monkeys.find_most_active::<2>();
        println!("actives: {:?}", actives);

        let business = actives.iter().fold(1, |a, i| a * *i);
        println!("business: {}", business);

        assert_eq!(business, 10605);
    }

}
