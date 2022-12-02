mod problems;
mod util;

use log::{info, warn, Level}; // trace, debug, info, warn, error
use env_logger;
use std::io::Write;
use std::time::{Instant, Duration};
use rustop::opts;
use crate::util::RetType;

macro_rules! ifelse {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

fn format_time(ts: Duration) -> String {
    if ts.as_micros() >= 1_000_000 {
        return format!("{} s", (ts.as_millis() as f32)/1000.0);
    } else if ts.as_micros() >= 1000 {
        return format!("{} ms", (ts.as_micros() as f32)/1000.0);
    } 
    return format!("{} μs", ts.as_micros());
}

fn execute_problem(num: i32, input: Vec<String>, part1: fn(Vec<String>) -> RetType, part2: fn(Vec<String>) -> RetType) {
    let start = Instant::now();
    let result_1 = part1(input.clone());
    let then_elapsed = start.elapsed();
    let then = Instant::now();
    let result_2 = part2(input.clone());
    let end_elapsed = then.elapsed();
    info!("Problem {}; Part 1: {} (Runtime: {})", num, result_1, format_time(then_elapsed));
    info!("Problem {}; Part 2: {} (Runtime: {})", num, result_2, format_time(end_elapsed));    
}

fn run_problem(num: i32, input: Vec<String>) {
    match num {
        // Example problem (problem from last year!)
        0 => execute_problem(num, input, problems::problem00::problem_001, problems::problem00::problem_002),
        // Problem 1; Elven Financemancy
        1 => execute_problem(num, input, problems::problem01::problem_011, problems::problem01::problem_012),
        _ => warn!("Problem number not available.")
    }
}


fn main() {
    // Set up logging
    env_logger::builder()
        .format(|buf, record| {
            let mut style = buf.style();

            let color = match record.level() {
                Level::Trace => env_logger::fmt::Color::Magenta,
                Level::Debug => env_logger::fmt::Color::Cyan,
                Level::Info  => env_logger::fmt::Color::Green,
                Level::Warn  => env_logger::fmt::Color::Yellow,
                Level::Error => env_logger::fmt::Color::Red,
            };

            style.set_color(color);
            writeln!(buf, "{}: {}", style.value(record.level()), record.args())
        })
        .init();

    let opts = opts! {
        synopsis "Advent of Code 2022";
        opt run_all:bool, desc: "Run all problems.";
        opt input_file:Option<String>, desc: "Custom input file for this problem.";
        param number:Option<i32>, desc:"Problem number to run.";
    };

    let (args, _rest) = opts.parse_or_exit();

    info!("{:?}", args.number);

    info!("==== Advent of Code 2022 ====");

    // Parse args
    if args.run_all {
        for i in 0..2 {
            let filename = format!("aoc2022/inputs/{:02}.txt", i).to_string();
            let input = util::load_file(filename);
            run_problem(i, input);
            info!("=========================");
        }
    } else {
        if let Some(num) = args.number {
            let filename = ifelse!(args.input_file.is_none(), format!("aoc2022/inputs/{:02}.txt", num).to_string(), args.input_file.unwrap());
            let input = util::load_file(filename);
            run_problem(num, input);
        }
    }
}
