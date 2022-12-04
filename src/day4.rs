
use crate::Error;

use std::{ops::RangeInclusive, str::FromStr, num::ParseIntError};


#[derive(Debug)]
pub struct SectionRange(RangeInclusive<usize>);

#[derive(Debug)]
pub struct SectionAssignment(SectionRange, SectionRange);

impl SectionAssignment {

    pub fn find_overlap(&self) -> RangeInclusive<usize> {
        let SectionAssignment(SectionRange(range1), SectionRange(range2)) = &self;

        let max_start = std::cmp::max(range1.start(), range2.start());
        let min_end = std::cmp::min(range1.end(), range2.end());

        *max_start..=*min_end
    }

    pub fn is_work_done_twice(&self) -> bool {
        let SectionAssignment(SectionRange(range1), SectionRange(range2)) = &self;
        let overlap = self.find_overlap();

        return &overlap == range1 || &overlap == range2;
    }

    pub fn has_overlap(&self) -> bool {
        let overlap = self.find_overlap();

        return overlap.end() >= overlap.start();
    }

}

#[derive(Debug)]
pub struct SectionAssignments(pub Vec<SectionAssignment>);

impl FromStr for SectionRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once('-').ok_or(Error(format!("invalid section range: {}", s)))?;

        let first_number = first.parse().map_err(|e: ParseIntError| Error(e.to_string()))?;
        let second_number = second.parse().map_err(|e: ParseIntError| Error(e.to_string()))?;

        Ok(SectionRange(first_number..=second_number))
    }
}

impl FromStr for SectionAssignment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_range, secong_range) = s.split_once(',').ok_or(Error(format!("invalid section assignment: {}", s)))?;

        let first_range = first_range.parse()?;
        let second_range = secong_range.parse()?;

        Ok(SectionAssignment(first_range, second_range))
    }
}

impl FromStr for SectionAssignments {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<SectionAssignment>, Error> = s.lines().map(|s| s.parse()).collect();
        Ok(SectionAssignments(res?))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &'static str = include_str!("../res/day4-ranges_example.txt");

    #[test]
    fn test_parse() {
        let assignments: SectionAssignments = TEST_INPUT.parse().unwrap();

        println!("{:#?}", assignments);
    }

    #[test]
    fn test_work_done_twice() {
        let assignments: SectionAssignments = TEST_INPUT.parse().unwrap();
        let SectionAssignments(assignments) = assignments;
        let count = assignments.iter().filter(|a| a.is_work_done_twice()).count();

        println!("twice work times: {}", count);
    }

    #[test]
    fn test_overlap() {
        let assignments: SectionAssignments = TEST_INPUT.parse().unwrap();
        let SectionAssignments(assignments) = assignments;
        let count = assignments.iter().filter(|a| a.has_overlap()).count();

        println!("overlapping work times: {}", count);
    }


}