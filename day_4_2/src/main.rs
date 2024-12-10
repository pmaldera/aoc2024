use std::env;
use std::fs::read_to_string;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter file path as arg.");
        return;
    }

    let file_path = &args[1];

    let lines: Vec<Vec<char>> = read_to_string(file_path) 
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut count = 0;

    // Could be fun multithread the following code, todo ?
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let check_branch_one: String;
            let check_branch_two: String;

            if y >= 1 && x >= 1 && y < lines.len()-1 && x < lines[y].len()-1 {
                check_branch_one = format!("{}{}{}", lines[y-1][x-1], lines[y][x], lines[y+1][x+1]);
                check_branch_two = format!("{}{}{}", lines[y+1][x-1], lines[y][x], lines[y-1][x+1]);

                if (check_branch_one == "MAS" || check_branch_one == "SAM") && (check_branch_two == "MAS" || check_branch_two == "SAM") {
                    count +=1;
                }
            }
        }
    }

    println!("{}", count);
}