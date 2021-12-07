use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("File error");

    let mut o2gen: Vec<&str> = contents.lines().collect();
    let mut co2scrub: Vec<&str> = contents.lines().collect();

    let mut pos: usize = 0;
    while o2gen.len() > 1 || co2scrub.len() > 1 {

        let o2place = count_ones_by_position(&mut o2gen, pos);
        let co2place = boolean_not_char(count_ones_by_position(&mut co2scrub, pos));
        if o2gen.len() > 1 {
            o2gen.retain(|x| x.chars().nth(pos).unwrap() == o2place);
        }
        if co2scrub.len() > 1 {
            co2scrub.retain(|x| x.chars().nth(pos).unwrap() == co2place);
        }
        pos = pos+1;
        println!("{} {}", o2gen.len(), co2scrub.len());
    }

    let o2generator = i32::from_str_radix(o2gen[0], 2).unwrap();
    let co2scrubber = i32::from_str_radix(co2scrub[0], 2).unwrap();

    println!("o2 generator {}", o2generator);
    println!("co2 scrubber {}", co2scrubber);
    println!("Product {}", o2generator * co2scrubber);
}

fn bin_char_to_int(c: char) -> i32 {
    match c {
        '1' => 1,
        _   => 0,
    }
}

fn count_ones_by_position(lines: &mut Vec<&str>, pos: usize) -> char {
    let mut sum: i32 = 0;
    let mut count: i32 = 0;

    for line in lines {
        sum = sum + bin_char_to_int(line.chars().nth(pos).unwrap());
        count = count + 1;
    }

    if sum * 2 >= count {
        return '1';
    } else {
        return '0';
    }
}

fn boolean_not_char(a: char) -> char {
    match a {
        '1' => '0',
        _ => '1',
    }
}
