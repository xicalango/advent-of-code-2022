extern crate core;

use std::num::ParseIntError;

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

#[derive(Debug)]
pub struct Error(String);

impl From<ParseIntError> for Error {
    fn from(pie: ParseIntError) -> Self {
        Error(pie.to_string())
    }
}

pub trait Scored {
    fn get_score(&self) -> u32;
}

fn main() {
    println!("day1");
    day1_main();
    println!();

    println!("day2");
    day2_main();
    println!();

    println!("day3");
    day3_main();
    println!();

    println!("day4");
    day4_main();
    println!();

    println!("day5");
    day5_main();
    println!();

    println!("day6");
    day6_main();
    println!();

    println!("day7");
    day7_main();
    println!();

    println!("day8");
    day8_main();
    println!();

    println!("day9");
    day9_main();
    println!();

    println!("day10");
    day10_main();
    println!();

    println!("day11");
    day11_main();
    println!();
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
    let mut monkey_state: AllMonkeys = monkey_meta.start_eval();

    monkey_state.eval_rounds::<ReduceWorry>(20);

    let active = monkey_state.find_most_active::<2>();

    let business = active.iter().fold(1, |a, i| a * *i);
    println!("business: {}", business);

}