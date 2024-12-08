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
    let mut activated:bool = true;
    let mut total: i32 = 0;
    let re = Regex::new(r"(don't\(\))|(do\(\))|(mul\(?\d+,?\d+\))").unwrap();
    let re_number = Regex::new(r"mul\(?(\d+),?(\d+)\)").unwrap();

    for (_, [matched]) in re.captures_iter(&file_content).map(|c| c.extract()) {
        if matched == "do()" {
            activated = true;
        } else if matched == "don't()" {
            activated = false
        } else {
            let Some(result) = re_number.captures(matched) else {continue};
            if activated {
                total += result[1].parse::<i32>().unwrap() * result[2].parse::<i32>().unwrap();
            }
        }
    }

    println!("{:?}", total);
}