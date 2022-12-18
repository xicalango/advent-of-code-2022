use std::collections::HashSet;
use std::str::FromStr;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::{Error, Scored};

use crate::utils::ranges::{RangeExt, RangeLength};

pub use crate::utils::vec2::Vector2;
pub use crate::utils::vec2::Vec2;

pub type Pos = i64;
pub type PosVec = Vec2<Pos>;

impl Scored for PosVec {
    fn get_score(&self) -> u64 {
        let Vec2(x, y) = self;
        (x * 4_000_000 + y) as u64
    }
}

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

#[derive(Debug, Default)]
pub struct RangeCollector {
    ranges: Vec<RangeInclusive<Pos>>,
}

impl RangeCollector {
    fn add_range(&mut self, add_range: RangeInclusive<Pos>) {
        let mut add_ranges = vec![add_range];

        for range in self.ranges.iter() {
            let mut new_ranges = Vec::new();

            for add_range in add_ranges.drain(..) {
                let split_off = add_range.split_off(range);

                if let Some(r1) = &split_off[0] {
                    new_ranges.push(r1.clone());
                }

                if let Some(r2) = &split_off[2] {
                    new_ranges.push(r2.clone());
                }
            }


            add_ranges = new_ranges;
        }

        for range in add_ranges {
            self.ranges.push(range);
        }
    }

    pub fn ranges(self) -> Vec<RangeInclusive<Pos>> {
        self.ranges
    }

    pub fn contains(&self, value: &Pos) -> bool {
        self.ranges.iter().any(|r| r.contains(value))
    }

    pub fn find_first_gap(&self) -> Option<Pos> {
        if self.ranges.is_empty() {
            return None;
        }

        let mut sorted_ranges = self.ranges.clone();
        sorted_ranges.sort_by_key(|r| *r.start());

        let mut iter = sorted_ranges.into_iter();
        let mut last_range = iter.next().unwrap();

        for range in iter {
            if last_range.end() + 1 != *range.start() {
                return Some(last_range.end() + 1);
            }
            last_range = range;
        }

        None
    }

    pub fn find_gaps(&self, search_range: RangeInclusive<Pos>) -> Vec<Pos> {
        search_range.into_iter().filter(|r| !self.contains(r)).collect()
    }

}

impl FromIterator<RangeInclusive<Pos>> for RangeCollector {
    fn from_iter<T: IntoIterator<Item=RangeInclusive<Pos>>>(iter: T) -> Self {
        let mut range_collector = RangeCollector::default();
        for item in iter {
            range_collector.add_range(item);
        }
        range_collector
    }
}

pub struct BeaconFinder<'a> {
    sensor_beacons: &'a Vec<SensorBeacon>,
}

impl<'a> BeaconFinder<'a> {
    pub fn new(sensor_beacons: &'a Vec<SensorBeacon>) -> BeaconFinder {
        BeaconFinder {
            sensor_beacons
        }
    }

    pub fn find_impossible_locations(&self, row: &Pos) -> RangeCollector {
        let mut range_collector = RangeCollector::default();

        for sb in self.sensor_beacons {
            let SensorBeacon(sensor, _) = sb;

            let dist = sb.get_distance();
            let dist_to_row = (row - sensor.get_y()).abs();

            if dist_to_row > dist {
                continue;
            }

            let remaining_dist = dist - dist_to_row;

            let range = sensor.get_x() - remaining_dist..=sensor.get_x() + remaining_dist;
            range_collector.add_range(range);
        }

        range_collector
    }

    pub fn find_beacons_on_row(&self, row: &Pos) -> usize {
        let mut xcoords = HashSet::new();

        self.sensor_beacons.iter()
            .filter(|SensorBeacon(_, beacon)| {
                xcoords.insert(beacon.get_x()) && beacon.get_y() == row
            }).count()
    }

    pub fn find_impossible_beacon(&self, row: &Pos) -> usize {
        let ranges = self.find_impossible_locations(row).ranges();
        let beacons_on_row = self.find_beacons_on_row(&row);

        let len: usize = ranges.iter().map(|r| r.len() as usize).sum();
        len - beacons_on_row
    }

    pub fn find_beacon_location_in_range(&self, range: RangeInclusive<Pos>) -> Option<PosVec> {
        for row in range {
            let range_collector = self.find_impossible_locations(&row);
            if let Some(first_gap) = range_collector.find_first_gap() {
                return Some(Vec2(first_gap, row))
            }
        }

        None
    }

    pub fn find_beacon_location_threaded<const N_THREADS: Pos>(&self, max_row: Pos) -> PosVec {
        let batch_size = max_row / N_THREADS;

        let target: Arc<Mutex<Option<PosVec>>> = Arc::new(Mutex::new(None));

        thread::scope(|scope| {

            let mut join_handles = Vec::new();

            for i in 0..N_THREADS {
                let target_clone = target.clone();

                let start_batch = i * batch_size;
                let end_batch = start_batch + batch_size - 1;

                let search_range = start_batch as Pos..=end_batch as Pos;
                let thread = scope.spawn(move|| {
                    let result = self.find_beacon_location_in_range(search_range);
                    if result.is_some() {
                        *target_clone.lock().unwrap() = result
                    }
                });
                join_handles.push(thread);
            }

            for handle in join_handles {
                handle.join().unwrap();
            }
        });

        let locked = target.lock().unwrap();
        locked.as_ref().unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = include_str!("../res/day15-beacons_example.txt");

    #[test]
    fn test_smarter_part1() {
        let sensor_beacons: Result<Vec<SensorBeacon>, Error> = EXAMPLE.lines().map(str::trim_end).map(str::parse).collect();
        let sensor_beacons = sensor_beacons.unwrap();

        println!("{:?}", sensor_beacons);

        let beacon_finder = BeaconFinder::new(&sensor_beacons);
        let count = beacon_finder.find_impossible_beacon(&10);
        println!("count: {}", count);
        assert_eq!(count, 26);
    }

    #[test]
    fn test_part_2() {
        let sensor_beacons: Result<Vec<SensorBeacon>, Error> = EXAMPLE.lines().map(str::trim_end).map(str::parse).collect();
        let sensor_beacons = sensor_beacons.unwrap();

        println!("{:?}", sensor_beacons);

        let beacon_finder = BeaconFinder::new(&sensor_beacons);
        let pos = beacon_finder.find_beacon_location_threaded::<4>(20);
        println!("{:?}", pos);
        assert_eq!(56000011, pos.get_score());
    }
}
