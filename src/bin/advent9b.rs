use std::env;
use std::fs;
use std::collections::HashSet;
use advent2021::find_filename;

fn main() {
    let filename = find_filename(env::args());
    let contents = fs::read_to_string(filename).expect("File error");

    let mut grid = Vec::new();
    for line in contents.lines() {
        let row = line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
        grid.push(row);
    }
    print_grid(&grid);

    let max = [grid.len() -1, grid[0].len() -1];

    let mut basins = Vec::new();

    println!("{} x {}", max[0], max[1]);
    
    // brute force corner (literally) cases first
    if grid[0][0] < grid[0][1] && grid[0][0] < grid[1][0] { // Top left
        basins.push(process_low(&grid, 0, 0));
    }
    if grid[0][max[1]] < grid[0][max[1]-1] && grid[0][max[1]] < grid[1][max[1]] { // Top right
        basins.push(process_low(&grid, 0, max[1]));
    }
    if grid[max[0]][0] < grid[max[0]][1] && grid[max[0]][0] < grid[max[0]-1][0] { // Bottom left
        basins.push(process_low(&grid, max[0], 0));
    }
    if grid[max[0]][max[1]] < grid[max[0]][max[1]-1] && grid[max[0]][max[1]] < grid[max[0]-1][max[1]] { // Bottom right
        basins.push(process_low(&grid, max[0], max[1]));
    }

    // edge cases
    for j in 1..max[1] { // top / bottom
        if grid[0][j] < grid[0][j-1] &&
           grid[0][j] < grid[0][j+1] &&
           grid[0][j] < grid[1][j] {
               basins.push(process_low(&grid, 0, j));
           }
        if grid[max[0]][j] < grid[max[0]][j-1] &&
           grid[max[0]][j] < grid[max[0]][j+1] &&
           grid[max[0]][j] < grid[max[0]-1][j] {
               basins.push(process_low(&grid, max[0], j));
           }
    }
    for i in 1..max[0] { // left / right
        if grid[i][0] < grid[i-1][0] &&
           grid[i][0] < grid[i+1][0] &&
           grid[i][0] < grid[i][1] {
               basins.push(process_low(&grid, i, 0));
           }
        if grid[i][max[1]] < grid[i-1][max[1]] &&
           grid[i][max[1]] < grid[i+1][max[1]] &&
           grid[i][max[1]] < grid[i][max[1]-1] {
               basins.push(process_low(&grid, i, max[1]));
           }
    }
    for j in 1..max[1] {
        for i in 1..max[0] {
        if grid[i][j] < grid[i-1][j] &&
           grid[i][j] < grid[i+1][j] &&
           grid[i][j] < grid[i][j-1] &&
           grid[i][j] < grid[i][j+1] {
               basins.push(process_low(&grid, i, j));
           }
            
        }
    }

    basins.sort();
    basins.reverse();
    basins.truncate(3);
    let mut prod = 1;
    for i in basins {
        print!("{} ", i);
        prod *= i as u32;
    }
    println!(" Product {}", prod);
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<u32>>) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn process_low(grid: &Vec<Vec<u32>>, r: usize, c: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut todo = HashSet::new();

    todo.insert([r,c]);

    loop {
        let start = visited.len();
        for p in &todo.clone() {
            let p1 = [p[0], p[1]];
            if grid[p[0]][p[1]] == 9 {
                todo.remove(&p1);
                continue;
            }
            // look up
            if p[0] > 0 && !visited.contains(&p1) {
                todo.insert([p[0]-1, p[1]]);
            }
            // look down
            if p[0] < rows-1 && !visited.contains(&p1) {
                todo.insert([p[0]+1, p[1]]);
            }
            // look left
            if p[1] > 0 && !visited.contains(&p1) {
                todo.insert([p[0], p[1]-1]);
            }
            // look right
            if p[1] < cols-1 && !visited.contains(&p1) {
                todo.insert([p[0], p[1]+1]);
            }
            visited.insert(p1);
        }

        if visited.len() == start { break; }
    }
    println!("Found {}, {}, with size {}", c, r, visited.len());
    visited.len()
}
