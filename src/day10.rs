use std::str::FromStr;
use crate::Error;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {

    pub fn get_cycles(&self) -> u8 {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }

}

#[derive(Debug, Clone)]
pub struct CPU {
    cycle: u32,
    x: i32,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Measurement(i32);

impl Measurement {
    pub fn measurement(&self) -> &i32 {
        &self.0
    }
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            cycle: 0,
            x: 1,
        }
    }
}

impl CPU {
    fn execute(&mut self, instruction: &Instruction) -> Option<Measurement> {

        let mut measurement = None;

        let cycles_to_add = instruction.get_cycles();

        for _ in 0..cycles_to_add {
            self.cycle += 1;
            self.gpu_trap();
            if self.is_cycle_magic() {
                measurement = Some(self.get_signal_strength())
            }
        }

        match instruction {
            Instruction::AddX(par) => {
                self.x += par;
            },
            _ => {},
        };

        measurement
    }

    fn get_signal_strength(&self) -> Measurement {
        Measurement(self.x * self.cycle as i32)
    }

    fn is_cycle_magic(&self) -> bool {
        self.cycle >= 20 && (self.cycle - 20) % 40 == 0
    }

    fn gpu_trap(&self) {
        let beam_x = (self.cycle - 1) % 40;

        if self.x.abs_diff(beam_x as i32) <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        if self.cycle % 40 == 0 {
            println!();
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            None => if s == "noop" {
                Ok(Instruction::Noop)
            } else {
                Err(Error(format!("Invalid instruction: {}", s)))
            },
            Some((cmd, par)) => match cmd {
                "addx" => Ok(Instruction::AddX(par.parse()?)),
                _ => Err(Error(format!("Invalid instruction: {}", s)))
            }
        }
    }
}

pub fn get_signal_strength<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Measurement> {
    let mut cpu = CPU::default();
    lines.filter_map(|line| {
        cpu.execute(&line.trim_end().parse().unwrap())
    }).collect()
}

#[cfg(test)]
mod test {

    use super::*;

    static EXAMPLE1: &'static str = include_str!("../res/day10-instr_example1.txt");
    static EXAMPLE2: &'static str = include_str!("../res/day10-instr_example2.txt");

    #[test]
    fn test_parse_simple() {
        let instructions: Result<Vec<Instruction>, Error> = EXAMPLE1.lines().map(|l| l.trim_end().parse()).collect();
        let instructions = instructions.unwrap();

        println!("{:#?}", instructions);
    }

    #[test]
    fn test_signal_strength_simple() {
        println!("{:?}", get_signal_strength(EXAMPLE1.lines()));
    }

    #[test]
    fn test_signal_strength_example2() {
        let strengths = get_signal_strength(EXAMPLE2.lines());
        let unwrapped_strengths: Vec<i32> = strengths.iter().map(|m| m.0).collect();
        assert_eq!(vec![420, 1140, 1800, 2940, 2880, 3960], unwrapped_strengths);
        assert_eq!(13140, unwrapped_strengths.iter().sum());
    }

}
