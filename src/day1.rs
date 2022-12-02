
pub fn read_elf_calories(calories_data: &str) -> Vec<Vec<u32>> {
    let mut all_data = Vec::new();


    let mut cur_data = Vec::new();
    for line in calories_data.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            all_data.push(cur_data.clone());
            cur_data.clear();
            continue;
        }

        let calories: u32 = line.parse().expect("invalid calories line");
        cur_data.push(calories);
    }

    if !cur_data.is_empty() {
        all_data.push(cur_data.clone());
    }

    all_data
}

pub fn accumulate_per_elf(elf_calories: &Vec<Vec<u32>>) -> Vec<u32> {
    elf_calories.iter().map(|v| v.iter().sum()).collect()
}

pub fn find_most_calories_elf(accumulated_calories: &Vec<u32>) -> usize {
    let most_calories_entry = accumulated_calories.iter().enumerate().max_by_key(|e| e.1);
    let most_calories_entry = most_calories_entry.expect("no thicc boy?");
    most_calories_entry.0
}

pub fn find_topk_calories_elfs(accumulated_calories: &Vec<u32>, k: usize) -> Vec<usize> {
    let mut calories: Vec<(usize, &u32)> = accumulated_calories.iter().enumerate().collect();
    calories.sort_by_key(|v| v.1);
    calories.iter().rev().take(k).map(|v| v.0).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_aoc2022_data() {
        let data = include_str!("../res/day1-calories_example.txt");

        let calorie_data = read_elf_calories(data);

        println!("{:#?}", calorie_data);
    }

    #[test]
    fn test_per_elf_data_aoc2022_data() {
        let data = include_str!("../res/day1-calories_example.txt");
        let calorie_data = read_elf_calories(data);
        let accumulated_data = accumulate_per_elf(&calorie_data);

        println!("{:#?}", accumulated_data);
    }

    #[test]
    fn test_find_most_calories_elf() {
        let data = include_str!("../res/day1-calories_example.txt");
        let calorie_data = read_elf_calories(data);
        let accumulated_data = accumulate_per_elf(&calorie_data);
        let most_calories_elf_idx = find_most_calories_elf(&accumulated_data);

        println!("elf number: {} carries {}", most_calories_elf_idx + 1, accumulated_data[most_calories_elf_idx]);
        assert_eq!(3, most_calories_elf_idx);
        assert_eq!(24000, accumulated_data[most_calories_elf_idx]);
    }

    #[test]
    fn test_find_topk_calories_elf() {
        let data = include_str!("../res/day1-calories_example.txt");
        let calorie_data = read_elf_calories(data);
        let accumulated_data = accumulate_per_elf(&calorie_data);
        let top3_calories_idxs = find_topk_calories_elfs(&accumulated_data, 3);

        println!("top3 elfes are: {:?}", top3_calories_idxs);

        let total_calories: u32 = top3_calories_idxs.iter().map(|v| accumulated_data[*v]).sum();

        println!("they carry: {}", total_calories);

        assert_eq!(vec![3, 2, 4], top3_calories_idxs);
        assert_eq!(45000, total_calories);
    }
}


