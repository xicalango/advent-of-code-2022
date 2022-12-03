use std::collections::HashSet;

use crate::Scored;

pub fn find_duplicate(line: &str) -> char {
    assert_eq!(line.len() % 2, 0);

    let midpoint = line.len() / 2;

    let slice1 = &line[..midpoint];
    let slice2 = &line[midpoint..];

    let slice1_chars: HashSet<char> = slice1.chars().collect();
    let slice2_chars: HashSet<char> = slice2.chars().collect();

    let repeat_char: Vec<&char> = slice1_chars.intersection(&slice2_chars).collect();

    assert_eq!(repeat_char.len(), 1);
    *repeat_char[0]
}

impl Scored for char {
    fn get_score(&self) -> u32 {
        let ascii_value = *self as u8;

        let normalized_value = (ascii_value - ('A' as u8)) as u32;

        if normalized_value < 26 {
            normalized_value + 27
        } else {
            normalized_value - 31
        }
    }
}

#[derive(Debug)]
pub struct Day3Input(pub &'static str);

impl Scored for Day3Input {
    fn get_score(&self) -> u32 {
        let Day3Input(input) = self;
        input.lines()
            .map(str::trim_end)
            .map(find_duplicate)
            .map(|c| c.get_score())
            .sum()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    static TEST_DATA: &'static str = include_str!("../res/day3-rucksack_example.txt");

    #[test]
    fn test_find_duplicate() {
        let scores: Vec<u32> = vec![16, 38, 42, 22, 20, 19];

        let mut scores_it = scores.iter();
        for line in TEST_DATA.lines().map(str::trim_end) {
            let dup = find_duplicate(line);
            let score = dup.get_score();
            println!("line: {} dup: {} score: {}", line, dup, score);

            assert_eq!(Some(&score), scores_it.next());
        }
    }

    #[test]
    fn test_sum() {
        let sum = Day3Input(TEST_DATA).get_score();
        println!("total score: {}", sum);

        assert_eq!(sum, 157);
    }

}
