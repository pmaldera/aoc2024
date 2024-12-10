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
            let mut check: String;
            // Searching north west
            if y >= 3 && x >= 3 {
                check = format!("{}{}{}{}", lines[y][x], lines[y-1][x-1], lines[y-2][x-2], lines[y-3][x-3]);
                if check == "XMAS" { count += 1;};
            }

            // Searching north
            if y >= 3 {
                check = format!("{}{}{}{}", lines[y][x], lines[y-1][x], lines[y-2][x], lines[y-3][x]);
                if check == "XMAS" { count += 1;};
            }

             // Searching north east
             if y >= 3 && x < lines[y].len()-3 {
                check = format!("{}{}{}{}", lines[y][x], lines[y-1][x+1], lines[y-2][x+2], lines[y-3][x+3]);
                if check == "XMAS" { count += 1;};
            }

            // Searching west
            if x >= 3 {
                check = format!("{}{}{}{}", lines[y][x], lines[y][x-1], lines[y][x-2], lines[y][x-3]);
                if check == "XMAS" { count += 1;};
            }

            // Searching east
            if x < lines[y].len()-3 {
                check = format!("{}{}{}{}", lines[y][x], lines[y][x+1], lines[y][x+2], lines[y][x+3]);
                if check == "XMAS" { count += 1;};
            }

            // Searching south west
            if y < lines.len()-3 && x >= 3 {
                check = format!("{}{}{}{}", lines[y][x], lines[y+1][x-1], lines[y+2][x-2], lines[y+3][x-3]);
                if check == "XMAS" { count += 1;};
            }

            // Searching south
            if y < lines.len()-3 {
                check = format!("{}{}{}{}", lines[y][x], lines[y+1][x], lines[y+2][x], lines[y+3][x]);
                if check == "XMAS" { count += 1;};
            }

            // Searching south east
            if y < lines.len()-3 && x < lines[y].len()-3 {
                check = format!("{}{}{}{}", lines[y][x], lines[y+1][x+1], lines[y+2][x+2], lines[y+3][x+3]);
                if check == "XMAS" { count += 1;};
            }
        } 
    }

    println!("{}", count);
}