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

pub fn rps_naive(input: Vec<String>) -> Result<u64, &'static str> {
    let mut score: u64 = 0;
    for line in input {
        let choice = match line.as_bytes()[0] as char {
            'A' => {
                // score += 1;
                Ok('r')
            },
            'B' => {
                // score += 2;
                Ok('p')
            },
            'C' => {
                // score += 3;
                Ok('s')
            },
            _ => {Err("Bad player pick.")},
        };
        let elf_choice = match line.as_bytes()[2] as char {
            'X' => {
                score += 1;
                Ok('r')
            },
            'Y' => {
                score += 2;
                Ok('p')
            },
            'Z' => {
                score += 3;
                Ok('s')
            },
            _ => {Err("Bad elf pick.")},
        };
        if choice == elf_choice {
            score += 3;
            trace!("You tied.");
        } else {
            match elf_choice {
                Ok('r') => {
                    if choice == Ok('s') {
                        score += 6;
                        trace!("You won, r vs s.");
                    } else {
                        trace!("You lose.");
                    }
                },
                Ok('p') => {
                    if choice == Ok('r') {
                        score += 6;
                        trace!("You won, p vs r.");
                    } else {
                        trace!("You lose.");
                    }
                },
                Ok('s') => {
                    if choice == Ok('p') {
                        score += 6;
                        trace!("You won, s vs p.");
                    } else {
                        trace!("You lose.");
                    }
                },
                Err(x) => return Err(x),
                _ => return Err("Unknown error."),
            };
        }
    }
    return Ok(score);
}

pub fn rps_elf(input: Vec<String>) -> u64 {
let mut score: u64 = 0;
    for line in input {
        score += match line.as_bytes()[0] as char {
            'A' => { // Elf picks rock
                match line.as_bytes()[2] as char {
                    'X' => 3, // We need to lose, pick scissor
                    'Y' => 4, // We need to tie, pick rock
                    'Z' => 8, // we need to win, pick paper
                    _ => 0,
                }

            },
            'B' => { // Elf picks paper
                match line.as_bytes()[2] as char {
                    'X' => 1, // We need to lose, pick rock
                    'Y' => 5, // We need to tie, pick paper
                    'Z' => 9, // we need to win, pick scissor
                    _ => 0,
                }

            },
            'C' => { // Elf picks scissor
                match line.as_bytes()[2] as char {
                    'X' => 2, // We need to lose, pick paper
                    'Y' => 6, // We need to tie, pick scissor
                    'Z' => 7, // we need to win, pick rock
                    _ => 0,
                }
            },
            _ => 0,
        };
    }
    return score
}

/**
 *  Problem #02, part 1
 *  Rock, paper, scissors?
 */
pub fn problem_021(input: Vec<String>) -> RetType {
    return match rps_naive(input) {
        Ok(x) => RetType::U64(x),
        Err(x) => {
            error!("{}", x);
            RetType::U64(0)
        },
    };
}

/**
 *  Problem #02, part 2
 *  Rock, paper, scissors?
 */
pub fn problem_022(input: Vec<String>) -> RetType {

    return RetType::U64(rps_elf(input));
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
        init();
        let input = vec![
            "A Y".to_string(),
            "B X".to_string(),
            "C Z".to_string(),
        ];
        let res = rps_naive(input);

        info!("{:?}", res);

        assert!(res.unwrap() == 15);
    }

    #[test]
    fn test_rps_naive2() {
        init();
        let input = vec![
            "C Y".to_string(),
            "C Y".to_string(),
            "C Y".to_string(),
        ];
        let res = rps_naive(input);

        info!("{:?}", res);

        assert!(res.unwrap() == 6);
    }

    #[test]
    fn test_rps_naive3() {
        init();
        let input = vec![
            "A Y".to_string(),
            "A Y".to_string(),
            "A Y".to_string(),
        ];
        let res = rps_naive(input);

        info!("{:?}", res);

        assert!(res.unwrap() == 24);
    }

    #[test]
    fn test_rps_naive4() {
        init();
        let input = vec![
            "A X".to_string(); 25000
        ];
        let res = rps_naive(input);

        info!("{:?}", res);

        assert!(res.unwrap() == 100000);
    }

    #[test]
    fn test_rps_naive5() {
        init();
        let input = vec![
            "C Z".to_string(), // tie, 6
            "C Z".to_string(), // tie, 6
            "C Z".to_string(), // tie, 6
            "B Z".to_string(), // win, 9
            "C Z".to_string(), // tie, 6
            "A Y".to_string(), // win, 8
        ];
        let res = rps_naive(input);

        info!("{:?}", res);

        assert!(res.unwrap() == 41);
    }

    #[test]
    fn test_elf_rps1() {
        init();
        let input = vec![
            "A Y".to_string(),
            "B X".to_string(),
            "C Z".to_string(),
        ];
        let res = rps_elf(input);

        info!("{:?}", res);

        assert!(res == 12);
    }

    // #[test]
    // fn test_all_combos() {
    //     init();

    //     // Rock, tie
    //     let input1 = vec!["A X".to_string()];
    //     let res1 = rps_naive(input1);
    //     assert!(res1.unwrap() == 4);

    //     // Rock, loss
    //     let input2 = vec!["A Y".to_string()];
    //     let res2 = rps_naive(input2);
    //     assert!(res2.unwrap() == 1);

    //     // Rock, win
    //     let input3 = vec!["A Z".to_string()];
    //     let res3 = rps_naive(input3);
    //     assert!(res3.unwrap() == 7);

    //     // Paper, win
    //     let input4 = vec!["B X".to_string()];
    //     let res4 = rps_naive(input4);
    //     assert!(res4.unwrap() == 8);

    //     // Paper, tie
    //     let input5 = vec!["B Y".to_string()];
    //     let res5 = rps_naive(input5);
    //     assert!(res5.unwrap() == 5);

    //     // Paper, loss
    //     let input6 = vec!["B Z".to_string()];
    //     let res6 = rps_naive(input6);
    //     assert!(res6.unwrap() == 2);

    //     // Scissors, loss
    //     let input7 = vec!["C X".to_string()];
    //     let res7 = rps_naive(input7);
    //     assert!(res7.unwrap() == 3);

    //     // Scissors, win
    //     let input8 = vec!["C Y".to_string()];
    //     let res8 = rps_naive(input8);
    //     assert!(res8.unwrap() == 9);

    //     // Scissors, tie
    //     let input9 = vec!["C Z".to_string()];
    //     let res9 = rps_naive(input9);
    //     assert!(res9.unwrap() == 6);
    // }
}