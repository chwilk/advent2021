use std::env;
use std::fs;
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

    let mut risk = 0;
    let max = [grid.len() -1, grid[0].len() -1];

    println!("{} x {}", max[0], max[1]);
    
    // brute force corner (literally) cases first
    if grid[0][0] < grid[0][1] && grid[0][0] < grid[1][0] { // Top left
        risk += 1 + grid[0][0];
    }
    if grid[0][max[1]] < grid[0][max[1]-1] && grid[0][max[1]] < grid[1][max[1]] { // Top right
        risk += 1 + grid[0][max[1]];
    }
    if grid[max[0]][0] < grid[max[0]][1] && grid[max[0]][0] < grid[max[0]-1][0] { // Bottom left
        risk += 1 + grid[max[0]][0];
    }
    if grid[max[0]][max[1]] < grid[max[0]][max[1]-1] && grid[max[0]][max[1]] < grid[max[0]-1][max[1]] { // Bottom right
        risk += 1 + grid[max[0]][max[1]];
    }

    // edge cases
    for j in 1..max[1] { // top / bottom
        if grid[0][j] < grid[0][j-1] &&
           grid[0][j] < grid[0][j+1] &&
           grid[0][j] < grid[1][j] {
               risk += 1 + grid[0][j];
               //println!("found low on top {}", grid[0][j]);
           }
        if grid[max[0]][j] < grid[max[0]][j-1] &&
           grid[max[0]][j] < grid[max[0]][j+1] &&
           grid[max[0]][j] < grid[max[0]-1][j] {
               risk += 1 + grid[max[0]][j];
               //println!("found low on bottom {}", grid[max[0]][j]);
           }
    }
    for i in 1..max[0] { // left / right
        if grid[i][0] < grid[i-1][0] &&
           grid[i][0] < grid[i+1][0] &&
           grid[i][0] < grid[i][1] {
               risk += 1 + grid[i][0];
               //println!("found low on left {}", grid[i][0]);
           }
        if grid[i][max[1]] < grid[i-1][max[1]] &&
           grid[i][max[1]] < grid[i+1][max[1]] &&
           grid[i][max[1]] < grid[i][max[1]-1] {
               risk += 1 + grid[i][max[1]];
               //println!("found low on right {}", grid[i][max[1]]);
           }
    }
    for j in 1..max[1] {
        for i in 1..max[0] {
        if grid[i][j] < grid[i-1][j] &&
           grid[i][j] < grid[i+1][j] &&
           grid[i][j] < grid[i][j-1] &&
           grid[i][j] < grid[i][j+1] {
               risk += 1 + grid[i][j];
               //println!("found low in midsection {}", grid[i][j]);
           }
            
        }
    }
    println!("risk: {}", risk);       
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
