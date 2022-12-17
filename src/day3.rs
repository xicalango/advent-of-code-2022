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
    fn get_score(&self) -> u64 {
        let ascii_value = *self as u8;

        let normalized_value = (ascii_value - ('A' as u8)) as u64;

        if normalized_value < 26 {
            normalized_value + 27
        } else {
            normalized_value - 31
        }
    }
}

fn chunked_iteration<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<char> {

    let mut result = Vec::new();

    loop {
        let group: Vec<Option<&str>> = vec![
            lines.next(),
            lines.next(),
            lines.next()
        ];

        if group.iter().any(Option::is_none) {
            break;
        }

        let hashsets: Vec<HashSet<char>> = group.iter()
            .map(|o| o.unwrap())
            .map(|l| l.chars().collect::<HashSet<char>>())
            .collect();

        let intersection1: HashSet<char> = hashsets[0].intersection(&hashsets[1]).map(|v| *v).collect();
        let intersection2: Vec<&char> = intersection1.intersection(&hashsets[2]).collect();

        assert_eq!(intersection2.len(), 1);
        result.push(*intersection2[0]);
    }

    result
}

#[derive(Debug)]
pub struct Day3Input(pub &'static str);

impl Scored for Day3Input {
    fn get_score(&self) -> u64 {
        let Day3Input(input) = self;
        input.lines()
            .map(str::trim_end)
            .map(find_duplicate)
            .map(|c| c.get_score())
            .sum()
    }
}

#[derive(Debug)]
pub struct Day3Chunked(pub &'static str);

impl From<Day3Input> for Day3Chunked {
    fn from(input: Day3Input) -> Self {
        let Day3Input(value) = input;
        Day3Chunked(value)
    }
}

impl Scored for Day3Chunked {
    fn get_score(&self) -> u64 {
        let Day3Chunked(input) = self;
        chunked_iteration(&mut input.lines()).iter()
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
        let scores: Vec<u64> = vec![16, 38, 42, 22, 20, 19];

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

    #[test]
    fn test_chunked() {
        let chunked: Day3Chunked = Day3Input(TEST_DATA).into();
        let sum = chunked.get_score();

        println!("chunked score: {}", sum);
        assert_eq!(sum, 70);
    }

}
