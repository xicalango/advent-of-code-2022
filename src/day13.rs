use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::zip;
use std::mem::replace;
use std::str::FromStr;
use crate::Error;

#[derive(Debug, Eq, Ord, Clone)]
pub enum Element {
    Value(u32),
    List(Vec<Element>)
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Value(v) => write!(f, "{}", v)?,
            Element::List(l) => {
                write!(f, "[")?;
                for (i, e) in l.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i < l.len()-1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

impl Element {
    pub fn as_list(&self) -> Option<&Vec<Element>> {
        match self {
            Element::Value(_) => None,
            Element::List(list) => Some(list),
        }
    }
}

impl PartialEq<Self> for Element {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Element) -> Option<Ordering> {
        let result = Some(match self {
            Element::Value(v1) => {
                match other {
                    Element::Value(v2) => {
                        v1.cmp(v2)
                    }
                    Element::List(_) => {
                        let left = Element::List(vec![Element::Value(*v1)]);
                        left.partial_cmp(other).unwrap()
                    }
                }
            }
            Element::List(l1) => {
                match other {
                    Element::Value(v2) => {
                        let right = Element::List(vec![Element::Value(*v2)]);
                        self.partial_cmp(&right).unwrap()
                    }
                    Element::List(l2) => {
                        for (e1, e2) in zip(l1, l2) {
                            let ordering = e1.partial_cmp(e2).unwrap();
                            match ordering {
                                Ordering::Less => return Some(Ordering::Less),
                                Ordering::Greater => return Some(Ordering::Greater),
                                Ordering::Equal => {}, //cmp next
                            }
                        }

                        l1.len().cmp(&l2.len())
                    }
                }
            }
        });
        return result
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
pub struct ListPair(pub Element, pub Element);

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
        el1.cmp(el2) == Ordering::Less
    }
}

#[derive(Debug)]
pub struct AllListPairs(pub Vec<ListPair>);

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
    static EXAMPLE1: &'static str = include_str!("../res/day13-lists_example1.txt");

    #[test]
    fn test_parse_lists() {
        let elements: Result<Vec<Element>, Error> = EXAMPLE.lines().filter(|l| !l.is_empty()).map(|l| l.parse()).collect();
        let mut elements = elements.unwrap();

        let div1 = Element::List(vec![Element::List(vec![Element::Value(2)])]);
        let div2 = Element::List(vec![Element::List(vec![Element::Value(6)])]);

        elements.push(div1.clone());
        elements.push(div2.clone());

        elements.sort();

        let mut accu: usize = 1;

        for (i, e) in elements.iter().enumerate() {
            println!("{}: {}", i+1, e);
            if e == &div1 || e == &div2 {
                accu *= i+1;
            }
        }

        println!("accu {}", accu);
        assert_eq!(accu, 140);
    }

    #[test]
    fn test_parse_lists1() {
        let elements: Result<Vec<Element>, Error> = EXAMPLE1.lines().filter(|l| !l.is_empty()).map(|l| l.parse()).collect();
        let mut elements = elements.unwrap();

        let div1 = Element::List(vec![Element::List(vec![Element::Value(2)])]);
        let div2 = Element::List(vec![Element::List(vec![Element::Value(6)])]);

        elements.sort();

        let mut accu: usize = 1;

        for (i, e) in elements.iter().enumerate() {
            println!("{}: {}", i+1, e);
        }

        println!("accu {}", accu);
    }

    #[test]
    fn test_parse_input() {
        let all_lists: AllListPairs = EXAMPLE.parse().unwrap();

        // println!("{:#?}", all_lists);

        let AllListPairs(pairs) = all_lists;

        for (i, pair) in pairs.iter().enumerate() {
            println!("{}: {}", i+1, pair.is_in_right_order());
            println!("{}, {}", pair.0, pair.1);
        }

        let code: usize = pairs.iter().enumerate().filter(|(_,e)| e.is_in_right_order()).map(|(i, _)| i+1).sum();
        println!("code: {}", code);
    }

}