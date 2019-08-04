use std::cmp::*;
use std::collections::HashMap;

#[derive(Debug, Eq)]
#[allow(dead_code)]
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
    pub output: String
}

impl RPN {
    pub fn new(operation:& String) -> Result<RPN, String> {
        let mut input:String = operation.clone();
        let output = get_posfix(&mut input)?;
        
        Ok(RPN {
            input: input,
            output: output
        })
    }    
}

/// Permite comparar operadores validos.
/// 
/// La función evalua dos operatores validos `(+, -, *, /)` c0 y c1
/// 
/// si c0 > c1 retorna Some(1) 
/// 
/// si c0 < c1 retorna Some(-1)
/// 
/// si c0 == c1 retorna Some(0)
/// 
/// # Example
/// ```
/// cmp_operator('+', '+'); // return Some(0)
/// cmp_operator('+', '*'); // return Some(1)
/// cmp_operator('*', '+'); // return Some(-1)
/// ```

fn cmp_operator(c0:& char, c1:& char) -> Result<i8, String> {
    if !"+-*/".contains(*c0) && !"+-*/".contains(*c1) {
        return Err(format!("err: cmp_operator c0->'{}' c1->'{}'", *c0, *c1));
    }
    
    let mut map:HashMap<char, u8> = HashMap::new();
    map.insert('+', 0);
    map.insert('-', 0);
    map.insert('*', 1);
    map.insert('/', 1);

    let v0:u8 = *map.get(c0).unwrap();
    let v1:u8 = *map.get(c1).unwrap();

    if v0 == v1 {
        return Ok(0);
    }
    else if v0 > v1 {
        return Ok(1);
    }

    Ok(-1)
}

fn rigth_paren_eval(heap:&mut Vec<char>, output:&mut String) -> Result<bool, String> {
    if heap.len() > 0 {
        match heap.pop() {
            Some(value) => {
                if "+-*/".contains(value) {
                    output.push(value);
                    rigth_paren_eval(heap, output)?;
                }
            },
            None => return Err(format!("err: rigth_paren_eval"))
        }
    }

    Ok(true)
}

fn count_operator(heap:& Vec<char>) -> usize {
    let mut count:usize = 0;
    
    for c in heap.iter() {
        count += 1;        
        if *c == '(' {
            count = 0;
        }
    }

    println!("heap: {:?}, count: {}", heap, count);
    (count)
}

fn evaluate_operator(heap:&mut Vec<char>, output:&mut String, c:& char, flag:bool) -> Result<bool, String> {
    if heap.len() > 0 {
        match heap.pop() {
            Some(value) => {
                if "+-*/".contains(value) && cmp_operator(c, &value)? <= 0 {
                    output.push(value);
                    evaluate_operator(heap, output, c, true)?;
                }
                else if "+-*/".contains(value) && cmp_operator(c, &value)? > 0 {
                    heap.push(value);
                    heap.push(*c);
                }
                else {
                    heap.push(value);
                    if flag {
                        heap.push(*c);
                    }
                }
            },
            None => return Err(format!("err: evaluate_operator None "))
        }
    }
    
    Ok(true)
}

#[allow(dead_code)]
fn evaluate_char(heap:&mut Vec<char>, output:&mut String, c:& char) -> Result<bool, String>{
    if *c as usize >= 48 && *c as usize <= 57 {
        output.push(*c);
        return Ok(true);        
    }
    else if *c == '(' {
        heap.push(*c);
        return Ok(true);    
    }
    else if *c == ')' {
        return rigth_paren_eval(heap, output);
    }
    else if "+-*/".contains(*c) {
        if count_operator(&heap) == 0 {
            println!("agregando...");
            heap.push(*c);
            return Ok(true);
        }
        else {
            return evaluate_operator(heap, output, c, false);
        }
    }
    
    Err(format!("err: character inavalid '{}'", *c))
}

#[allow(dead_code)]
fn get_posfix(input: &mut String) -> Result<String, String> {
    input.retain(|c| c != ' ');
    input.push(')');
    let mut output = String::new();
    let mut heap:Vec<char> = Vec::new();
    heap.push('(');

    for c in input.chars() {
        evaluate_char(&mut heap, &mut output, &c)?;
    }

    Ok(output)
}

pub fn get_rpn(ope_array:& Vec<String>) -> Result<Vec<RPN>, String> {
    let mut rpn_array:Vec<RPN> = Vec::new();

    for ope in ope_array {
        rpn_array.push(RPN::new(& ope)?);
    }

    Ok(rpn_array)
}