use std::collections::HashMap;
use std::fmt::Debug;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct Bench {
    times: HashMap<usize, Duration>,
}

impl Bench {
    pub fn new() -> Bench {
        Bench {
            times: HashMap::new(),
        }
    }

    pub fn run_day<F: FnOnce()>(&mut self, day: usize, f: F) {
        println!("day{}", day);
        let start = SystemTime::now();
        f();
        let elapsed = start.elapsed().unwrap_or(Duration::default());
        println!("day{} took {:?}", day, &elapsed);
        println!();
        self.times.insert(day, elapsed);
    }

    pub fn print_times(&self) {
        let mut keys: Vec<&usize> = self.times.keys().collect();
        keys.sort();
        for key in keys {
            println!("day{}: {:?}", key, self.times[key]);
        }
    }

    pub fn print_slowest_days<const N: usize>(&self) {
        let mut values: Vec<(&usize, &Duration)> = self.times.iter().collect();
        values.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));

        println!("{} slowest days:", N);
        for (day, dur) in values.iter().take(N) {
            println!("day{}: {:?}", day, dur);
        }
    }

    pub fn print_total_time(&self) {
        let mut total_time = Duration::default();
        for duration in self.times.values() {
            total_time += *duration;
        }
        println!("total time elapsed: {:?}", total_time);
    }
}

