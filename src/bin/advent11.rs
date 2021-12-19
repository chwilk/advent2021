use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = find_filename(&args);
    let contents = fs::read_to_string(filename).expect("File error");

    let mut grid = [[0; 10] ; 10];
    let mut i = 0;
    for line in contents.lines() {
        let row = line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
        for j in 0..10 {
            grid[i][j] = row[j] as u8;
        }
        i += 1;
    }

    let mut flashes = 0;
    for _i in 1..=100 {
        flashes += step_grid(&mut grid);
    }

    println!("{} flashes", flashes);
}

#[allow(dead_code)]
fn print_grid(grid: [[u8; 10]; 10]) {
    for i in 0..10 {
        for j in 0..10 {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}

fn step_grid(grid: &mut [[u8; 10]; 10]) -> u32 {
    let mut flashes = 0;
    for i in 0..10 {
        for j in 0..10 {
            grid[i][j] += 1;
        }
    }
    loop {
        let flashed = flashes;
        for i in 0..10 {
            for j in 0..10 {
                if grid[i][j] > 9 {
                    flash(grid, i, j);
                    flashes += 1;
                }
            }
        }
        if flashed == flashes { break; }
    }
    flashes
}

fn flash(grid: &mut [[u8; 10]; 10], x: usize, y: usize) {
    grid[x][y] = 0;
    let len_x = match x {
        0 => 2,
        9 => 2,
        _ => 3,
    };
    let len_y = match y {
        0 => 2,
        9 => 2,
        _ => 3,
    };
    let ini_x = match x {
        0 => 0,
        _ => x-1,
    };
    let ini_y = match y {
        0 => 0,
        _ => y-1,
    };
    for i in ini_x..ini_x+len_x {
        for j in ini_y..ini_y+len_y {
            if grid[i][j] != 0 {
                grid[i][j] += 1;
            }
        }
    }
}
