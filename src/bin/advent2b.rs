use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("File error");

    let lines = contents.lines();

    let mut pos: (i32, i32, i32) = (0, 0, 0); // (pos, depth, aim)

    for line in lines {
        pos = parse(&line, pos);
    }
    println!("({}, {}, {})", pos.0, pos.1, pos.2);
    println!("mult: {}", pos.0 * pos.1)
}

fn parse(line: &str, mut pos: (i32, i32, i32)) -> (i32, i32, i32) {
    let command: Vec<&str> = line.split(' ').collect();
    let distance: i32 = command[1].parse::<i32>().unwrap();
    match command[0] {
        "up" => pos.2 = pos.2 - distance,
        "down" => pos.2 = pos.2 + distance,
        "forward" => {
            pos.0 = pos.0 + distance;
            pos.1 = pos.1 + pos.2 * distance;
        },
        &_ => (),
    }
    pos
}
