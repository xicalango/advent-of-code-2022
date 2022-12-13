use std::cmp::Ordering;
use std::iter::zip;
use std::mem::{replace, transmute};
use std::str::FromStr;
use crate::Error;

#[derive(Debug)]
pub enum Element {
    Value(u32),
    List(Vec<Element>)
}

impl Element {

    pub fn as_list(&self) -> Option<&Vec<Element>> {
        match self {
            Element::Value(_) => None,
            Element::List(list) => Some(list),
        }
    }

}

impl Element {
    fn is_in_right_order(&self, other: &Element) -> Ordering {
        return match self {
            Element::Value(v1) => {
                match other {
                    Element::Value(v2) => {
                        v1.cmp(v2)
                    }
                    Element::List(_) => {
                        let left = Element::List(vec![Element::Value(*v1)]);
                        left.is_in_right_order(other)
                    }
                }
            }
            Element::List(l1) => {
                match other {
                    Element::Value(v2) => {
                        let right = Element::List(vec![Element::Value(*v2)]);
                        self.is_in_right_order(&right)
                    }
                    Element::List(l2) => {
                        for (e1,e2) in zip(l1,l2) {
                            match e1.is_in_right_order(e2) {
                                Ordering::Less => return Ordering::Less,
                                Ordering::Greater => {return Ordering::Greater},
                                Ordering::Equal => {}, //cmp next
                            }
                        }

                        if l1.len() <= l2.len() {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }
                }
            }
        }
    }
}

impl FromStr for Element {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list_stack: Vec<Vec<Element>> = Vec::new();

        let mut accu_string = String::new();

        for c in s.chars() {

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

#[derive(Debug)]
pub struct ListPair(Element, Element);

impl FromIterator<Element> for ListPair {
    fn from_iter<T: IntoIterator<Item=Element>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        let item1 = i.next().unwrap();
        let item2 = i.next().unwrap();
        assert!(i.next().is_none());

        ListPair(item1, item2)
    }
}

impl ListPair {

    pub fn is_in_right_order(&self) -> bool {
        let ListPair(el1, el2) = self;
        el1.is_in_right_order(el2) == Ordering::Less
    }
}

#[derive(Debug)]
pub struct AllListPairs(Vec<ListPair>);

impl FromStr for AllListPairs {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lists = Vec::new();
        let mut collector = Vec::new();

        for line in s.lines().map(|v| v.trim_end()) {
            if line.is_empty() {
                let cur_col = replace(&mut collector, Vec::new());
                let list_pair = ListPair::from_iter(cur_col.into_iter());
                lists.push(list_pair);
                continue;
            }
            let element: Element = line.parse()?;
            collector.push(element);
        }

        Ok(AllListPairs(lists))
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

    #[test]
    fn test_parse_input() {
        let all_lists: AllListPairs = EXAMPLE.parse().unwrap();

        // println!("{:#?}", all_lists);

        let AllListPairs(pairs) = all_lists;

        for (i, pair) in pairs.iter().enumerate() {
            println!("{}: {}", i+1, pair.is_in_right_order());
        }

    }

}