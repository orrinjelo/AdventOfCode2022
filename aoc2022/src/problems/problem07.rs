use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use regex::Regex;
use std::fmt;
// use std::rc::Rc;
// use std::cell::RefCell;
use trees::{Tree, Node};
use trees::rust::{Pin};

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
struct File {
    name: String,
    file_size: usize,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
struct Directory {
    name: String,
    contents: Vec<File>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum CommandType {
    CD(String),
    LS,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum FileType {
    DIRECTORY(Directory),
    FILE(File),
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum ConsoleInput {
    COMMAND(CommandType),
    FILE(FileType),
}

impl fmt::Debug for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::DIRECTORY(x) => write!(f, "{:?}", x),
            FileType::FILE(x) => write!(f, "{:?}", x),
        }
    }
}

impl fmt::Debug for ConsoleInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsoleInput::COMMAND(x) => write!(f, "{:?}", x),
            ConsoleInput::FILE(x) => write!(f, "{:?}", x),
        }
    }
}

fn parse_line(line: &String) -> Result<ConsoleInput, String> {
    if line.as_bytes()[0] as char == '$' {
        if line.contains("ls") {
            return Ok(ConsoleInput::COMMAND(CommandType::LS));
        } else if line.contains("cd") {
            let re = Regex::new(r"\$ cd (.*)").unwrap();
            let cap = re.captures(&line).unwrap();
            // debug!("Dir: {:?}", cap.get(1));
            return Ok(ConsoleInput::COMMAND(CommandType::CD(cap.get(1).unwrap().as_str().to_string())));
        } else {
            return Err("Unknown command.".to_string());
        }
    } else if line.contains("dir") {
        let re = Regex::new(r"dir (.*)").unwrap();
        let cap = re.captures(&line).unwrap();
        // debug!("[{:?}]", cap.get(1));
        return Ok(ConsoleInput::FILE(FileType::DIRECTORY(
            Directory {
                name: cap.get(1).unwrap().as_str().to_string(),
                contents: Vec::new()
            }
        )));
    } else {
        let re = Regex::new(r"(\d) (.*)").unwrap();
        let cap = re.captures(&line).unwrap();
        debug!("% {} ({})", cap.get(2).unwrap().as_str().to_string(), cap.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap());
        return Ok(ConsoleInput::FILE(FileType::FILE(
            File {
                name: cap.get(2).unwrap().as_str().to_string(),
                file_size: cap.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap(),
            }
        )));
    }
    // return Err("Nope.".to_string());
}

// fn construct_tree(input: &Vec<String>) -> Tree<Directory> {
//     let mut root: Option<Tree<Directory>> = None;
//     let mut pwd: Option<&mut Directory> = None;
//     for line in input {
//         match parse_line(&line) {
//             Ok(x) => {
//                 match x {
//                     ConsoleInput::COMMAND(y) => {
//                         match y {
//                             CommandType::CD(z) => {
//                                 if z == "/".to_string() {
//                                     root = Some(Tree::new(
//                                         Directory {
//                                             name: z,
//                                             contents: Vec::new(),
//                                         }
//                                     ));
//                                     pwd = Some(root.unwrap().root_mut());
//                                     continue;
//                                 }

//                                 if z == "..".to_string() {
//                                     if pwd == Some(root.unwrap().root_mut()) || pwd == None {
//                                         error!("Invalid directory.");
//                                     } else {
//                                         pwd = Some(pwd.unwrap().parent().unwrap());
//                                     }
//                                 } else {
//                                     let mut not_found = true;
//                                     for child in pwd.unwrap().into_inner().iter() {
//                                         if child.data().name == z {
//                                             pwd = Some(&mut child);
//                                             not_found = false;
//                                         }
//                                     }
//                                     if not_found {
//                                         error!("Directory not found.");
//                                     }
//                                 }
//                             },
//                             CommandType::LS => {
//                                 // Kind of a NOOP in this case, I guess?
//                             },
//                             // _ => {},

//                         }
//                     },
//                     ConsoleInput::FILE(y) => {
//                         match y {
//                             FileType::DIRECTORY(d) => {
//                                 // pwd.unwrap().push_back(Tree::new(
//                                 // ));
//                                 let x = Directory {
//                                     name: "foobar".to_string(),
//                                     contents: Vec::new(),
//                                 };

//                                 let xx = Tree::new(x);

//                                 let node = pwd.unwrap();
//                                 node.push_back(xx);
//                             },
//                             _ => {},
//                         }
//                     },
//                 }
//             },
//             Err(x) => {
//                 error!("{}", x);
//             }
//         }
//     }
//     return root.unwrap();
// }

/**
 *  Problem #07, part 1
 */
pub fn problem_071(input: Vec<String>) -> RetType {
    return RetType::U32(0);
}

/**
 *  Problem #07, part 2
 */
pub fn problem_072(input: Vec<String>) -> RetType {
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
    fn test_parse_line() {
        init();

        assert!(parse_line(&"$ ls".to_string()) == Ok(ConsoleInput::COMMAND(CommandType::LS)));
        assert!(parse_line(&"$ cd /".to_string()) == Ok(ConsoleInput::COMMAND(CommandType::CD("/".to_string()))));
        assert!(parse_line(&"$ cd ..".to_string()) == Ok(ConsoleInput::COMMAND(CommandType::CD("..".to_string()))));
        assert!(parse_line(&"$ cd abcde".to_string()) == Ok(ConsoleInput::COMMAND(CommandType::CD("abcde".to_string()))));
        assert!(parse_line(&"$ poop".to_string()) == Err("Unknown command.".to_string()));
        assert!(parse_line(&"dir fghij".to_string()) == Ok(ConsoleInput::FILE(FileType::DIRECTORY(
            Directory {
                name: "fghij".to_string(),
                contents: Vec::new(),
            }
        ))));

        assert!(parse_line(&"8 bitesize.jpg".to_string()) == Ok(ConsoleInput::FILE(FileType::FILE(
            File {
                name: "bitesize.jpg".to_string(),
                file_size: 8,
            }
        ))));
    }

    #[test]
    fn test_trees() {
        init();

        let mut tree = Tree::new(
            Directory {
                name: "home".to_string(),
                contents: Vec::new(),
            }
        );

        let mut root = Tree::new(
            Directory {
                name: "/".to_string(),
                contents: Vec::new(),
            }
        );

        root.push_back(tree);

        debug!("{:?}", root.iter().next().unwrap().parent().unwrap());

        debug!("{:?}", root.root().as_ref());
    }


    // #[test]
    // fn test_create_tree() {
    //     init();

    //     let input = vec![
    //         "$ cd /".to_string(),
    //         "$ ls".to_string(),
    //         "$ cd foo".to_string(),
    //         "$ cd bar".to_string(),
    //         "$ cd ..".to_string(),
    //     ];

    //     construct_tree(&input);
    // }
}