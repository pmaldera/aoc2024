use std::{env, fs};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter file path as arg.");
        return;
    }

    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path).unwrap();
    let mut total: i32 = 0;
    let re = Regex::new(r"mul\(?(\d+),?(\d+)\)").unwrap();

    for (_, [number_1, number_2]) in re.captures_iter(&file_content).map(|c| c.extract()) {
        total += number_1.parse::<i32>().unwrap() * number_2.parse::<i32>().unwrap();
    }

    println!("{:?}", total);
}