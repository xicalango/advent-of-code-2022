pub mod day1;
pub mod day2;

#[derive(Debug)]
pub struct Error(String);


fn day2_main() {
    use day2::*;

    let input_data = include_str!("../res/day2-guide.txt");
    let guide: StrategyGuide = input_data.parse().unwrap();

    println!("total score: {}", guide.get_score());
}

fn main() {
    day2_main();
}
