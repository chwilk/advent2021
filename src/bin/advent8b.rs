use std::env;
use std::fs;
use std::collections::HashSet;
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
    let mut sum: u32 = 0;
    let patterns: Vec<HashSet<char>> = pat_str.split(' ').map(|x| x.chars().collect()).collect();
    let mut digits = [10; 10];
    let values: Vec<HashSet<char>> = val_str.split(' ').map(|x| x.chars().collect()).collect();
    for i in 0..=9 { // find 1478 and mark them
        match patterns[i].len() {
            2 => digits[1] = i,
            4 => digits[4] = i,
            3 => digits[7] = i,
            7 => digits[8] = i,
            _ => (),
        }
    }
    for i in 0..=9 {
        if patterns[i].len() == 6 {                                                                       // 0,6,9
            if patterns[digits[1]].intersection(&patterns[i]).collect::<Vec<&char>>().len() == 1 {        // 6
                digits[6] = i;
            } else if patterns[digits[4]].intersection(&patterns[i]).collect::<Vec<&char>>().len() == 4 { // 9
                digits[9] = i;
            } else {                                                                                      // 0
                digits[0] = i;
            }
        } else if patterns[i].len() == 5 {                                                                // 2,3,5
            if patterns[digits[1]].intersection(&patterns[i]).collect::<Vec<&char>>().len() == 2 {        // 3
                digits[3] = i;
            } else if patterns[digits[4]].intersection(&patterns[i]).collect::<Vec<&char>>().len() == 3 { // 5
                digits[5] = i;
            } else {                                                                                      // 2
                digits[2] = i;
            }
        }
    }
    for v in values {
        sum *= 10;
        for i in 0..=9 {
            if patterns[digits[i]] == v {
                sum += i as u32;
            }
        }
    }
    sum
}
