use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = find_filename(&args);
    let contents = fs::read_to_string(filename).expect("File error");

    let lines = contents.lines();

    let mut sum = 0;

    for line in lines {
        let entry: Vec<&str> = line.split('|').collect();
        sum += decode_7segment(entry[0], entry[1]);
    }
    println!("sum of displays: {}", sum);
}

fn decode_7segment(pat_str: &str, val_str: &str) -> u32 {
    let mut sum = 0;
    let patterns: Vec<String> = pat_str.split(' ').map(|x| alphabetize(x)).collect();
    let mut digits = [10; 10];
    let values: Vec<String> = val_str.split(' ').map(|x| alphabetize(x)).collect();
    for i in 0..=9 {
        match patterns[i].len() {
            2 => digits[i] = 1,
            4 => digits[i] = 4,
            3 => digits[i] = 7,
            7 => digits[i] = 8,
            _ => print!("{} is unk ", i),
        }
    }
    println!();
    sum
}

fn alphabetize(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect::<String>()
}
