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

    let file = File::open(file_path).expect("Should be able to read the giveen file");
    let lines = io::BufReader::new(file).lines();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in lines.flatten() {
        let mut splits = line.split("   ");
        let mut num: i32 = splits.next().unwrap().parse().unwrap();
        left_list.push(num);
        num = splits.next().unwrap().parse().unwrap();
        right_list.push(num);
    }

    left_list.sort();
    right_list.sort();

    let mut total: i32 = 0;

    for i in 0..(left_list.len()) {
        total += (left_list[i]-right_list[i]).abs();
    }

    println!("{:?}", total);
}