use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::utils::{Error, Surroundings};
use crate::utils::bfs::{Graph, bfs};
use crate::utils::minmax::MinMax;
use crate::utils::vec3::{Vec3, Vector3};
use crate::utils::vec2::{Vec2, Vector2};

pub type Pos = i8;

pub trait SurfaceArea {
    fn calc_surface_area(&self) -> usize;
}

pub trait OuterSurfaceArea {
    fn calc_outer_surface_area(&self) -> usize;
}

#[derive(Debug)]
pub struct Droplet(HashSet<Vec3<Pos>>);

impl FromStr for Droplet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = HashSet::new();

        for line in s.lines().map(str::trim_end) {
            let pos: Vec3<Pos> = line.parse()?;
            positions.insert(pos);
        }

        Ok(Droplet(positions))
    }
}

impl SurfaceArea for Droplet {
    fn calc_surface_area(&self) -> usize {
        let Droplet(positions) = self;
        let mut counter = 0;

        for position in positions.iter() {
            for surrounding in position.get_surroundings() {
                if !positions.contains(&surrounding) {
                    counter += 1;
                }
            }
        }

        counter
    }

}

impl OuterSurfaceArea for Droplet {
    fn calc_outer_surface_area(&self) -> usize {
        self.filled_droplet().calc_surface_area()
    }

}

impl Droplet {

    pub fn min_max_x(&self) -> (&Pos, &Pos) {
        let Droplet(positions) = self;
        positions.iter().map(|v| v.get_x()).min_max().unwrap()
    }

    pub fn min_max_y(&self) -> (&Pos, &Pos) {
        let Droplet(positions) = self;
        positions.iter().map(|v| v.get_y()).min_max().unwrap()
    }

    pub fn min_max_z(&self) -> (&Pos, &Pos) {
        let Droplet(positions) = self;
        positions.iter().map(|v| v.get_z()).min_max().unwrap()
    }

    pub fn filled_droplet(&self) -> Droplet {
        let (min_z, max_z) = self.min_max_z();

        let mut positions = HashSet::new();

        for z in *min_z..=*max_z {
            let slice = self.slice_z(z);
            for filled in slice.fill_empty_spots().into_iter().map(|v| v.extend(z)) {
                positions.insert(filled);
            }
        }

        Droplet(positions)

    }

    pub fn slice_z(&self, height: Pos) -> DropletSlice {
        let Droplet(positions) = self;

        let result = positions.iter()
            .filter(|v| v.get_z() == &height)
            .map(|v| v.forget_z())
            .map(|v| v.map(|i| **i));

        DropletSlice::new(result)
    }

}

#[derive(Debug)]
pub struct DropletSlice {
    positions: HashSet<Vec2<Pos>>,
    top_left: Vec2<Pos>,
    bottom_right: Vec2<Pos>,
}

impl DropletSlice {

    pub fn new(pos: impl Iterator<Item=Vec2<Pos>>) -> DropletSlice {
        let positions: HashSet<Vec2<Pos>> = pos.collect();
        let (min_x, max_x) = positions.iter().map(|v| v.get_x()).min_max().unwrap();
        let (min_y, max_y) = positions.iter().map(|v| v.get_y()).min_max().unwrap();

        let top_left = Vec2::new(*min_x, *min_y);
        let bottom_right = Vec2::new(*max_x, *max_y);

        DropletSlice {
            positions,
            top_left,
            bottom_right,
        }
    }

    pub fn bounding_box(&self) -> (&Vec2<Pos>, &Vec2<Pos>) {
        (&self.top_left, &self.bottom_right)
    }

    pub fn set_top_left(&mut self, top_left: Vec2<Pos>) {
        self.top_left = top_left;
    }

    pub fn set_bottom_right(&mut self, bottom_right: Vec2<Pos>) {
        self.bottom_right = bottom_right;
    }

    pub fn fill_empty_spots(&self) -> HashSet<Vec2<Pos>> {
        let (mv@Vec2(min_x, min_y), Vec2(max_x, max_y)) = self.bounding_box();

        let reachable = bfs(self, &mv.map(|v| v.saturating_sub(1)));

        let mut filled_positions: HashSet<Vec2<Pos>> = HashSet::new();

        for y in *min_y..=*max_y {
            for x in *min_x..=*max_x {
                let pos = Vec2::new(x, y);
                if !reachable.contains_key(&pos) {
                    filled_positions.insert(pos);
                }
            }
        }

        filled_positions

    }

}

impl Graph for DropletSlice {
    type Position = Vec2<Pos>;
    type Property = ();

    fn get_property(&self, _: &Self::Position) -> &Self::Property {
        &()
    }

    fn get_surroundings(&self, pos: &Self::Position) -> Vec<Self::Position> {
        let Vec2(min_x, min_y) = self.top_left.map(|v| v.saturating_sub(2));
        let Vec2(max_x, max_y) = self.bottom_right.map(|v| v.saturating_add(2));

        pos.get_surroundings().iter().filter(|p| {
            p.get_x() >= &min_x &&
                p.get_y() >= &min_y &&
                p.get_x() <= &max_x &&
                p.get_y() <= &max_y &&
                !self.positions.contains(p)
        }).map(|v| v.clone()).collect()
    }
}

impl Display for DropletSlice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (Vec2(min_x, min_y), Vec2(max_x, max_y)) = self.bounding_box();

        for y in *min_y..=*max_y {
            for x in *min_x..=*max_x {
                let char = if self.positions.contains(&Vec2::new(x, y)) {
                    "#"
                } else {
                    "."
                };
                write!(f, "{}", char)?;
            }
            write!(f, "\n")?;
        }

        Ok(())

    }
}

#[cfg(test)]
mod test {
    use crate::day18::*;

    static EXAMPLE: &'static str = include_str!("../res/day18-faces_example.txt");

    #[test]
    fn test_parse() {
        let droplet: Droplet = EXAMPLE.parse().unwrap();
        println!("{:#?}", droplet);
        println!("surface area: {}", droplet.calc_surface_area());
    }

    #[test]
    fn test_slice() {
        let droplet: Droplet = EXAMPLE.parse().unwrap();

        println!("hollow");
        for i in 1..=6 {
            let mut slice = droplet.slice_z(i);
            slice.set_top_left(Vec2(1, 1));
            slice.set_bottom_right(Vec2(3, 3));
            println!("{}", slice);
        }

        let filled_droplet = droplet.filled_droplet();

        println!("filled");
        for i in 1..=6 {
            let mut slice = filled_droplet.slice_z(i);
            slice.set_top_left(Vec2(1, 1));
            slice.set_bottom_right(Vec2(3, 3));
            println!("{}", slice);
        }

        let outer_surface_area = droplet.calc_outer_surface_area();
        assert_eq!(58, outer_surface_area);
    }

}
