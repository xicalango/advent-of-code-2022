pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

#[derive(Debug)]
pub struct Error(String);

pub trait Scored {
    fn get_score(&self) -> u32;
}

fn main() {
    println!("day1");
    day1_main();

    println!("day2");
    day2_main();

    println!("day3");
    day3_main();

    println!("day4");
    day4_main();

    println!("day5");
    day5_main();

    println!("day6");
    day6_main();
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
