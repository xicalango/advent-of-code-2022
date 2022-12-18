use std::collections::HashSet;
use std::str::FromStr;
use crate::utils::{Error, Surroundings};
use crate::utils::vec3::Vec3;

pub type Pos = u8;

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

impl Droplet {

    pub fn calc_surface_area(&self) -> usize {
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

}
