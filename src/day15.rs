use std::str::FromStr;
use crate::Error;

use crate::utils::Vec2;

pub type Pos = i64;
pub type PosVec = Vec2<Pos>;

#[derive(Debug)]
struct SenderBeacon(PosVec, PosVec);

impl FromStr for SenderBeacon {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(&[' ', '=', ',', ':'][..]).collect();

        match split[..] {
            ["Sensor", "at", "x", sx, "", "y", sy, "", "closest", "beacon", "is", "at", "x", bx, "", "y", by] => {
                let sx: Pos = sx.trim().parse().map_err(|_| Error(format!("cannot parse {}", sx)))?;
                let sy: Pos = sy.trim().parse().map_err(|_| Error(format!("cannot parse {}", sy)))?;
                let bx: Pos = bx.trim().parse().map_err(|_| Error(format!("cannot parse {}", bx)))?;
                let by: Pos = by.trim().parse().map_err(|_| Error(format!("cannot parse {}", by)))?;
                Ok(SenderBeacon(Vec2(sx, sy), Vec2(bx, by)))
            }
            _ => Err(Error(format!("invalid line: {:?}", split)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = include_str!("../res/day15-beacons_example.txt");

    #[test]
    fn test_parse_input() {
        let sender_beacons: Result<Vec<SenderBeacon>, Error> = EXAMPLE.lines().map(str::trim_end).map(str::parse).collect();
        let sender_beacons = sender_beacons.unwrap();
        println!("{:#?}", sender_beacons);

        let mut counter = 0;

        for SenderBeacon(s,b) in sender_beacons {
            let dist = s.clone() | b.clone();
            println!("{:?} - {:?} = {}", s, b, dist);

            for x in -5..=26 {
                let pos = PosVec::new(x, 10);
                let x_dist = s.clone() | pos.clone();

                println!("  {:?} - {:?} = {}", s, pos, x_dist);

                if x_dist <= dist {
                    counter += 1;
                    break;
                }
            }
        }

        println!("counter: {}", counter);
    }
}