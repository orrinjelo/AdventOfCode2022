use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use gif::{Frame, Encoder, Repeat};
use std::fs::File;
use std::borrow::Cow;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct Snek {
    segments: Vec<Coord>,
    // head: Coord,
    // tail: Coord,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    L, U, R, D,
}

impl Snek {
    pub fn new(start: (i32, i32), size: usize) -> Snek {
        let mut snek = Snek {
            segments: Vec::new()
        };
        for _i in 0..size {
            snek.segments.push(
                Coord {
                    x: start.0,
                    y: start.1,
                }
            );
        }
        snek
    }

    pub fn step(&mut self, dir: Direction, segment_num: usize) {
        debug!("PRE  {} pos: {:?} {:?}", segment_num, self.segments[segment_num], dir);

        match dir {
            Direction::L => {
                if segment_num == 0 {
                    self.segments[0].x -= 1;
                    if segment_num != self.segments.len() - 1 {
                        self.step(Direction::L, segment_num+1);
                    }
                } else {
                    let seg_x = self.segments[segment_num].x;
                    let seg_y = self.segments[segment_num].y;
                    let prv_x = self.segments[segment_num-1].x;
                    let prv_y = self.segments[segment_num-1].y;

                    if (seg_x - prv_x).abs() > 1 {
                        self.segments[segment_num].x -= 1;
                        if segment_num != self.segments.len() - 1 {
                            self.step(Direction::L, segment_num+1);
                        }
                        if (seg_y - prv_y).abs() >= 1 && seg_y > prv_y {
                            self.segments[segment_num].y -= 1;
                            if segment_num != self.segments.len() - 1 {
                                self.step(Direction::U, segment_num+1);
                            }
                        } else if (seg_y - prv_y).abs() >= 1 && seg_y < prv_y {
                            self.segments[segment_num].y += 1;
                            if segment_num != self.segments.len() - 1 {
                                self.step(Direction::D, segment_num+1);
                            }
                        }
                    }

                //     if (self.segments[segment_num].x- self.segments[segment_num-1].x).abs() > 1 {
                //         self.segments[segment_num].x -= 1;
                //         if self.segments[segment_num].y > self.segments[segment_num-1].y {
                //             self.segments[segment_num].y -= 1;
                //         } else if self.segments[segment_num].y < self.segments[segment_num-1].y {
                //             self.segments[segment_num].y += 1;
                //         }
                //     }
                }
            },
            Direction::R => {
                if segment_num == 0 {
                    self.segments[0].x += 1;
                    if segment_num != self.segments.len() - 1 {
                        self.step(Direction::R, segment_num+1);
                    }
                } else {
                    let seg_x = self.segments[segment_num].x;
                    let seg_y = self.segments[segment_num].y;
                    let prv_x = self.segments[segment_num-1].x;
                    let prv_y = self.segments[segment_num-1].y;

                    if (seg_x - prv_x).abs() > 1 {
                        self.segments[segment_num].x += 1;
                        if segment_num != self.segments.len() - 1 {
                            self.step(Direction::R, segment_num+1);
                        }
                        if (seg_y - prv_y).abs() >= 1 && seg_y > prv_y {
                            self.segments[segment_num].y -= 1;
                            if segment_num != self.segments.len() - 1 {
                                self.step(Direction::U, segment_num+1);
                            }
                        } else if (seg_y - prv_y).abs() >= 1 && seg_y < prv_y {
                            self.segments[segment_num].y += 1;
                            if segment_num != self.segments.len() - 1 {
                                self.step(Direction::D, segment_num+1);
                            }
                        }
                    }

                    // if (self.segments[segment_num].x - self.segments[segment_num-1].x).abs() > 1 {
                    //     self.segments[segment_num].x += 1;
                    //     if self.segments[segment_num].y > self.segments[segment_num-1].y {
                    //         self.segments[segment_num].y -= 1;
                    //     } else if self.segments[segment_num].y < self.segments[segment_num-1].y {
                    //         self.segments[segment_num].y += 1;
                    //     }
                    // }
                }
            },
            Direction::U => {
                if segment_num == 0 {
                    self.segments[0].y -= 1;
                    if segment_num != self.segments.len() - 1 {
                        self.step(Direction::U, segment_num+1);
                    }
                } else {
                    let seg_x = self.segments[segment_num].x;
                    let seg_y = self.segments[segment_num].y;
                    let prv_x = self.segments[segment_num-1].x;
                    let prv_y = self.segments[segment_num-1].y;

                    if (seg_y - prv_y).abs() > 1 {
                        self.segments[segment_num].y -= 1;
                        if segment_num != self.segments.len() - 1 {
                            self.step(Direction::U, segment_num+1);
                        }
                        if (seg_x - prv_x).abs() >= 1 && seg_x > prv_x {
                            self.segments[segment_num].x -= 1;
                            if segment_num != self.segments.len() - 1 {
                                self.step(Direction::L, segment_num+1);
                            }
                        } else if (seg_x - prv_x).abs() >= 1 && seg_x < prv_x {
                            self.segments[segment_num].x += 1;
                            if segment_num != self.segments.len() - 1 {
                                self.step(Direction::R, segment_num+1);
                            }
                        }
                    }

                    // if (self.segments[segment_num].y - self.segments[segment_num-1].y).abs() > 1 {
                    //     self.segments[segment_num].y -= 1;
                    //     if self.segments[segment_num].x > self.segments[segment_num-1].x {
                    //         self.segments[segment_num].x -= 1;
                    //     } else if self.segments[segment_num].x < self.segments[segment_num-1].x {
                    //         self.segments[segment_num].x += 1;
                    //     }
                    // }
                }
            },
            Direction::D => {
                if segment_num == 0 {
                    self.segments[0].y += 1;
                    if segment_num != self.segments.len() - 1 {
                        self.step(Direction::D, segment_num+1);
                    }
                } else {
                    let seg_x = self.segments[segment_num].x;
                    let seg_y = self.segments[segment_num].y;
                    let prv_x = self.segments[segment_num-1].x;
                    let prv_y = self.segments[segment_num-1].y;

                    if (seg_y - prv_y).abs() > 1 {
                        self.segments[segment_num].y -= 1;
                        if segment_num != self.segments.len() - 1 {
                            self.step(Direction::D, segment_num+1);
                        }
                        if (seg_x - prv_x).abs() >= 1 && seg_x > prv_x {
                            self.segments[segment_num].x -= 1;
                            if segment_num != self.segments.len() - 1 {
                                self.step(Direction::L, segment_num+1);
                            }
                        } else if (seg_x - prv_x).abs() >= 1 && seg_x < prv_x {
                            self.segments[segment_num].x += 1;
                            if segment_num != self.segments.len() - 1 {
                                self.step(Direction::R, segment_num+1);
                            }
                        }
                    }

                    // if (self.segments[segment_num].y - self.segments[segment_num-1].y).abs() > 1 {
                    //     self.segments[segment_num].y += 1;
                    //     if self.segments[segment_num].x > self.segments[segment_num-1].x {
                    //         self.segments[segment_num].x -= 1;
                    //     } else if self.segments[segment_num].x < self.segments[segment_num-1].x {
                    //         self.segments[segment_num].x += 1;
                    //     }
                    // }
                }
            },
        };
        debug!("POST {} pos: {:?} {:?}", segment_num, self.segments[segment_num], dir);
    }

