use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {
    let filename = find_filename(env::args());
    let contents = fs::read_to_string(filename).expect("File error"); 

    let mut lines = contents.lines();
    let mut vents = Vec::new();
    let mut max = [0, 0];

    loop {
        let l = lines.next();
        if l == None {
            break;
        }
        let mut a = [0; 4];
        let v: Vec<usize> = l.unwrap().split(|x| (x == ',') || (x == ' ')).filter_map(|x| x.parse::<usize>().ok()).collect();
        for i in 0..4 {
            a[i] = v[i];
            if a[i] > max[i%2] { max[i%2] = a[i];}
        }
        //println!("{},{} -> {},{}", a[0], a[1], a[2], a[3]);
        vents.push(a);
    }

    println!("Max indices found {}, {}", max[0], max[1]);

    let mut grid = vec![vec![0; max[1]+1]; max[0]+1];
    for v in vents {
        if v[0] == v[2] { // vertical
            let mut y_range = v[1]..=v[3];
            if v[1] > v[3] {
                y_range = v[3]..=v[1];
            }
            for j in y_range {
                grid[v[0]][j] += 1;
            }
        } else if v[1] == v[3] { // horiz
            let mut x_range = v[0]..=v[2];
            if v[0] > v[2] {
                x_range = v[2]..=v[0];
            }
            for i in x_range {
                grid[i][v[1]] += 1;
            }
        } else { // Diagon alley
            let mut slope: i8 = 1;
            let mut start = [0,0];
            let size;
            if v[0] > v[2] {
                start[0] = v[2];
                start[1] = v[3];
                size = v[0] - v[2];
                slope = -1;
            } else {
                start[0] = v[0];
                start[1] = v[1];
                size = v[2] - v[0];
            }
            if v[1] > v[3] {
                slope *= -1;
            }
            for i in 0..=size {
                let y;
                if slope == 1 {
                    y = start[1] + i;
                } else {
                    y = start[1] - i;
                }
                grid[start[0] + i][y] += 1;
            }
        }
    }

    //print_grid(&grid, max);
    println!("zones: {}", count_hot_zones(&grid, max));
}

#[allow(dead_code)]
fn print_grid (grid: &Vec<Vec<usize>>, max: [usize; 2]) {
    for j in 0..=max[1] {
        for i in 0..=max[0] {
            if grid[i][j] == 0 {
                print!(".");
            } else {
                print!("{}", grid[i][j]);
            }
        }
        print!("\n");
    }
}

fn count_hot_zones (grid: &Vec<Vec<usize>>, max: [usize; 2]) -> u32 {
    let mut zones = 0;

    for j in 0..=max[1] {
        for i in 0..=max[0] {
            if grid[i][j] > 1 {
                zones += 1;
            }
        }
    }
    zones
}
