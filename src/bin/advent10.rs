use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {
    let filename = find_filename(env::args());
    let contents = fs::read_to_string(filename).expect("File error");

    let mut total_score = 0;


    'lines: for l in contents.lines() {
        let mut stack = Vec::new();
        let tokens = l.chars();
        for t in tokens {
            if is_open(t) {
                stack.push(t);
            } else {
                let o = stack.pop().unwrap();
                if !closes(o, t) {
                    total_score += score(t);
                    println!("got {} when closing {}", t, o);
                    continue 'lines;
                }
            }
        }
        println!();
    }
    println!("Total score: {}", total_score);
}

fn score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _   => 0,
    }
}

fn is_open(c: char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _   => false,
    }
}

fn closes(o: char, c: char) -> bool {
    match o {
        '(' => c == ')',
        '[' => c == ']',
        '{' => c == '}',
        '<' => c == '>',
        _   => false,
    }
}
