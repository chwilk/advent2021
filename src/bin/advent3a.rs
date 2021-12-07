use std::env;
use std::fs;
use std::ops::Shl;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("File error");

    let mut count: i32 = 0;

    let firstline = contents.lines().next().unwrap();
    let bits = firstline.len();
    let mut sums = vec![0; bits];

    for line in contents.lines() {
        for c in line.chars().enumerate() {
            sums[c.0] = sums[c.0] + bin_char_to_int(c.1);
        }
        count = count + 1;
    }

    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;

    for sum in sums {
          gamma = gamma << 1;
          epsilon = epsilon << 1;

          if sum * 2 > count {
              gamma = gamma + 1;
          } else {
              epsilon = epsilon + 1;
          }
    }

    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
}

fn bin_char_to_int(c: char) -> i32 {
    match c {
        '1' => 1,
        _   => 0,
    }
}
