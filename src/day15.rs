use std::str::FromStr;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use crate::{Error, Scored};

pub use crate::utils::Vector2;
pub use crate::utils::Vec2;

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

        let mut positions = HashSet::new();

        let mut ranges = Vec::new();

        for sb in self.sensor_beacons {
            let SensorBeacon(sensor, beacon) = sb;

            let dist = sb.get_distance();
            let dist_to_row = (row - sensor.get_y()).abs();

            if dist_to_row > dist {
                continue
            }

            let remaining_dist = dist - dist_to_row;

            ranges.push((sensor.get_x()-remaining_dist..=sensor.get_x()+remaining_dist));

            for i in sensor.get_x()-remaining_dist..=sensor.get_x()+remaining_dist {
                if beacon.get_x() == &i && beacon.get_y() == &row {
                    continue
                }
                positions.insert(i);
            }
        }

        positions.len() as u64
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