use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
// use std::str;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}


fn unrepeated_idx(packets: &String) -> u32 {
    for i in 0..packets.len()-4 {
        let substr = packets.as_bytes();
        if substr[i+1..i+4].contains(&substr[i]) || substr[i+2..i+4].contains(&substr[i+1]) || substr[i+2] == substr[i+3] {
            continue;
        }
        // debug!("{}", i+4);

        return (i+4) as u32
    }
    return 0
}


fn unrepeated_idx_msg(packets: &String, length: usize) -> u32 {
    for i in 0..packets.len()-length {
        let mut dupe: bool = false;
        let substr = packets.as_bytes();

        // debug!("#1/{}: {:?}", i, str::from_utf8(&substr[i..i+length]));

        for j in 0..length-1 {
            // debug!("#1.1/{}: {:?} {:?}", i, str::from_utf8(&substr[i+j+1..i+length]), str::from_utf8(&substr[i+j..i+j+1]));
            if substr[i+j+1..i+length].contains(&substr[i+j]) {
                dupe = true;
                break;
            }
        }

        if dupe {
            continue;
        }

        // debug!("#2/{}: {:?}", i, dupe);

        if !dupe && substr[i+length-1] == substr[i+length-2] {
            dupe = true;
        }

        if dupe {
            continue;
        }

        // debug!("#3/{}: {:?}", i, dupe);

        return (i+length) as u32
    }
    return 0
}

/**
 *  Problem #06, part 1
 */
pub fn problem_061(input: Vec<String>) -> RetType {
    return RetType::U32(unrepeated_idx(&input[0]));
}

/**
 *  Problem #06, part 2
 */
pub fn problem_062(input: Vec<String>) -> RetType {
    return RetType::U32(unrepeated_idx_msg(&input[0], 14));
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

    /**
        bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
        nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
        nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
        zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
     */
    #[test]
    fn test_find_idx() {
        init();

        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();

        let res = unrepeated_idx(&input);

        // debug!("{}", res);

        assert!(res == 7);

        let input1 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let input2 = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let input3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let input4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();

        assert!(unrepeated_idx(&input1) == 5);
        assert!(unrepeated_idx(&input2) == 6);
        assert!(unrepeated_idx(&input3) == 10);
        assert!(unrepeated_idx(&input4) == 11);

    }

    #[test]
    fn test_find_idx2() {
        init();

        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();

        let res = unrepeated_idx_msg(&input, 4);

        // debug!("{}", res);

        assert!(res == 7);

        let input1 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let input2 = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let input3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let input4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();

        assert!(unrepeated_idx_msg(&input1, 4) == 5);
        assert!(unrepeated_idx_msg(&input2, 4) == 6);
        assert!(unrepeated_idx_msg(&input3, 4) == 10);
        assert!(unrepeated_idx_msg(&input4, 4) == 11);


        // mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
        // bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
        // nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
        // nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
        // zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26


        let input5 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let input6 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let input7 = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let input8 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let input9 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();

        assert!(unrepeated_idx_msg(&input5, 14) == 19);
        assert!(unrepeated_idx_msg(&input6, 14) == 23);
        assert!(unrepeated_idx_msg(&input7, 14) == 23);
        assert!(unrepeated_idx_msg(&input8, 14) == 29);
        assert!(unrepeated_idx_msg(&input9, 14) == 26);

    }
}