    pub fn move_me(&mut self, dir: Direction, n: u32) {
        for _i in 0..n {
            self.step(dir, 0);
        }
    }
}

struct Arena {
    size_x: usize,
    size_y: usize,
    snek: Snek,
    field: Vec<Vec<u8>>,
}

impl Arena {
    pub fn new(size: (usize, usize), snek_len: usize) -> Arena {
        let mut arena = Arena {
            size_x: size.0,
            size_y: size.1,
            snek: Snek::new((0,0), snek_len),
            field: vec![vec![0u8; size.0]; size.1],
        };
        arena.field[size.1/2][size.0/2] = 1;
        arena
    }

    pub fn move_snek(&mut self, dir: Direction, n: u32) {
        for i in 0..n {
            // debug!("Moving {}/{}", i, n);
            self.snek.step(dir, 0);
            // debug!("{:?}", self.snek);
            let tail_x = self.snek.segments[self.snek.segments.len()-1].x;
            let tail_y = self.snek.segments[self.snek.segments.len()-1].y;
            self.field[(self.size_y as i32/2 + tail_y) as usize][(self.size_x as i32/2 + tail_x) as usize] = 1;
        }
    }

    pub fn print(&self) {
        let head_x = self.snek.segments[0].x;
        let head_y = self.snek.segments[0].y;
        let tail_x = self.snek.segments[self.snek.segments.len()-1].x;
        let tail_y = self.snek.segments[self.snek.segments.len()-1].y;

        for row in 0..self.size_y {
            for col in 0..self.size_x {
                if self.size_x as i32/2 + head_x == col as i32 && self.size_y as i32/2 + head_y == row as i32 {
                    print!("H");
                } else if self.size_x as i32/2 + tail_x == col as i32 && self.size_y as i32/2 + tail_y == row as i32 {
                    print!("T");
                } else {
                    let mut found = false;

                    for i in 1..self.snek.segments.len()-1 {
                        if self.size_x as i32/2 + self.snek.segments[i].x == col as i32 && self.size_y as i32/2 + self.snek.segments[i].y == row as i32 {
                            print!("{}",i);
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        if self.field[row][col] == 1 {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                }
            }
            println!("");
        }
    }

    pub fn count_painted(&self) -> u32 {
        let mut count = 0;
        for row in 0..self.size_y {
            for col in 0..self.size_x {
                if self.field[row][col] == 1 {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn parse_line(line: &String) -> (Direction, u32) {
    let direction = match line.chars().nth(0).unwrap() {
        'U' => Direction::U,
        'D' => Direction::D,
        'L' => Direction::L,
        'R' => Direction::R,
        _ => Direction::U,
    };
    let mut l = line.clone();
    l.remove(0);
    l.remove(0);
    let num = l.parse::<u32>().unwrap();
    (direction, num)
}

/**
 *  Problem #09, part 1
 */
pub fn problem_091(input: Vec<String>) -> RetType {

    let mut arena = Arena::new((1000,1000), 2);

    for line in input {
        let (dir, n) = parse_line(&line);
        arena.move_snek(dir, n);
    }

    return RetType::U32(arena.count_painted());
}

/**
 *  Problem #09, part 2
 */
pub fn problem_092(input: Vec<String>) -> RetType {
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
    fn test_moves() {
        let mut arena = Arena::new((11,11), 2);
        arena.move_snek(Direction::R, 4);

        // arena.print();
    }

    #[test]
    fn test_part_1() {
        init();

        let input = vec![
            "R 4".to_string(),
            "U 4".to_string(),
            "L 3".to_string(),
            "D 1".to_string(),
            "R 4".to_string(),
            "D 1".to_string(),
            "L 5".to_string(),
            "R 2".to_string(),
        ];

        let mut arena = Arena::new((11,11), 2);

        for line in input {
            let (dir, n) = parse_line(&line);
            arena.move_snek(dir, n);
        }

        // arena.print();

        assert!(arena.count_painted() == 13);
    }

    #[test]
    fn test_part_2() {
        init();

        let input = vec![
            "R 5".to_string(),
            "U 8".to_string(), // was 8
            // "L 8".to_string(),
            // "D 3".to_string(),
            // "R 17".to_string(),
            // "D 10".to_string(),
            // "L 25".to_string(),
            // "U 20".to_string(),
        ];

        let mut arena = Arena::new((50,50), 10);

        for line in input {
            let (dir, n) = parse_line(&line);
            arena.move_snek(dir, n);
        }

        arena.print();

        assert!(arena.count_painted() == 36);
    }
}