use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use plotters::prelude::*;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(Debug)]
pub struct ArboralLandscape {
   tree_map: Vec<Vec<u8>>,
   visible_map: Vec<Vec<bool>>,
   score_map: Vec<Vec<u32>>,
}

impl ArboralLandscape {
    pub fn new(string_map: Vec<String>) -> ArboralLandscape {
        let rows = string_map.len();
        let cols = string_map[0].len();

        let mut tm = vec![vec![0u8; cols]; rows];

        for row_num in 0..rows {
            for col_num in 0..cols {
                tm[row_num][col_num] = string_map[row_num].chars().nth(col_num).unwrap().to_digit(10).unwrap() as u8;
            }
        }

        let mut vm = vec![vec![false; cols]; rows];
        let mut sm = vec![vec![0; cols]; rows];

        // Right
        // let mut highest_tree: u8 = 0;
        for row_num in 0..rows {
            let mut highest_tree: u8 = tm[row_num][0];
            for col_num in 0..cols {
                vm[row_num][col_num] = {
                    if vm[row_num][col_num] == true {
                        if highest_tree < tm[row_num][col_num] {
                            highest_tree = tm[row_num][col_num];
                        }
                        true
                    } else if col_num == 0 {
                        highest_tree = tm[row_num][col_num];
                        true
                    } else if highest_tree >= tm[row_num][col_num] {
                        false
                    } else {
                        highest_tree = tm[row_num][col_num];
                        true
                    }
                };
            }
        }

        // Left
        for row_num in 0..rows {
            let mut highest_tree: u8 = tm[row_num][cols - 1];
            for col_num_f in 0..cols {
                let col_num = cols - col_num_f - 1;
                vm[row_num][col_num] = {
                    if vm[row_num][col_num] == true {
                        if highest_tree < tm[row_num][col_num] {
                            highest_tree = tm[row_num][col_num];
                        }
                        true
                    } else if col_num == cols - 1 {
                        highest_tree = tm[row_num][col_num];
                        true
                    } else if highest_tree >= tm[row_num][col_num] {
                        false
                    } else {
                        highest_tree = tm[row_num][col_num];
                        true
                    }
                };
            }
        }

        // Down
        for col_num in 0..cols {
            let mut highest_tree: u8 = tm[0][col_num];
            for row_num in 0..rows {
                // debug!("X {},{} => {}/{}", row_num, col_num, tm[row_num][col_num], highest_tree);
                vm[row_num][col_num] = {
                    if vm[row_num][col_num] == true {
                        if highest_tree < tm[row_num][col_num] {
                            highest_tree = tm[row_num][col_num];
                        }
                        // debug!("A {},{} => {}/{}", row_num, col_num, tm[row_num][col_num], highest_tree);
                        true
                    } else if row_num == 0 {
                        highest_tree = tm[row_num][col_num];
                        // debug!("B {},{} => {}/{}", row_num, col_num, tm[row_num][col_num], highest_tree);
                        true
                    } else if highest_tree >= tm[row_num][col_num] {
                        // debug!("C {},{} => {}/{}", row_num, col_num, tm[row_num][col_num], highest_tree);
                        false
                    } else {
                        highest_tree = tm[row_num][col_num];
                        // debug!("D {},{} => {}/{}", row_num, col_num, tm[row_num][col_num], highest_tree >= tm[row_num][col_num]);
                        true
                    }
                };
            }
        }

        // Up
        for col_num in 0..cols {
            let mut highest_tree: u8 = tm[rows - 1][col_num];
            for row_num_n in 0..rows {
                let row_num = rows - row_num_n - 1;
                vm[row_num][col_num] = {
                    if vm[row_num][col_num] == true {
                        if highest_tree < tm[row_num][col_num] {
                            highest_tree = tm[row_num][col_num];
                        }
                        true
                    } else if row_num == rows-1 {
                        highest_tree = tm[row_num][col_num];
                        true
                    } else if highest_tree >= tm[row_num][col_num] {
                        false
                    } else {
                        highest_tree = tm[row_num][col_num];
                        true
                    }
                };
            }
        }
        // debug!("{:?}", vm);

        // Score map
        for col in 0..cols {
            for row in 0..rows {
                let mut left_score = 0;
                if col != 0 { // Left
                    for c in (0..col).rev() {
                        left_score += 1;
                        if tm[row][c] >= tm[row][col] {
                            break;
                        }
                    }
                }
                let mut right_score = 0;
                if col != cols-1 { // Right
                    for c in col+1..cols {
                        right_score += 1;
                        if tm[row][c] >= tm[row][col] {
                            break;
                        }
                    }
                }
                let mut up_score = 0;
                if row != 0 { // Up
                    for r in (0..row).rev() {
                        up_score += 1;
                        if tm[r][col] >= tm[row][col] {
                            break;
                        }
                    }
                }
                let mut down_score = 0;
                if row != rows-1 { // Down
                    for r in row+1..rows {
                        down_score += 1;
                        if tm[r][col] >= tm[row][col] {
                            break;
                        }
                    }
                }
                sm[row][col] = left_score * right_score * up_score * down_score;
            }
        }

        ArboralLandscape {
            tree_map: tm,
            visible_map: vm,
            score_map: sm,
        }
    }

