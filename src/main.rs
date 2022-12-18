mod bench;
pub mod utils;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day18;
pub mod day17;

use bench::Bench;
use crate::utils::Error;

pub trait Scored {
    fn get_score(&self) -> u64;
}

fn main() {
    let mut bench = Bench::new();

    bench.run_day(1, day1_main);
    bench.run_day(2, day2_main);
    bench.run_day(3, day3_main);
    bench.run_day(4, day4_main);
    bench.run_day(5, day5_main);
    bench.run_day(6, day6_main);
    bench.run_day(7, day7_main);
    bench.run_day(8, day8_main);
    bench.run_day(9, day9_main);
    bench.run_day(10, day10_main);
    bench.run_day(11, day11_main);
    bench.run_day(12, day12_main);
    bench.run_day(13, day13_main);
    bench.run_day(14, day14_main);
    bench.run_day(15, day15_main);
    bench.run_day(18, day18_main);

    bench.print_times();
    println!();
    bench.print_total_time();
    println!();
    bench.print_slowest_days::<5>();
}

fn day1_main() {
    use day1::*;

    let input_data = include_str!("../res/day1-calories.txt");

    let calorie_data = read_elf_calories(input_data);
    let accumulated_data = accumulate_per_elf(&calorie_data);
    let most_calories_elf_idx = find_most_calories_elf(&accumulated_data);

    println!("most calories elf: {}, carries: {}", most_calories_elf_idx+1, accumulated_data[most_calories_elf_idx]);

    let top3_elf_idxs = find_topk_calories_elfs(&accumulated_data, 3);
    let total_calories: u32 = top3_elf_idxs.iter().map(|v| accumulated_data[*v]).sum();

    println!("top3 elf idxs: {:?}, carry: {}", top3_elf_idxs, total_calories);
}

fn day2_main() {
    use day2::*;

    let input_data = include_str!("../res/day2-guide.txt");
    let guide: StrategyGuide = input_data.parse().unwrap();
    let basic_game = guide.evaluate_game::<BasicEvaluator>();
    let advanced_game = guide.evaluate_game::<AdvancedEvaluator>();

    println!("total score basic: {}", basic_game.get_score());
    println!("total score advanced: {}", advanced_game.get_score());
}

fn day3_main() {
    use day3::*;

    let input_data = include_str!("../res/day3-rucksack.txt");
    let data = Day3Input(input_data);
    let score = data.get_score();
    println!("score: {}", score);

    let chunked_data: Day3Chunked = data.into();
    let chunked_score = chunked_data.get_score();
    println!("chunked score: {}", chunked_score);
}

fn day4_main() {
    use day4::*;

    let input_data = include_str!("../res/day4-ranges.txt");

    let SectionAssignments(assignments) = input_data.parse().unwrap();

    let work_done_twice_counts = assignments.iter().filter(|a| a.is_work_done_twice()).count();

    println!("work done twice by {} elfes", work_done_twice_counts);

    let any_overlap_count = assignments.iter().filter(|a| a.has_overlap()).count();

    println!("any overlap count {}", any_overlap_count);
}

fn day5_main() {
    use day5::*;

    let input_data = include_str!("../res/day5-stacks.txt");

    let stacks: Stacks<9> = input_data.parse().unwrap();
    let instructions: AllInstructions = input_data.parse().unwrap();
    { 
        let mut stacks = stacks.clone();
        instructions.eval::<9, CrateMover9000>(&mut stacks);
        println!("9000: {}", stacks.top_stacks_str());
    }

    {
        let mut stacks: Stacks<9> = stacks.clone();
        instructions.eval::<9, CrateMover9001>(&mut stacks);
        println!("9001: {}", stacks.top_stacks_str());
    }
}

fn day6_main() {
    use day6::*;

    let input_data = include_str!("../res/day6-code.txt");

    let sync_pos = find_sync_start::<4>(input_data);

    println!("sync start: {:?}", sync_pos);

    let msg_pos = find_sync_start::<14>(input_data);

    println!("msg start: {:?}", msg_pos);
}

fn day7_main() {
    use day7::*;

    let input_data = include_str!("../res/day7-bash.txt");
    let commands: Result<Vec<Command>, Error> = input_data.lines().map(|l| l.trim_end().parse()).collect();
    let commands = commands.unwrap();

    let mut root = DirEnt::empty_dir("/");
    {
        let mut env = Environment::new(&mut root);
        for cmd in commands {
            env.eval(&cmd);
        }
    }

    let du_by_dir = root.du_by_dir();

    let sum_to_delete: usize = du_by_dir.values().filter(|v| **v <= 100_000).sum();

    println!("sum_to_delete: {}", sum_to_delete);

    let complete_usage = du_by_dir["/"];
    const DISK_SPACE: usize = 70_000_000;
    let unused_space = DISK_SPACE - complete_usage;

    const NEEDED_SPACE: usize = 30_000_000;
    let cleanup_space = NEEDED_SPACE - unused_space;

    let mut dir_sizes: Vec<isize> = du_by_dir.values().map(|f| *f as isize - cleanup_space as isize).collect();
    dir_sizes.sort();

    let element = dir_sizes.iter().find(|e| e >= &&0);

    println!("found: {:?}", element.unwrap() + cleanup_space as isize);
}

fn day8_main() {
  use day8::*;

  let input_data = include_str!("../res/day8-trees.txt");
  let field: Field<9801> = input_data.parse().unwrap();
  
  let vis_field = field.to_visibility_field();
  println!("visible trees: {}", vis_field.count_non_zero());

  let score_field = field.to_score_field();
  println!("max score: {:?}", score_field.max());
}

