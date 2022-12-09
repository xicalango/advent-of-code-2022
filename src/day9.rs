use std::collections::HashSet;
use std::str::FromStr;
use crate::Error;

#[derive(Debug, Eq, PartialEq, Hash, Default, Clone)]
pub struct Vec2(i32, i32);

impl Vec2 {

    pub fn move_mut(&mut self, delta: &Vec2) {
        let Vec2(x,y) = self;
        let Vec2(dx, dy) = delta;

        *x += dx;
        *y += dy;
    }

    pub fn dist_sq(&self, other: &Vec2) -> u32 {
        let Vec2(x1,y1) = self;
        let Vec2(x2, y2) = other;

        let dx = x1 - x2;
        let dy = y1 - y2;

        (dx * dx + dy * dy) as u32
    }

    pub fn normalize(&mut self) {
        let Vec2(x,y) = self;
        *x = x.signum();
        *y = y.signum();
    }

    pub fn move_towards(&mut self, target: &Vec2) {
        let Vec2(x1,y1) = self;
        let Vec2(x2, y2) = target;

        let mut delta = Vec2(*x2-*x1, *y2-*y1);
        delta. normalize();
        self.move_mut(&delta);

    }
}


#[derive(Debug)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Dir::Right),
            "L" => Ok(Dir::Left),
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            _ => Err(Error(format!("invalid dir: {}", s))),
        }
    }
}

impl From<&Dir> for Vec2 {
    fn from(cmd: &Dir) -> Self {
        match cmd {
            Dir::Left => Vec2(-1, 0),
            Dir::Right => Vec2(1, 0),
            Dir::Up => Vec2(0, 1),
            Dir::Down => Vec2(0, -1),
        }
    }
}

#[derive(Debug)]
pub struct Command {
    dir: Dir,
    steps: u32,
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = s.split_once(" ").ok_or(Error(format!("invalid line: {}", s)))?;

        let dir: Dir = dir.parse()?;
        let steps: u32 = steps.parse()?;

        Ok(Command { dir, steps })
    }
}

pub fn apply_commands(commands: &Vec<Command>) -> usize {
    let mut head = Vec2::default();
    let mut tail = Vec2::default();
    let mut position_collector: HashSet<Vec2> = HashSet::new();

    position_collector.insert(tail.clone());

    for cmd in commands {
        let dir_vector: Vec2 = (&cmd.dir).into();

        for _ in 0..cmd.steps {
            let head_prev = head.clone();
            head.move_mut(&dir_vector);
            if head.dist_sq(&tail) > 2 {
                tail = head_prev;
                position_collector.insert(tail.clone());
            }
        }
    }

    position_collector.len()
}

pub fn apply_commands_10fold(commands: &Vec<Command>) -> usize {
    let mut head = Vec2::default();
    let mut tails: [Vec2; 9] = Default::default();
    let mut position_collector: HashSet<Vec2> = HashSet::new();

    for cmd in commands {

        let dir_vector: Vec2 = (&cmd.dir).into();

        for _ in 0..cmd.steps {

            /*

            println!("dir: {:?} step: {}/{}", cmd.dir, i+1, cmd.steps); 

            for y in 0..6 {
                for x in 0..6 {
                    let pos = Vec2(x, 5-y);
                    if head == pos {
                        print!("H");
                    } else {
                        let mut prnt = false;
                        for (i, t) in tails.iter().enumerate() {
                            if t == &pos {
                                print!("{}", i+1);
                                prnt = true;
                                break;
                            }
                        }

                        if !prnt {
                            if position_collector.contains(&pos) {
                                print!("#");
                            } else {
                                print!(".");
                            }
                        }
                    }
                }
                println!();
            }
            println!();
            println!();
            */

            head.move_mut(&dir_vector);

            let mut prev_head = &head;

            for tail in tails.iter_mut() {
                if prev_head.dist_sq(tail) > 2 {
                    tail.move_towards(prev_head);
                }
                prev_head = tail;
            }
            position_collector.insert(tails[8].clone());
        }
    }

    position_collector.len()
}

#[cfg(test)]
mod test {

    use super::*;

    static INPUT: &'static str = include_str!("../res/day9-steps_example.txt");

    #[test]
    fn test_parse() {
        let cmds : Result<Vec<Command>, Error> = INPUT.lines().map(|l| l.trim_end().parse()).collect();

        let cmds = cmds.unwrap();
        println!("{:#?}", cmds);
    }

    #[test]
    fn test_run() {
        let cmds : Result<Vec<Command>, Error> = INPUT.lines().map(|l| l.trim_end().parse()).collect();
        let cmds = cmds.unwrap();

        let count = apply_commands(&cmds);
        println!("tail visited: {}", count);
        assert_eq!(count, 13);
    }

    #[test]
    fn test_run_10() {
        let cmds : Result<Vec<Command>, Error> = INPUT.lines().map(|l| l.trim_end().parse()).collect();
        let cmds = cmds.unwrap();

        let count = apply_commands_10fold(&cmds);
        println!("tail visited: {}", count);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_run_10_2() {
        let input = include_str!("../res/day9-steps_example2.txt");
        let cmds : Result<Vec<Command>, Error> = input.lines().map(|l| l.trim_end().parse()).collect();
        let cmds = cmds.unwrap();

        let count = apply_commands_10fold(&cmds);
        println!("tail visited: {}", count);
        assert_eq!(count, 36);

    }
}
