use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use std::collections::HashSet;
use crate::util::RetType;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

fn common_char(a: &str, b: &str) -> char {
    let set: HashSet<char> = a.chars().collect();

    let mut ret_char = '0';

    b.chars().any(|c| {
        if set.contains(&c) {
            ret_char = c;
            true
        } else {
            false
        }
    });

    ret_char
}

fn common_char3(a: &str, b: &str, c: &str) -> char {
    let seta: HashSet<char> = a.chars().collect();
    let setb: HashSet<char> = b.chars().collect();

    let mut ret_char = '0';

    c.chars().any(|x| {
        if seta.contains(&x) && setb.contains(&x) {
            ret_char = x;
            true
        } else {
            false
        }
    });

    ret_char
}

fn decode_char(a: char) -> u32 {
     let uchar = a as u32;
     if uchar > 96 { // lower case
        uchar - 96
     } else {
        uchar - 38
     }
}

fn split_string(a: &str) -> (&str, &str) {
    a.split_at(a.len() / 2)
}

/**
 *  Problem #03, part 1
 */
pub fn problem_031(input: Vec<String>) -> RetType {
    let mut sum = 0;
    for line in input {
        // let s_slice: &str = &*line;
        let (a, b) = split_string(&*line);
        let c = common_char(a, b);
        sum += decode_char(c);
    }
    return RetType::U32(sum);
}

/**
 *  Problem #03, part 2
 */
pub fn problem_032(input: Vec<String>) -> RetType {
    let mut sum = 0;
    for l in 0..input.len()/3 {
        // let s_slice: &str = &*line;
        let c = common_char3(&*input[3*l], &*input[3*l+1], &*input[3*l+2]);
        sum += decode_char(c);
    }
    return RetType::U32(sum);
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
    fn test_common_char() {
        init();
        let input_a = "vJrwpWtwJgWr";
        let input_b = "hcsFMMfFFhFp";
        let input_c = "jqHRNqRjqzjGDLGL";
        let input_d = "rsFMfFZSrLrFZsSL";
        let input_e = "PmmdzqPrV";
        let input_f = "vPwwTWBwg";
        let input_g = "wMqvLMZHhHMvwLH";
        let input_h = "jbvcjnnSBnvTQFn";
        let input_i = "ttgJtRGJ";
        let input_j = "QctTZtZT";
        let input_k = "CrZsJsPPZsGz";
        let input_l = "wwsLwLmpwMDw";

        let res1 = common_char(input_a, input_b);
        let res2 = common_char(input_c, input_d);
        let res3 = common_char(input_e, input_f);
        let res4 = common_char(input_g, input_h);
        let res5 = common_char(input_i, input_j);
        let res6 = common_char(input_k, input_l);

        // info!("{:?}", res);

        assert!(res1 == 'p');
        assert!(res2 == 'L');
        assert!(res3 == 'P');
        assert!(res4 == 'v');
        assert!(res5 == 't');
        assert!(res6 == 's');
    }

    #[test]
    fn test_decode() {
        assert!('a' as u32 - 96 == 1);

        assert!(decode_char('a') == 1);
        assert!(decode_char('A') == 27);
    }

    #[test]
    fn test_split_half() {
        assert!(split_string("vJrwpWtwJgWrhcsFMMfFFhFp") == ("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
    }

    #[test]
    fn test_common_char3() {
        assert!(common_char3("vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg") == 'r');
    }
}