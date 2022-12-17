use std::str::FromStr;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use crate::{Error, Scored};

use crate::utils::ranges::RangeExt;

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
struct RangeCollector {
    ranges: Vec<RangeInclusive<Pos>>,
}

impl RangeCollector {

    fn add_range(&mut self, add_range: RangeInclusive<Pos>) {
        let mut add_range = Some(add_range);

        for range in self.ranges.iter_mut() {
            let cur_add_range = add_range.take();

            if cur_add_range.is_none() {
                break;
            }

            let cur_add_range = cur_add_range.unwrap();

            println!("trying to add range {:?} to {:?}", cur_add_range, range);
            add_range = range.join_mut(cur_add_range);
            println!("result: {:?}/{:?}", range, add_range);
        }

        if let Some(range) = add_range {
            self.ranges.push(range);
        }
    }

    pub fn ranges(&self) -> &Vec<RangeInclusive<Pos>> {
        &self.ranges
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

    pub fn find_impossible_beacon(&self, row: Pos) -> u64 {

        let mut range_collector = RangeCollector::default();

        for sb in self.sensor_beacons {
            let SensorBeacon(sensor, _) = sb;

            let dist = sb.get_distance();
            let dist_to_row = (row - sensor.get_y()).abs();

            if dist_to_row > dist {
                continue
            }

            let remaining_dist = dist - dist_to_row;

            let range = sensor.get_x() - remaining_dist..=sensor.get_x() + remaining_dist;
            println!("new range: {:?}", range);
            range_collector.add_range(range);
        }

        println!("{:?}", range_collector);

        // for (rs, re) in ranges {
        //     for i in rs..re {
        //         positions.insert(i);
        //     }
        // }

        0
    }

    pub fn find_impossible_beacon_positions<const N_THREADS: Pos>(&self, row: Pos, range_adj: Pos) -> u64 {
        let min_x = self.sensor_beacons.iter().map(|SensorBeacon(sp, bp)| min(sp.get_x(), bp.get_x())).min().unwrap();
        let max_x = self.sensor_beacons.iter().map(|SensorBeacon(sp, bp)| max(sp.get_x(), bp.get_x())).max().unwrap();

        let min_x = min_x - range_adj;
        let max_x = max_x + range_adj;
        let full_range = max_x - min_x;
        let batch_size = full_range / N_THREADS;

        let counter = Arc::new(AtomicU64::new(0));
        let beacons = Arc::new(self.sensor_beacons);

        thread::scope(|scope| {
            let mut threads = Vec::new();
            for ti in 0..N_THREADS {
                let counter_clone = counter.clone();
                let beacons_clone = beacons.clone();
                let row_clone = row.clone();

                let thread_start_idx = (ti * batch_size) + min_x;
                let thread_end_idx = thread_start_idx + batch_size;

                println!("thread {} from {} to {}", ti, thread_start_idx, thread_end_idx);

                let thread = scope.spawn(move || {
                    let mut local_counter = 0;

                    for x in thread_start_idx..thread_end_idx {
                        let pos = PosVec::new(x, row_clone);

                        let any_beacon = beacons_clone.iter()
                            .any(|sb @ SensorBeacon(s, b)| {
                                if &pos == b {
                                    return false;
                                }
                                let dist = sb.get_distance();
                                let x_dist = s.clone() | pos.clone();
                                x_dist <= dist
                            });

                        if any_beacon {
                            local_counter += 1;
                        }
                    }

                    counter_clone.fetch_add(local_counter, Ordering::Relaxed);
                });

                threads.push(thread);
            }

            for thread in threads {
                thread.join().unwrap();
            }
        });

        counter.load(Ordering::Relaxed)
    }

    pub fn find_beacon_location<const N_THREADS: usize>(&self, dimension: Pos) -> PosVec {
        let found_pos = Arc::new(Mutex::new(None));
        let found_flag = Arc::new(AtomicBool::new(false));
        let beacons = Arc::new(self.sensor_beacons);

        let batch_size = dimension as usize / N_THREADS;

        thread::scope(|scope| {
            let mut threads = Vec::new();

            for ti in 0..N_THREADS {
                let thread_start_x = (ti * batch_size) as Pos;
                let thread_end_x = thread_start_x + batch_size as Pos;

                let found_pos_clone = found_pos.clone();
                let beacons_clone = beacons.clone();
                let found_flag_clone = found_flag.clone();

                let thread = scope.spawn(move|| {
                    for try_x in thread_start_x..thread_end_x {
                        if found_flag_clone.fetch_and(true, Ordering::Relaxed) {
                            break;
                        }
                        println!("thread {} starting on {}", ti, try_x);
                        for try_y in 0..=dimension {
                            if found_flag_clone.fetch_and(true, Ordering::Relaxed) {
                                break;
                            }
                            let pos = PosVec::new(try_x, try_y);

                            let any_beacon = beacons_clone.iter()
                                .any(|sb @ SensorBeacon(s, b)| {
                                    if &pos == b {
                                        return true;
                                    }
                                    let dist = sb.get_distance();
                                    let x_dist = s.clone() | pos.clone();
                                    x_dist <= dist
                                });

                            if any_beacon {
                                continue;
                            } else {
                                *found_pos_clone.lock().unwrap() = Some(pos);
                                found_flag_clone.fetch_or(true, Ordering::Relaxed);
                                break;
                            }
                        }
                    }
                });

                threads.push(thread);
            }

            for thread in threads {
                thread.join().unwrap();
            }

        });

        let fp = found_pos.lock().unwrap();
        let pos = fp.as_ref().unwrap();
        pos.clone()
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
        let count = beacon_finder.find_impossible_beacon_positions::<4>(10, 10);
        println!("count: {}", count);
        assert_eq!(count, 26);
    }

    #[test]
    fn test_smarter_part1() {
        let sensor_beacons: Result<Vec<SensorBeacon>, Error> = EXAMPLE.lines().map(str::trim_end).map(str::parse).collect();
        let sensor_beacons = sensor_beacons.unwrap();

        println!("{:?}", sensor_beacons);

        let beacon_finder = BeaconFinder::new(&sensor_beacons);
        let count = beacon_finder.find_impossible_beacon(10);
        println!("count: {}", count);
        // assert_eq!(count, 26);
    }

    #[test]
    fn test_part_2() {
        let sensor_beacons: Result<Vec<SensorBeacon>, Error> = EXAMPLE.lines().map(str::trim_end).map(str::parse).collect();
        let sensor_beacons = sensor_beacons.unwrap();

        println!("{:?}", sensor_beacons);

        let beacon_finder = BeaconFinder::new(&sensor_beacons);
        let pos = beacon_finder.find_beacon_location::<4>(20);
        println!("{:?}", pos);
        assert_eq!(56000011, pos.get_score());
    }
}