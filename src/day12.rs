use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::Error;

#[derive(Debug)]
pub struct HeightMap {
    map: Vec<Vec<u8>>,
    width: u8,
    height: u8,
    start_pos: (u8, u8),
    end_pos: (u8, u8),
}

fn parse_height(c: &char) -> u8 {
    match c {
        'S' | 'E' => 0,
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

        Ok(HeightMap{
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
                    },
                    i => ((i-1) + 'a' as u8) as char
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
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

}
