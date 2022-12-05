

use std::str::FromStr;

use crate::Error;

#[derive(Debug, Default)]
pub struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();

        match split[..] {
            ["move", c, "from", f, "to", t] => Ok(Instruction { count: c.parse().unwrap(), from: f.parse().unwrap(), to: t.parse().unwrap() }),
            _ => Err(Error(format!("invalid line: {}", s))),
        }
    }
}

#[derive(Debug)]
pub struct AllInstructions(Vec<Instruction>);

impl AllInstructions {

    pub fn eval<const N: usize, L: EvalLogic>(&self, stacks: &mut Stacks<N>) {
        let AllInstructions(instructions) = self;
        for instruction in instructions {
            stacks.eval::<L>(instruction);
        }
    }

}

impl FromStr for AllInstructions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut result = Vec::new();

        let mut lines = s.lines();
        for l in &mut lines {
            if l.is_empty() {
                break;
            }
        }

        for line in lines {
            let instruction = line.trim_end().parse()?;
            result.push(instruction);
        }

        Ok(AllInstructions(result))
    }
}

pub trait EvalLogic {
    fn eval<const N: usize>(stacks: &mut Stacks<N>, instruction: &Instruction);
}

pub struct CrateMover9000;
pub struct CrateMover9001;

impl EvalLogic for CrateMover9000 {
    fn eval<const N: usize>(stacks: &mut Stacks<N>, instruction: &Instruction) {
        let Stacks(stacks) = stacks;
        for _ in 0..instruction.count {
            let element = stacks[instruction.from-1].pop().unwrap();
            stacks[instruction.to-1].push(element);
        }
    }
}

impl EvalLogic for CrateMover9001 {
    fn eval<const N: usize>(stacks: &mut Stacks<N>, instruction: &Instruction) {
        let Stacks(stacks) = stacks;
        let mut tmp_stack = Vec::new();
        for _ in 0..instruction.count {
            let element = stacks[instruction.from-1].pop().unwrap();
            tmp_stack.push(element)
        }

        for element in tmp_stack.iter().rev() {
            stacks[instruction.to-1].push(*element);
        }
    }
}

#[derive(Debug)]
pub struct Stacks<const N: usize>([Vec<char>; N]);

impl<const N: usize> Stacks<N> {

    /// converts an stack index to a position in a string
    /// position: 0123456789A
    /// line:     [A] [B] [C]
    /// index:     0   1   2
    fn index_to_str_pos(i: usize) -> usize {
        (i * 4) + 1
    }

    pub fn top_stacks(&self) -> Vec<&char> {
        let Stacks(stacks) = self;
        stacks.iter().map(|v| &v[v.len()-1]).collect()
    }

    pub fn top_stacks_str(&self) -> String {
        self.top_stacks().iter().map(|c| *c).collect()
    }

    pub fn eval<L: EvalLogic>(&mut self, instruction: &Instruction) {
        L::eval(self, instruction);
    }
}

impl<const N: usize> FromStr for Stacks<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const INIT: Vec<char> = Vec::new();
        let mut result: [Vec<char>; N] = [INIT; N];

        for line in s.lines() {
            if line.chars().nth(Self::index_to_str_pos(0)) == Some('1') {
                // TODO: hack
                break;
            }
            for i in 0..N {
                let pos = Self::index_to_str_pos(i);

                let c = line.chars().nth(pos).unwrap_or(' ');
                if c != ' ' {
                    result[i].push(c);
                }
            }
        }

        for stack in &mut result {
            stack.reverse();
        }

        Ok(Stacks(result))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    static INPUT_TEXT: &'static str = include_str!("../res/day5-stacks_example.txt");

    #[test]
    fn test_parse() {
        let stacks: Stacks<3> = INPUT_TEXT.parse().unwrap();

        println!("{:#?}", stacks);
    }

    #[test]
    fn test_parse_instructions() {
        let instructions: AllInstructions = INPUT_TEXT.parse().unwrap();

        println!("{:?}", instructions);
    }

    #[test]
    fn test_eval() {
        let mut stacks: Stacks<3> = INPUT_TEXT.parse().unwrap();
        let instructions: AllInstructions = INPUT_TEXT.parse().unwrap();

        for instr in &instructions.0 {
            stacks.eval::<CrateMover9000>(instr);
        }

        let top_stacks = stacks.top_stacks_str();
        println!("{}", top_stacks);
    }

    #[test]
    fn test_eval_9001() {
        let mut stacks: Stacks<3> = INPUT_TEXT.parse().unwrap();
        let instructions: AllInstructions = INPUT_TEXT.parse().unwrap();

        instructions.eval::<3, CrateMover9001>(&mut stacks);

        let top_stacks = stacks.top_stacks_str();
        println!("{}", top_stacks);
    }
}
