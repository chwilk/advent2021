use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {
    let filename = find_filename(env::args());

    let contents = fs::read_to_string(filename).expect("File error");

    let lines = contents.lines();
    let depths = lines.filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();
    let sliding_depths = sliding_window(&depths);

    println!("Total drops {}", count_drops(depths));
    println!("Sliding drops {}", count_drops(sliding_depths));
    
}

fn count_drops(depths: Vec<i32>) -> i32 {
    let mut prev = depths[0];
    let mut count=0;
    
    for x in &depths {
        if x > &prev {
            count = count + 1;
        }
        prev = *x;
    }
    count
}

fn sliding_window(depths: &Vec<i32>) -> Vec<i32> {
    let len = depths.len()-2;
    let mut sdepths = vec![0; len];

    for i in 0..=len-1 {
        sdepths[i] = depths[i] + depths[i+1] + depths[i+2];
    }
    
    sdepths
}
