use std::fmt::{Debug, Formatter};
use std::hash::Hash;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Debug, Copy, Clone)]
pub enum Turn {
    Left,
    Right,
}

impl Direction {

    pub fn get_left_turn_direction(&self) -> Direction {
        use Direction::*;

        match self {
            Right => Up,
            Up => Left,
            Left => Down,
            Down => Right,
        }
    }

    pub fn get_right_turn_direction(&self) -> Direction {
        use Direction::*;

        match self {
            Right => Down,
            Up => Right,
            Left => Up,
            Down => Left,
        }
    }

    pub fn turn(&mut self, turn: Turn) {
        let dir = match turn {
            Turn::Left => self.get_left_turn_direction(),
            Turn::Right => self.get_right_turn_direction(),
        };

        *self = dir;
    }

}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

pub trait Position: Clone + Eq + Hash {
    fn get_step_position(&self, dir: &Direction) -> Self;

    fn step(&mut self, dir: &Direction) where Self: Sized {
        *self = self.get_step_position(dir);
    }
}

pub trait World {
    type Position: Position;

    fn wrap_position(&self, pos: &mut Self::Position);
    fn is_accessible(&self, pos: &Self::Position) -> bool;
}

pub struct Turtle<'a, W>
    where W: World {
    world: &'a W,
    turtle_pos: W::Position,
    turtle_direction: Direction,
}

pub enum StepError<P: Position> {
    Inaccessible(P),
    NStepError(usize, Box<StepError<P>>)
}

impl<P: Position + Debug> Debug for StepError<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StepError::Inaccessible(p) => write!(f, "Inaccessible({:?})", p),
            StepError::NStepError(n, e) => write!(f, "NStepError({}, {:?})", n, e),
        }
    }
}

impl<'a, W> Turtle<'a, W>
where W: World {

    pub fn new(world: &'a W, turtle_pos: W::Position, turtle_direction: Direction) -> Turtle<'a, W> {
        Turtle {
            world,
            turtle_pos,
            turtle_direction,
        }
    }

    pub fn turn(&mut self, turn: Turn) {
        self.turtle_direction.turn(turn);
    }

    pub fn step(&mut self) -> Result<(), StepError<W::Position>> {
        let mut new_pos = self.turtle_pos.get_step_position(&self.turtle_direction);
        self.world.wrap_position(&mut new_pos);

        if !self.world.is_accessible(&new_pos) {
            return Err(StepError::Inaccessible(new_pos));
        }

        self.turtle_pos.step(&self.turtle_direction);
        Ok(())
    }

    pub fn n_step(&mut self, n: usize) -> Result<(), StepError<W::Position>> {
        for step in 1..=n {
            self.step()
                .map_err(|e| StepError::NStepError(step, Box::new(e)))?;
        }

        Ok(())
    }

    pub fn turtle_pos(&self) -> &W::Position {
        &self.turtle_pos
    }

    pub fn turtle_dir(&self) -> &Direction {
        &self.turtle_direction
    }

}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use super::*;
    use crate::utils::vec2::*;

    impl Position for Vec2<i32> {
        fn get_step_position(&self, dir: &Direction) -> Self {
            let Vec2(x, y) = self;

            match dir {
                Direction::Right => Vec2::new(x + 1, *y),
                Direction::Up => Vec2::new(*x, y - 1),
                Direction::Left => Vec2::new(x - 1, *y),
                Direction::Down => Vec2::new(*x, y + 1)
            }
        }
    }

    impl World for HashSet<Vec2<i32>> {
        type Position = Vec2<i32>;

        fn wrap_position(&self, _: &mut Self::Position) {
        }

        fn is_accessible(&self, pos: &Self::Position) -> bool {
            !self.contains(pos)
        }
    }

    #[test]
    fn test_turtle() {
        let mut world = HashSet::new();
        world.insert(Vec2::new(1, -4));

        let mut turtle = Turtle::new(&world, Vec2::new(0, 0), Direction::default());

        println!("turtle pos: {:?}", turtle.turtle_pos());

        turtle.step().unwrap();
        turtle.turn(Turn::Left);

        println!("turtle pos: {:?}", turtle.turtle_pos());
        turtle.n_step(10).unwrap();

        println!("turtle pos: {:?}", turtle.turtle_pos());

    }

}


