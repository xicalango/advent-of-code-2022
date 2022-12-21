use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::Error;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {

    pub fn eval(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Sub => lhs - rhs,
            Operation::Mul => lhs * rhs,
            Operation::Div => lhs / rhs,
        }
    }
    
    pub fn invert(&self) -> Operation {
        match self {
            Operation::Add => Operation::Sub,
            Operation::Sub => Operation::Add,
            Operation::Mul => Operation::Div,
            Operation::Div => Operation::Mul,
        }
    }

}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Operation::Add => '+',
            Operation::Sub => '-',
            Operation::Mul => '*',
            Operation::Div => '/',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
pub enum MonkeyAction {
    Value(i64),
    Instruction{
        id1: String,
        op: Operation,
        id2: String,
    },
}

impl MonkeyAction {
    pub fn value(&self) -> Option<&i64> {
        if let MonkeyAction::Value(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn instruction_ids(&self) -> Option<(&str, &str)> {
        if let MonkeyAction::Instruction {id1, op: _, id2} = self {
            Some((id1.as_str(), id2.as_str()))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct MonkeyDefinition {
    id: String,
    action: MonkeyAction,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Sub),
            "*" => Ok(Operation::Mul),
            "/" => Ok(Operation::Div),
            _ => Err(Error::cannot_parse(s)),
        }
    }
}

impl FromStr for MonkeyAction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(number) => Ok(MonkeyAction::Value(number)),
            Err(_) => {
                let splits: Vec<&str> = s.splitn(3, " ").collect();
                let id1 = splits.get(0).ok_or(Error::cannot_parse(s))?.to_string();
                let op: Operation = splits[1].parse()?;
                let id2 = splits.get(2).ok_or(Error::cannot_parse(s))?.to_string();

                Ok(MonkeyAction::Instruction {
                    id1,
                    op,
                    id2,
                })
            }
        }
    }
}

impl FromStr for MonkeyDefinition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, action) = s.split_once(":").ok_or(Error::cannot_parse(s))?;

        let monkey_action: MonkeyAction = action.trim().parse()?;

        Ok(MonkeyDefinition {
            id: id.to_string(),
            action: monkey_action
        })
    }
}

#[derive(Debug)]
pub struct MonkeyDefinitions(HashMap<String, MonkeyDefinition>);

impl FromStr for MonkeyDefinitions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let definitions: Result<Vec<MonkeyDefinition>, Error> = s.lines().map(str::trim_end).map(|s| s.parse()).collect();
        let definitions = definitions?.into_iter().map(|d| (d.id.clone(), d)).collect();
        Ok(MonkeyDefinitions(definitions))
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Value(i64),
    X,
    XOp(Box<Value>, Operation, Box<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Value(v) => write!(f, "{}", v),
            Value::X => write!(f, "x"),
            Value::XOp(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
        }
    }
}

impl MonkeyDefinitions {

    fn get_initial_value_cache(&self) -> HashMap<&str, i64> {
        let MonkeyDefinitions(definitions) = self;
        definitions.iter()
            .filter_map(|(id, d)|
                d.action.value()
                    .map(|v| (id.as_str(), *v)))
            .collect()
    }

    pub fn eval(&self) -> i64 {
        let MonkeyDefinitions(definitions) = self;
        let mut value_cache: HashMap<&str, i64> = self.get_initial_value_cache();
        let mut eval_stack: Vec<&str> = Vec::new();

        eval_stack.push("root");

        while let Some(last) = eval_stack.pop() {
            if value_cache.contains_key(last) {
                continue;
            }

            eval_stack.push(last);
            let def = &definitions[last];
            if let MonkeyAction::Instruction {id1, op, id2} = &def.action {
                let id1 = id1.as_str();
                let id2 = id2.as_str();
                if value_cache.contains_key(id1) && value_cache.contains_key(id2) {
                    let lhs = value_cache[id1];
                    let rhs = value_cache[id2];
                    value_cache.insert(last, op.eval(lhs, rhs));
                } else {
                    if !value_cache.contains_key(id1) {
                        eval_stack.push(id1)
                    }
                    if !value_cache.contains_key(id2) {
                        eval_stack.push(id2)
                    }
                }
            } else {
                panic!();
            }
        }

        value_cache["root"]
    }

    pub fn human_eval(&self) -> (Value, Value) {
        let MonkeyDefinitions(definitions) = self;
        let mut value_cache: HashMap<&str, Value> = self.get_initial_value_cache().into_iter()
            .map(|(k, v)| (k, Value::Value(v))).collect();
        let mut eval_stack: Vec<&str> = Vec::new();

        value_cache.insert("humn", Value::X);

        let (root_id1, root_id2) = definitions["root"].action.instruction_ids().unwrap();

        eval_stack.push(root_id1);
        eval_stack.push(root_id2);

        while let Some(last) = eval_stack.pop() {
            if value_cache.contains_key(last) {
                continue;
            }

            eval_stack.push(last);

            let def = &definitions[last];
            if let MonkeyAction::Instruction {id1, op, id2} = &def.action {
                let id1 = id1.as_str();
                let id2 = id2.as_str();

                if value_cache.contains_key(id1) && value_cache.contains_key(id2) {
                    let lhs = &value_cache[id1];
                    let rhs = &value_cache[id2];

                    let result = match (lhs, rhs) {
                        (Value::Value(v1), Value::Value(v2)) => Value::Value(op.eval(*v1, *v2)),
                        (x@Value::X, ov) => Value::XOp(Box::new(x.clone()), op.clone(), Box::new(ov.clone())),
                        (ov, x@Value::X) => Value::XOp(Box::new(ov.clone()), op.clone(), Box::new(x.clone())),
                        (xop1, xop2) => Value::XOp(Box::new(xop1.clone()), op.clone(), Box::new(xop2.clone())),
                    };
                    value_cache.insert(last, result);
                } else {
                    if !value_cache.contains_key(id1) {
                        eval_stack.push(id1)
                    }
                    if !value_cache.contains_key(id2) {
                        eval_stack.push(id2)
                    }
                }

            }
        }

        let v1 = value_cache.remove(root_id1).unwrap();
        let v2 = value_cache.remove(root_id2).unwrap();

        (v1, v2)
    }

}

pub fn solve(v1: &Value, v2: i64) -> i64 {
    
    let mut cur_v1 = v1.clone();
    let mut cur_v2 = v2.clone();
    
    while let Value::XOp(op1, op, op2) = cur_v1 {
        let inv_op = op.invert();
        if let Value::Value(v) = *op1 {
            cur_v1 = *op2;
            cur_v2 = inv_op.eval(cur_v2, v);
        } else if let Value::Value(v) = *op2 {
            cur_v1 = *op1;
            cur_v2 = inv_op.eval(cur_v2, v);
        } else {
            panic!();
        }
    }
    
    cur_v2
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = include_str!("../res/day21-shout_example.txt");

    #[test]
    fn test_parse() {
        let defs: MonkeyDefinitions = EXAMPLE.parse().unwrap();
        println!("{:#?}", defs);
    }

    #[test]
    fn test_eval() {
        let defs: MonkeyDefinitions = EXAMPLE.parse().unwrap();
        let code = defs.eval();
        // println!("{}", code);
        assert_eq!(152, code);
    }

    #[test]
    fn test_eval_2() {
        let defs: MonkeyDefinitions = EXAMPLE.parse().unwrap();
        let (lhs, rhs) = defs.human_eval();
        println!("{}", lhs);
        println!("{}", rhs);
        if let Value::Value(v) = rhs {
            println!("solved: {}", solve(&lhs, v));
        }
    }
}
