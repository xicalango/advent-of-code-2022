use std::str::FromStr;
use crate::Error;

#[derive(Debug)]
pub enum Element {
    Value(u32),
    List(Vec<Element>)
}

impl FromStr for Element {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list_stack: Vec<Vec<Element>> = Vec::new();

        let mut accu_string = String::new();

        for c in s.trim_end().chars() {

            match c {
                '[' => list_stack.push(Vec::new()),
                ']' => {
                    let mut last_list = list_stack.pop().ok_or(Error(format!("invalid nesting for: {} (on ])", s)))?;
                    if !accu_string.is_empty() {
                        let v: u32 = accu_string.parse()?;
                        last_list.push(Element::Value(v));
                        accu_string.clear();
                    }
                    match list_stack.last_mut() {
                        None => return Ok(Element::List(last_list)),
                        Some(l) => l.push(Element::List(last_list)),
                    }
                },
                ',' => {
                    let last_list = list_stack.last_mut().ok_or(Error(format!("invalid nesting for: {} (on ,)", s)))?;

                    if !accu_string.is_empty() {
                        let v: u32 = accu_string.parse()?;
                        last_list.push(Element::Value(v));
                        accu_string.clear();
                    }
                },
                v => {
                    accu_string.push(v);
                }
            }

        }

        Err(Error(format!("invalid string: {}", s)))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    static EXAMPLE: &'static str = include_str!("../res/day13-lists_example.txt");

    #[test]
    fn test_parse_lists() {
        for list_line in EXAMPLE.lines().filter(|l| !l.is_empty()) {
            let list: Element = list_line.parse().unwrap();
            println!("{:#?}", list);
        }

    }

}