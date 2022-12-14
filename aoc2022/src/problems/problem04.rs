use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use std::cmp::{min, max};

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

fn parse_line(line: String) -> ((u32, u32), (u32, u32)) {
    let parts: Vec<&str> = (*line).split(",").collect::<Vec<&str>>();
    let range_a = parts[0].split("-").collect::<Vec<&str>>();
    let range_b = parts[1].split("-").collect::<Vec<&str>>();
    ((range_a[0].parse::<u32>().unwrap(), range_a[1].parse::<u32>().unwrap()), (range_b[0].parse::<u32>().unwrap(), range_b[1].parse::<u32>().unwrap()))
}

// fn is_overlapping(ranges: ((u32, u32), (u32, u32))) -> bool {
//     max(ranges.0.1, ranges.1.1) - min(ranges.0.0, ranges.1.0) < (ranges.0.1 - ranges.0.0) + (ranges.1.1 - ranges.1.0)
// }

fn get_overlap(ranges: ((u32, u32), (u32, u32))) -> u32 {
    max(0, min(ranges.0.1 as i32, ranges.1.1 as i32) - max(ranges.0.0 as i32, ranges.1.0 as i32) + 1) as u32
}

/**
 *  Problem #04, part 1
 */
pub fn problem_041(input: Vec<String>) -> RetType {
    let mut sum = 0;
    for line in input {
        let ((a1, a2), (b1, b2)) = parse_line(line);
        if (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2) {
            sum += 1;
        }
    }
    return RetType::U32(sum);
}

/**
 *  Problem #04, part 2
 */
pub fn problem_042(input: Vec<String>) -> RetType {
    let mut sum = 0;
    for line in input {
        let ((a1, a2), (b1, b2)) = parse_line(line);
        if get_overlap(((a1, a2), (b1, b2))) > 0 {
            sum += 1;
        }
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
    fn test_parse_line() {
        init();
        let input = "2-4,6-8".to_string();
        assert!(parse_line(input) == ((2, 4), (6, 8)));
    }

    #[test]
    fn test_part1() {
        init();
        let input = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        let res = problem_041(input);
        assert!(res == RetType::U32(2));
    }

    /**
        5-7,7-9 overlaps in a single section, 7.
        2-8,3-7 overlaps all of the sections 3 through 7.
        6-6,4-6 overlaps in a single section, 6.
        2-6,4-8 overlaps in sections 4, 5, and 6.
     */
    #[test]
    fn test_overlap() {
        init();

        // assert!(is_overlapping(((5,7),(7,9))));
        assert!(get_overlap(((5,7),(7,9))) == 1);
        assert!(get_overlap(((2,8),(3,7))) == 5);
        assert!(get_overlap(((6,6),(4,6))) == 1);
        assert!(get_overlap(((2,6),(4,8))) == 3);
    }
}