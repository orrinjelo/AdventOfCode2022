// use itertools::Itertools;
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

// fn naive_best_elf(input_vector: Vec<u32>) -> Result<(u32, u32), &'static str> {    
// }

/**
 *  Problem #01, part 1
 *  Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
 */
pub fn problem_011(input: Vec<String>) -> RetType {
    let mut best_val = 0;
    let mut current_sum = 0;

    for line in input.iter() {
        if line.trim().is_empty() {
            if current_sum > best_val {
                best_val = current_sum;
            }
            current_sum = 0;
        } else {
            current_sum += line.parse::<u32>().unwrap();
        }
    }

    return RetType::U32(best_val);
}


/**
 *  Problem #01, part 2
 */
pub fn problem_012(input: Vec<String>) -> RetType {
    let mut calories: Vec<u32> = Vec::new();
    let mut current_sum = 0u32;

    for line in input.iter() {
        if line.trim().is_empty() {
            calories.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += line.parse::<u32>().unwrap();
        }
    }

    if current_sum != 0 {
        calories.push(current_sum);
    }

    calories.sort();
    calories.reverse();

    return RetType::U32(calories[0] + calories[1] + calories[2]);
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
    fn test_problem_011() {
        init();
        let input_vec = vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
        ];
        let result = problem_011(input_vec);
        match result {
            RetType::U32(24000) => {
                assert!(true);
            },
            RetType::U32(_) => {
                error!("Wrong result: {}", result);
                assert!(false);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_problem_012() {
        init();
        let input_vec = vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
        ];
        let result = problem_012(input_vec);
        match result {
            RetType::U32(45000) => {
                assert!(true);
            },
            RetType::U32(_) => {
                error!("Wrong result: {}", result);
                assert!(false);
            },
            _ => assert!(false),
        }
    }
}