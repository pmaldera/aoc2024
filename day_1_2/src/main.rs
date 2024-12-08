use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

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

    let mut appeareances_count: HashMap<i32, i32> = HashMap::new();

    for line in lines.flatten() {
        let mut splits = line.split("   ");
        let mut num: i32 = splits.next().unwrap().parse().unwrap();
        left_list.push(num);
        num = splits.next().unwrap().parse().unwrap();
        appeareances_count.entry(num).and_modify(|e| { *e += 1 }).or_insert(1);
    }

    left_list.sort();

    let mut total: i32 = 0;

    for i in 0..(left_list.len()) {
        total += appeareances_count.get(&left_list[i]).or(Some(&0)).unwrap() * left_list[i];
    }

    println!("{:?}", total);
}