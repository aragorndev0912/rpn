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

            assert_eq!(rpn_array[0].output, "6 2 5 *+8 4/-");
            assert_eq!(rpn_array[1].output, "6 2+ 5 *8 4/-");
            assert_eq!(rpn_array[2].output, "6 2+ 5* 8 *4-");
            assert_eq!(rpn_array[3].output, "5 9+ 2* 6 5*+");
            assert_eq!(rpn_array[4].output, "8 5 4 *+12 68/-");
            
            assert_eq!(rpn_array[0].calculate(), Ok(14));
            assert_eq!(rpn_array[1].calculate(), Ok(38));
            assert_eq!(rpn_array[2].calculate(), Ok(316));
            assert_eq!(rpn_array[3].calculate(), Ok(58));
            assert_eq!(rpn_array[4].calculate(), Ok(28));
            

            for rpn in rpn_array.iter() {
                println!("{:?}, {:?}", rpn, rpn.calculate());
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
