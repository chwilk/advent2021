use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = find_filename(&args);

    let mut days = 80;

    if args.len() == 3 {
        days = args[2].parse::<i32>().unwrap();
    }

    let contents = fs::read_to_string(filename).expect("File error"); 

    let mut lines = contents.lines();
    let initial: Vec<u8> = lines.next().unwrap().split(',').filter_map(|x| x.parse::<u8>().ok()).collect();
    let mut fish = [0; 9];
    for lf in initial {
        fish[lf as usize] += 1;
    }
    for _i in 1..days+1 {
        fish = breed_fish(fish);
    }
    println!("{}: {} [{}, {} .. {}]", days, sum_fish(&fish), fish[0], fish[1], fish[8]);
}

fn sum_fish(fish: &[u64; 9]) -> u64 {
    let mut sum = 0;
    for f in fish {
        sum += f;
    }
    sum
}

fn breed_fish(mut fish: [u64; 9]) -> [u64; 9]{
    let breeding = fish[0];
    for i in 0..8 {
        fish[i] = fish[i+1];
    }

    fish[6] += breeding;
    fish[8] = breeding;
    fish
}
