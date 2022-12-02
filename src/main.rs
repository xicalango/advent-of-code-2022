pub mod day1;
pub mod day2;

#[derive(Debug)]
pub struct Error(String);

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

    println!("total score: {}", guide.get_score());
}

fn main() {
    day1_main();
    day2_main();
}
