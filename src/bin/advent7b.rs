use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = find_filename(&args);
    let contents = fs::read_to_string(filename).expect("File error");
    let mut lines = contents.lines();
    let crabs: Vec<i32> = lines.next().unwrap().split(',').filter_map(|x| x.parse::<i32>().ok()).collect();

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let mut minima = 1144314220;
    let mut ideal = 0;

    println!("Min/Max {}/{}", min, max);
    // Scan all the values!
    for i in (*min as usize)..=(*max as usize) {
        let costi = cost(&crabs, i.try_into().unwrap());
        if costi < minima {
            minima = costi;
            ideal = i;
        }
        println!("{}: {}", i, costi);
    }

    println!("Lowest cost at {} with {}", ideal, minima);
}

fn cost(crabs: &Vec<i32>, pos: i32) -> i32 {
    let mut sum: i32 = 0;
    for c in crabs {
        sum += gauss((c - pos).abs());
    }
    sum
}

fn gauss(i: i32) -> i32 {
    (i + 1) * i / 2
}
