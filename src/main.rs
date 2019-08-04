/**
 * author Franklin Morales (Aragorn)
 * July 2019
 */
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod rpn;
use rpn::*;

const EXTENSION: &'static str = "ope";

fn get_ope<'a>(path: &'a str) -> Result<Vec<String>, String> {
    let mut list: Vec<String> = Vec::new();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(format!("err: no fund file")),
    };
    for line in BufReader::new(file).lines() {
        list.push(line.unwrap().clone());
    }

    Ok(list)
}

/// Obtiene el path del fichero a abrir verificando si
/// existe.
fn init() -> Result<bool, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(format!("err: incorrect number params"));
    }

    let dir = args[1].clone();
    let path = Path::new(&dir);
    match path.extension() {
        Some(ex) => {
            if ex != EXTENSION {
                return Err(format!("err: extension name: {:?} is incorrect", ex));
            }

            let list: Vec<String> = match get_ope(&dir) {
                Ok(list) => list,
                Err(e) => return Err(e),
            };

            let rpn_array: Vec<RPN> = get_rpn(&list)?;

            assert_eq!(rpn_array[0].output, "62+5*84/-");
            assert_eq!(rpn_array[1].output, "62+5*8*4-");
            assert_eq!(rpn_array[2].output, "59+2*65*+");
            assert_eq!(rpn_array[3].output, "854*+126/-");
            
            for rpn in rpn_array.iter() {
                println!("{:?}", rpn);
            }

            Ok(true)
        }
        None => Err(format!("err: most exist extension")),
    }
}

fn main() {
    match init() {
        Ok(_) => println!("compilation success!"),
        Err(e) => println!("{}", e),
    }
}
