use std::str::FromStr;
use std::cmp::{max, min};
use crate::Error;

use crate::utils::Vector2;
use crate::utils::Vec2;

pub type Pos = i64;
pub type PosVec = Vec2<Pos>;

#[derive(Debug)]
pub struct SensorBeacon(pub PosVec, pub PosVec);

impl SensorBeacon {
    fn get_distance(&self) -> Pos {
        let SensorBeacon(sp, bp) = self;
        sp.clone() | bp.clone()
    }
}

impl FromStr for SensorBeacon {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(&[' ', '=', ',', ':'][..]).collect();

        match split[..] {
            ["Sensor", "at", "x", sx, "", "y", sy, "", "closest", "beacon", "is", "at", "x", bx, "", "y", by] => {
                let sx: Pos = sx.trim().parse().map_err(|_| Error(format!("cannot parse {}", sx)))?;
                let sy: Pos = sy.trim().parse().map_err(|_| Error(format!("cannot parse {}", sy)))?;
                let bx: Pos = bx.trim().parse().map_err(|_| Error(format!("cannot parse {}", bx)))?;
                let by: Pos = by.trim().parse().map_err(|_| Error(format!("cannot parse {}", by)))?;
                Ok(SensorBeacon(Vec2(sx, sy), Vec2(bx, by)))
            }
            _ => Err(Error(format!("invalid line: {:?}", split)))
        }
    }
}

pub struct BeaconFinder<'a> {
    sensor_beacons: &'a Vec<SensorBeacon>,
}

impl<'a> BeaconFinder<'a> {

    pub fn new(sensor_beacons : &'a Vec<SensorBeacon>) -> BeaconFinder {
        BeaconFinder {
            sensor_beacons
        }
    }

    pub fn find_impossible_beacon_positions(&self, row: Pos) -> u64 {
        let min_x = self.sensor_beacons.iter().map(|SensorBeacon(sp, bp)| min(sp.get_x(), bp.get_x())).min().unwrap();
        let max_x = self.sensor_beacons.iter().map(|SensorBeacon(sp, bp)| max(sp.get_x(), bp.get_x())).max().unwrap();

        let mut counter = 0;

        let mut pos = PosVec::new(0, row);

        for x in min_x-2000000..=max_x+2000000 {
            pos.set_x(x);

            let any_beacon = self.sensor_beacons.iter()
                .any(|sb@SensorBeacon(s, b)| {
                    if &pos == b {
                        return false;
                    }
                    let dist = sb.get_distance();
                    let x_dist = s.clone() | pos.clone();
                    x_dist <= dist
                });

            if any_beacon {
                counter += 1;
            }
        }

        counter
    }

}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = include_str!("../res/day15-beacons_example.txt");

    #[test]
    fn test_parse_input() {
        let sensor_beacons: Result<Vec<SensorBeacon>, Error> = EXAMPLE.lines().map(str::trim_end).map(str::parse).collect();
        let sensor_beacons = sensor_beacons.unwrap();
        println!("{:?}", sensor_beacons);

        let beacon_finder = BeaconFinder::new(&sensor_beacons);
        let count = beacon_finder.find_impossible_beacon_positions(10);
        println!("count: {}", count);
        assert_eq!(count, 26);
    }
}