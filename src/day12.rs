
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::Error;
use crate::utils::bfs::{Bfs, Graph};

type Vec2 = (u8, u8);

#[derive(Debug)]
pub struct HeightMap {
    map: Vec<Vec<u8>>,
    width: u8,
    height: u8,
    start_pos: Vec2,
    end_pos: Vec2,
}

fn parse_height(c: &char) -> u8 {
    match c {
        'S' => 1,
        'E' => 26,
        c => (*c as u8 - 'a' as u8) + 1
    }
}

impl FromStr for HeightMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width: Option<u8> = None;
        let mut map: Vec<Vec<u8>> = Default::default();
        let mut start_pos: Option<(u8, u8)> = None;
        let mut end_pos: Option<(u8, u8)> = None;

        for line in s.lines() {
            let line = line.trim_end();

            if width.is_none() {
                width = Some(line.len() as u8);
            }
            let mut row = Vec::new();

            for (i, c) in line.chars().enumerate() {
                row.push(parse_height(&c));
                if c == 'S' {
                    assert!(start_pos.is_none());
                    start_pos = Some((i as u8, map.len() as u8))
                } else if c == 'E' {
                    assert!(end_pos.is_none());
                    end_pos = Some((i as u8, map.len() as u8))
                }
            }

            map.push(row);
        }

        let height = map.len() as u8;

        Ok(HeightMap {
            map,
            width: width.unwrap(),
            height,
            start_pos: start_pos.unwrap(),
            end_pos: end_pos.unwrap(),
        })
    }
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, height) in row.iter().enumerate() {
                let c = match height {
                    0 => {
                        let pos = (x as u8, y as u8);
                        if self.start_pos == pos {
                            'S'
                        } else if self.end_pos == pos {
                            'E'
                        } else {
                            panic!();
                        }
                    }
                    i => ((i - 1) + 'a' as u8) as char
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl HeightMap {
    pub fn filtered_bfs<F: Fn(&u8, &u8) -> bool>(&self, filter: F) -> Bfs<F, HeightMap> {
        Bfs::new(self, filter)
    }

    pub fn surroundings(&self, pos: &Vec2) -> Vec<Vec2> {
        let mut result = Vec::new();

        let (x, y) = pos;

        if *x > 0 {
            result.push((x - 1, *y));
        }

        if *x < self.width - 1 {
            result.push((x + 1, *y));
        }

        if *y > 0 {
            result.push((*x, y - 1));
        }

        if *y < self.height - 1 {
            result.push((*x, y + 1));
        }

        result
    }

    pub fn get_at(&self, pos: &Vec2) -> &u8 {
        let (x, y) = pos;
        &self.map[*y as usize][*x as usize]
    }

    pub fn get_end_pos(&self) -> &Vec2 {
        &self.end_pos
    }

    pub fn get_start_pos(&self) -> &Vec2 {
        &self.start_pos
    }

    pub fn get_lowest_positions(&self) -> Vec<Vec2> {
        let mut result = Vec::new();
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 1 {
                    result.push((x as u8, y as u8));
                }
            }
        }

        result
    }
}

impl Graph for HeightMap {
    type Position = Vec2;
    type Property = u8;

    fn get_property(&self, pos: &Self::Position) -> &Self::Property {
        self.get_at(pos)
    }

    fn get_surroundings(&self, pos: &Self::Position) -> Vec<Self::Position> {
        self.surroundings(pos)
    }
}

pub fn can_climb(cur: &u8, next: &u8) -> bool {
    *next <= *cur || *next == cur+1
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = include_str!("../res/day12-map_example.txt");

    #[test]
    fn test_parse() {
        let hm: HeightMap = EXAMPLE.parse().unwrap();

        println!("{}", hm);
    }

    #[test]
    fn test_bfs() {
        let hm: HeightMap = EXAMPLE.parse().unwrap();

        let bfs = hm.filtered_bfs(can_climb);
        let dists = bfs.run(&hm.start_pos);

        for row in 0..hm.height {
            for h in 0..hm.width {
                print!("{:02}  ", dists[&(h, row)]);
            }
            println!();
        }

        let (ex, ey) = &hm.end_pos;

        let steps = dists[&(*ex, *ey)];
        println!("{}", steps);
        assert_eq!(31, steps);
    }

    #[test]
    fn test_ms_bfs() {
        let hm: HeightMap = EXAMPLE.parse().unwrap();
        let end_pos = &hm.get_end_pos();
        
        let bfs = hm.filtered_bfs(|c, n| *c <= *n || *c == n+1);
        let dists = bfs.run(&end_pos);
        
        let lowest = *hm.get_lowest_positions().iter().filter_map(|(lx, ly)| dists.get(&(*lx, *ly))).min().unwrap();

        println!("{}", lowest);
        assert_eq!(29, lowest);
    }
}
