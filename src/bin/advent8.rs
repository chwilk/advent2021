use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {
    let filename = find_filename(env::args());
    let contents = fs::read_to_string(filename).expect("File error");

    let lines = contents.lines();

    let mut sum = 0;

    for line in lines {
        let entry: Vec<&str> = line.split('|').collect();
        sum += count_1478(entry[1]);
    }
    println!("sum of 1, 4, 7, and 8: {}", sum);
}

fn count_1478(line: &str) -> u32 {
    let mut sum = 0;
    let patterns: Vec<&str> = line.split(' ').collect();
    for p in patterns {
        match p.len() {
            2 => sum += 1,
            4 => sum += 1,
            3 => sum += 1,
            7 => sum += 1,
            _ => (),
        }
    }
    sum
}