fn day9_main() {
    use day9::*;

    let input_data = include_str!("../res/day9-steps.txt");
    let cmds : Result<Vec<Command>, Error> = input_data.lines().map(|l| l.trim_end().parse()).collect();
    let cmds = cmds.unwrap();

    let count = apply_commands(&cmds);
    println!("visited fields: {}", count);

    let count_10fold = apply_commands_10fold(&cmds);
    println!("visited fields (10fold): {}", count_10fold);
}

fn day10_main() {
    use day10::*;

    let input_data = include_str!("../res/day10-instr.txt");
    let signal_strengths = get_signal_strength(input_data.lines());
    let signal_strength_sum: i32 = signal_strengths.iter().map(|m| m.measurement()).sum();

    println!("signal strengths: {}", signal_strength_sum);
}

fn day11_main() {
    use day11::*;

    let input_data = include_str!("../res/day11-apes.txt");
    let monkey_meta: AllMonkeyMeta = input_data.parse().unwrap();
    {
        let mut monkey_state: AllMonkeys = monkey_meta.start_eval();

        monkey_state.eval_rounds::<20>(&ReduceWorry);

        let active = monkey_state.find_most_active::<2>();

        let business = active.iter().fold(1, |a, i| a * *i);
        println!("business 20: {}", business);
    }

    {
        let mut monkey_state: AllMonkeys = monkey_meta.start_eval();

        let worry = monkey_meta.get_worry_mod();

        monkey_state.eval_rounds::<10_000>(&worry);

        let active = monkey_state.find_most_active::<2>();

        let business = active.iter().fold(1, |a, i| a * *i);
        println!("business 10000: {}", business);
    }

}

fn day12_main() {
    use day12::*;

    let input_data = include_str!("../res/day12-map.txt");

    let height_map: HeightMap = input_data.parse().unwrap();

    let bfs = height_map.filtered_bfs(height_map.get_start_pos(), can_climb);
    let dists = bfs.run();

    /*
    for row in dists.iter() {
        for h in row {
            print!("{:03}   ", h);
        }
        println!();
    }
     */

    let (ex, ey) = &height_map.get_end_pos();

    println!("{}", dists[*ey as usize][*ex as usize]);


    {
        let end_pos = &height_map.get_end_pos();

        let bfs = height_map.filtered_bfs(&end_pos, |c, n| *c <= *n || *c == n+1);
        let dists = bfs.run();

        let lowest = height_map.get_lowest_positions().iter().map(|(lx, ly)| dists[*ly as usize][*lx as usize]).filter(|v| v > &0).min().unwrap();

        println!("{}", lowest);
    }
}

fn day13_main() {
    use day13::*;

    let input_data = include_str!("../res/day13-lists.txt");

    let all_list_pairs: AllListPairs = input_data.parse().unwrap();

    let AllListPairs(pairs) = all_list_pairs;

    let code: usize = pairs.iter().enumerate()
        .filter(|(_,e)| e.is_in_right_order())
        .map(|(i, _)| i+1)
        .sum();
    println!("code: {}", code);

    {
        let elements: Result<Vec<Element>, Error> = input_data.lines().filter(|l| !l.is_empty()).map(|l| l.parse()).collect();
        let mut elements = elements.unwrap();

        let div1 = Element::List(vec![Element::List(vec![Element::Value(2)])]);
        let div2 = Element::List(vec![Element::List(vec![Element::Value(6)])]);

        elements.push(div1.clone());
        elements.push(div2.clone());

        elements.sort();

        let mut accu: usize = 1;

        for (i, e) in elements.iter().enumerate() {
            if e == &div1 || e == &div2 {
                accu *= i+1;
            }
        }

        println!("accu {}", accu);
    }
}

fn day14_main() {
    use day14::*;

    let input_data = include_str!("../res/day14-paths.txt");

    let rows: Result<Vec<LineRow>, Error> = input_data.lines().map(str::trim_end).map(|l| l.parse::<LineRow>()).collect();
    let rows = rows.unwrap();
    let bounding_box: Box = rows.iter().collect();
    let Vec2(bx, by) = bounding_box.get_bottom_right();
    {
        let stop_line = by;

        let mut world = World::new(Vec2(bx + 1, by + 1), Vec2(500, 0));
        world.insert_lines(&rows);
        println!("{}", world.view_port());

        let mut counter: usize = 0;

        loop {
            let end_pos = world.drop_sand();
            if end_pos.get_y() >= &stop_line {
                break;
            }
            counter += 1;
        }

        println!("{}", world.view_port());
        println!("rest: {}", counter);
    }

    {
        let insert_pos = Vec2(500, 0);
        let mut world = World::new(Vec2(bx * 2, by + 2), insert_pos.clone());
        world.insert_lines(&rows);

        let mut counter: usize = 0;

        loop {
            counter += 1;
            let end_pos = world.drop_sand();
            if end_pos == insert_pos {
                break;
            }
        }

        println!("rest: {}", counter);
    }
}

fn day15_main() {
    use day15::*;

    let input_data = include_str!("../res/day15-beacons.txt");
    let sensor_beacons: Result<Vec<SensorBeacon>, Error> = input_data.lines().map(str::trim_end).map(str::parse).collect();
    let sensor_beacons = sensor_beacons.unwrap();

    let beacon_finder = BeaconFinder::new(&sensor_beacons);
    let count = beacon_finder.find_impossible_beacon(&2_000_000);
    println!("count {}", count);

    let pos = beacon_finder.find_beacon_location_threaded::<4>(4_000_000);
    println!("pos: {:?} freq: {}", pos, pos.get_score());
}

fn day18_main() {
    use day18::*;

    let input_data = include_str!("../res/day18-faces.txt");
    let droplet: Droplet = input_data.parse().unwrap();

    println!("surface area: {}", droplet.calc_surface_area());
}