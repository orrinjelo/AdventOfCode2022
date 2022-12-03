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

pub fn rps_naive(input: Vec<String>) -> u32 {
    let mut score = 0;
    for line in input {
        let choice = match line.as_bytes()[0] as char {
            'A' => {
                score += 1;
                'r'
            },
            'B' => {
                score += 2;
                'p'
            },
            'C' => {
                score += 3;
                's'
            },
            _ => {'x'},
        };
        let elf_choice = match line.as_bytes()[2] as char {
            'X' => {
                score += 1;
                'r'
            },
            'Y' => {
                score += 2;
                'p'
            },
            'Z' => {
                score += 3;
                's'
            },
            _ => {'x'},
        };
        if choice == elf_choice {
            score += 3;
        } else {
            match choice {
                'r' => {
                    if elf_choice == 's' {
                        score += 6;
                    }
                },
                'p' => {
                    if elf_choice == 'r' {
                        score += 6;
                    }
                },
                's' => {
                    if elf_choice == 'p' {
                        score += 6;
                    }
                },
                _ => {},
            };
        }
    }
    return score;
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
        rps_naive(input);
    }
}