use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use regex::Regex;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}


fn parse_crates(diagram: Vec<&String>) -> Vec<Vec<char>> {
    let mut crates: Vec<Vec<char>> = Vec::new();

    // Determine how many columns we have
    let cols = (*diagram[diagram.len()-1]).trim().split(' ').next_back();
    match cols {
        Some(x) => {
            let ncols = x.parse::<usize>().unwrap();
            for _i in 0..ncols {
                crates.push(Vec::new());
            }
            for l in 1..diagram.len() {
                let y = diagram.len() - l - 1;
                for x in 0..ncols {
                    if diagram[y].len() > x*4+1 {
                        let c = diagram[y].as_bytes()[x*4+1] as char;
                        if c != ' ' {
                            crates[x].push(c);
                        }
                    }
                }
            }
        },
        None => {},
    };

    crates
}

fn parse_command(command: &String, crates: &mut Vec<Vec<char>>) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let cap = re.captures(&command).unwrap();
    let num = cap.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
    let src = cap.get(2).map_or("", |m| m.as_str()).parse::<usize>().unwrap() - 1;
    let dst = cap.get(3).map_or("", |m| m.as_str()).parse::<usize>().unwrap() - 1;

    for _i in 0..num {
        match crates[src].pop() {
            Some(x) => {
                crates[dst].push(x);
            },
            None => {
                error!("Stack {} is empty!", src);
            }
        };
    }

    // debug!("{:?}", cap);
}

fn parse_command_9001(command: &String, crates: &mut Vec<Vec<char>>) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let cap = re.captures(&command).unwrap();
    let num = cap.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
    let src = cap.get(2).map_or("", |m| m.as_str()).parse::<usize>().unwrap() - 1;
    let dst = cap.get(3).map_or("", |m| m.as_str()).parse::<usize>().unwrap() - 1;

    let mut lazy_stack: Vec<char> = Vec::new();

    for _i in 0..num {
        match crates[src].pop() {
            Some(x) => {
                lazy_stack.push(x);
            },
            None => {
                error!("Stack {} is empty!", src);
            }
        };
    }
    for _i in 0..num {
        match lazy_stack.pop() {
            Some(x) => {
                crates[dst].push(x);
            },
            None => {
                error!("Stack {} is empty!", src);
            }
        };
    }


    // debug!("{:?}", cap);
}


/**
 *  Problem #05, part 1
 */
pub fn problem_051(input: Vec<String>) -> RetType {
    let mut crates_vec: Vec<&String> = Vec::new();
    let mut i: usize = 0;
    loop {
        if input[i].trim() == "".to_string() {
            break;
        } else {
            crates_vec.push(&input[i]);
        }

        i += 1;
    }
    let mut diagram = parse_crates(crates_vec);

    for j in i+1..input.len() {
        parse_command(&input[j], &mut diagram);
    }

    // debug!("{:?}", diagram);

    let mut answer: String = String::new();

    for stack in diagram {
        answer.push(
            match stack.last() {
                Some(x) => {*x},
                None => {'?'},
            }
        );
    }

    return RetType::STRING(answer);
}

/**
 *  Problem #05, part 2
 */
pub fn problem_052(input: Vec<String>) -> RetType {
    let mut crates_vec: Vec<&String> = Vec::new();
    let mut i: usize = 0;
    loop {
        if input[i].trim() == "".to_string() {
            break;
        } else {
            crates_vec.push(&input[i]);
        }

        i += 1;
    }
    let mut diagram = parse_crates(crates_vec);

    for j in i+1..input.len() {
        parse_command_9001(&input[j], &mut diagram);
    }

    // debug!("{:?}", diagram);

    let mut answer: String = String::new();

    for stack in diagram {
        answer.push(
            match stack.last() {
                Some(x) => {*x},
                None => {'?'},
            }
        );
    }

    return RetType::STRING(answer);
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
    fn test_parse_crates() {
        init();
        let input_pre = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3".to_string(),
        ];

        let input = vec![&input_pre[0], &input_pre[1], &input_pre[2], &input_pre[3]];

        let res = parse_crates(input);

        // debug!("{:?}", res);

        assert!(res == vec![vec!['Z','N'], vec!['M', 'C', 'D'], vec!['P']]);
    }


    #[test]
    fn test_parse_command() {
        init();
        let mut crates = vec![vec!['Z','N'], vec!['M', 'C', 'D'], vec!['P']];
        let command = "move 1 from 2 to 1".to_string();

        parse_command(&command, &mut crates);

        debug!("{:?}", crates);

        assert!(crates == vec![vec!['Z','N','D'], vec!['M', 'C'], vec!['P']])
    }

    #[test]
    fn test_part1() {
        init();

        let input = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];

        let res = problem_051(input);
        debug!("{}", res);

        assert!(res == RetType::STRING("CMZ".to_string()));
    }
}