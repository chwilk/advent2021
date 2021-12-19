use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = find_filename(&args);
    let contents = fs::read_to_string(filename).expect("File error");

    let mut scorelist = Vec::new();

    'lines: for l in contents.lines() {
        let mut line_score = 0;
        let mut stack = Vec::new();
        let tokens = l.chars();
        for t in tokens {
            if is_open(t) {
                stack.push(t);
            } else {
                let o = stack.pop().unwrap();
                if !closes(o, t) {
                    println!("got {} when closing {}, discarding line", t, o);
                    continue 'lines;
                }
            }
        }
        while stack.len() > 0 {
            line_score *= 5;
            line_score += score(get_close(stack.pop().unwrap()));
        }
        println!("Line score: {}", line_score);
        scorelist.push(line_score)
    }
    scorelist.sort();
    let total_score = scorelist[scorelist.len()/2];
    println!("Total score: {}", total_score);
}

fn score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
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

fn get_close(o: char) -> char {
    match o {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _   => '*',
    }
}
