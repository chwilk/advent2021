//use std::error::Error;
use std::format;

pub fn find_filename(args: &[String]) -> String {
    let filename: String;
    if args.len() < 2 { // Try to guess a test#.dat from binary name
        let day = parse_int(&args[0]).unwrap();
        filename = format!("test{}.dat", day);
    } else {
        filename = args[1].clone();
    }
    filename
}

fn parse_int(input: &str) -> Option<u32> {
    input
        .chars()
        .skip_while(|ch| !ch.is_digit(10))
        .take_while(|ch| ch.is_digit(10))
        .fold(None, |acc, ch| {
            ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b)
        })
}
