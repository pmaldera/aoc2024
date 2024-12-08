use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter file path as arg.");
        return;
    }

    let file_path = &args[1];

    let file = File::open(file_path).expect("Should be able to read the given file");
    let lines = io::BufReader::new(file).lines();
    let mut total: i32 = 0;

    'outer: for line in lines.flatten() {
        let mut splits = line.split(" ");
        let mut num: i32 = splits.next().unwrap().parse().unwrap();
        let mut next_num:i32 = splits.next().unwrap().parse().unwrap();
        let mut result: i32 = num - next_num;
        let mut result_abs: i32 = result.abs();
        let incr_or_decr_indicator: i32 = num - next_num;
        let mut error_count: i8 = 0;

        if !(result_abs >= 1 && result_abs <= 3 && (incr_or_decr_indicator * result > 0)) {
            error_count += 1;
            if error_count > 1 {
                continue 'outer;
            }
        }
        
        while let Some(next_str) = splits.next() {
            num = next_num;
            next_num = next_str.parse().unwrap();
            result = num - next_num;
            result_abs = result.abs();

            if !(result_abs >= 1 && result_abs <= 3 && (incr_or_decr_indicator * result > 0)) {
                error_count += 1;
                if error_count > 1 {
                    continue 'outer;
                }
            }
        }

        total += 1;
    }

    println!("{:?}", total);
}