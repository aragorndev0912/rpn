use std::collections::BinaryHeap;
use std::cmp::*;

#[derive(Debug, Eq)]
pub struct Node {
    v: u32,
    s: u8
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.v.cmp(&other.v)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

#[derive(Debug)]
pub struct RPN {
    pub input: String,
    pub output: String,
    heap: BinaryHeap<Node>
}

impl RPN {
    pub fn new(operation:& String) -> RPN {
        let input = operation.clone();
        let output = String::new();

        RPN {
            input: input,
            output: output,
            heap: BinaryHeap::new()
        }
    }    

    #[allow(dead_code)]
    fn get_posfix<'a, 'b>(&self, _input: &'a str) -> &'b str {
        ""
    }
}

pub fn get_rpn(ope_array:& Vec<String>) -> Vec<RPN> {
    let mut rpn_array:Vec<RPN> = Vec::new();

    for ope in ope_array {
        rpn_array.push(RPN::new(& ope));
    }

    rpn_array
}