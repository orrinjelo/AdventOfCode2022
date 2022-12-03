use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

pub fn rps_naive(input: Vec<String>) -> Result<u32, &'static str> {
    let mut score = 0;
    for line in input {
        let choice = match line.as_bytes()[0] as char {
            'A' => {
                score += 1;
                Ok('r')
            },
            'B' => {
                score += 2;
                Ok('p')
            },
            'C' => {
                score += 3;
                Ok('s')
            },
            _ => {Err("Bad player pick.")},
        };
        let elf_choice = match line.as_bytes()[2] as char {
            'X' => {
                Ok('r')
            },
            'Y' => {
                Ok('p')
            },
            'Z' => {
                Ok('s')
            },
            _ => {Err("Bad elf pick.")},
        };
        if choice == elf_choice {
            score += 3;
            println!("You tied.");
        } else {
            match choice {
                Ok('r') => {
                    if elf_choice == Ok('s') {
                        score += 6;
                        println!("You won, r vs s.");
                    }
                },
                Ok('p') => {
                    if elf_choice == Ok('r') {
                        score += 6;
                        println!("You won, p vs r.");
                    }
                },
                Ok('s') => {
                    if elf_choice == Ok('p') {
                        score += 6;
                        println!("You won, s vs p.");
                    }
                },
                Err(x) => return Err(x),
                _ => return Err("Unknown error."),
            };
        }
    }
    return Ok(score);
}

/**
 *  Problem #02, part 1
 *  Rock, paper, scissors?
 */
pub fn problem_021(input: Vec<String>) -> RetType {

    return RetType::U32(0);
}

/**
 *  Problem #02, part 2
 *  Rock, paper, scissors?
 */
pub fn problem_022(input: Vec<String>) -> RetType {

    return RetType::U32(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        match env_logger::try_init() {
            Ok(_) => {
                info!("Initializing logging...");
            },
            Err(_) => {

            }
        }
    }

    #[test]
    fn test_rps_naive() {
        let input = vec![
            "A Y".to_string(),
            "B X".to_string(),
            "C Z".to_string(),
        ];
        let res = rps_naive(input);

        println!("{:?}", res);

        assert!(res.unwrap() == 15);
    }
}