    #[allow(dead_code)]
    pub fn print_n(&self) {
        for row in &self.tree_map {
            for c in 0..row.len() {
                print!("{}", row[c]);
            }
            print!("");
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        self.print_n();
    }

    #[allow(dead_code)]
    pub fn count_visible(&self) -> usize {
        self.visible_map.iter().map(|x| x.iter().map(|y| if *y {1} else {0}).sum::<usize>()).sum()
    }

    #[allow(dead_code)]
    pub fn find_max_score(&self) -> u32 {
        let flatten_array: Vec<u32> = self.score_map
                        .iter()
                        .flat_map(|array| array.iter())
                        .cloned()
                        .collect();
        *flatten_array.iter().max().unwrap()
    }

    #[allow(dead_code)]
    pub fn plot_trees(&self) {
        let root = BitMapBackend::new("tree_map.png", (1024, 768)).into_drawing_area();

        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Just a bunch of trees.", ("sans-serif", 80))
            .margin(5)
            .top_x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0i32..100i32, 100i32..0i32).unwrap();

        // chart
        //     .configure_mesh()
        //     .x_labels(100)
        //     .y_labels(100)
        //     .max_light_lines(4)
        //     .x_label_offset(35)
        //     .y_label_offset(25)
        //     .disable_x_mesh()
        //     .disable_y_mesh()
        //     .label_style(("sans-serif", 20))
        //     .draw().unwrap();

        chart.draw_series(
            self.tree_map
                .iter()
                .zip(0..)
                .map(|(l, y)| l.iter().zip(0..).map(move |(v, x)| (x as i32, y as i32, v)))
                .flatten()
                .map(|(x, y, v)| {
                    Rectangle::new(
                        [(x, y), (x + 1, y + 1)],
                        RGBColor(
                            15u8,
                            *v * 25u8,
                            15u8,
                        )
                        .filled(),
                    )
                }),
        ).unwrap();
    }

    #[allow(dead_code)]
    pub fn plot_visible_ext(&self) {
        let root = BitMapBackend::new("visible_map.png", (1024, 768)).into_drawing_area();

        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Tree visibility.", ("sans-serif", 80))
            .margin(5)
            .top_x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0i32..100i32, 100i32..0i32).unwrap();

        // chart
        //     .configure_mesh()
        //     .x_labels(100)
        //     .y_labels(100)
        //     .max_light_lines(4)
        //     .x_label_offset(35)
        //     .y_label_offset(25)
        //     .disable_x_mesh()
        //     .disable_y_mesh()
        //     .label_style(("sans-serif", 20))
        //     .draw().unwrap();

        chart.draw_series(
            self.visible_map
                .iter()
                .zip(0..)
                .map(|(l, y)| l.iter().zip(0..).map(move |(v, x)| (x as i32, y as i32, v)))
                .flatten()
                .map(|(x, y, v)| {
                    Rectangle::new(
                        [(x, y), (x + 1, y + 1)],
                        RGBColor(
                            15u8,
                            (*v as u8) * 255u8,
                            15u8,
                        )
                        .filled(),
                    )
                }),
        ).unwrap();
    }
    
    #[allow(dead_code)]
    pub fn plot_visible_int(&self) {
        let root = BitMapBackend::new("score_map.png", (1024, 768)).into_drawing_area();

        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("ln(score)", ("sans-serif", 80))
            .margin(5)
            .top_x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0i32..100i32, 100i32..0i32).unwrap();

        // chart
        //     .configure_mesh()
        //     .x_labels(100)
        //     .y_labels(100)
        //     .max_light_lines(4)
        //     .x_label_offset(35)
        //     .y_label_offset(25)
        //     .disable_x_mesh()
        //     .disable_y_mesh()
        //     .label_style(("sans-serif", 20))
        //     .draw().unwrap();

        chart.draw_series(
            self.score_map
                .iter()
                .zip(0..)
                .map(|(l, y)| l.iter().zip(0..).map(move |(v, x)| (x as i32, y as i32, v)))
                .flatten()
                .map(|(x, y, v)| {
                    Rectangle::new(
                        [(x, y), (x + 1, y + 1)],
                        RGBColor(
                            15u8,
                            (((*v as f64).ln() / 14.0) * 255.0) as u8,
                            15u8,
                        )
                        .filled(),
                    )
                }),
        ).unwrap();
    }


    // pub fn traverse(&mut self, down: u8, right: u8) -> u128 {
    //     let mut tree_count: u128 = 0;
    //     let mut position = (0, 0);
    //     let row_len = self.tree_map[0].len();
    //     while position.0 < self.tree_map.len() {
    //         tree_count += self.tree_map[position.0][position.1 % row_len] as u128;
    //         position = (position.0 + down as usize, position.1 + right as usize);
    //     }
    //     return tree_count;
    // }
}


/**
 *  Problem #08, part 1
 */
pub fn problem_081(input: Vec<String>) -> RetType {
    let landscape = ArboralLandscape::new(input);
    landscape.plot_trees();
    landscape.plot_visible_ext();
    landscape.plot_visible_int();
    return RetType::U32(landscape.count_visible() as u32);
}

/**
 *  Problem #08, part 2
 */
pub fn problem_082(input: Vec<String>) -> RetType {
    let landscape = ArboralLandscape::new(input);
    return RetType::U32(landscape.find_max_score());
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
    fn test_parse_trees() {
        init();

        let input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let landscape = ArboralLandscape::new(input);
        // landscape.print();
        debug!("{:?}", landscape.count_visible());
    }

    #[test]
    fn test_part2() {
        init();

        let input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let landscape = ArboralLandscape::new(input);
        // landscape.print();
        debug!("{:?}", landscape);
        assert!(landscape.find_max_score() == 8);
    }
}