use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let width = 10;
    let height = 10;
    let mut grid_raw = vec![0; width * height];

    // Vector of 'width' elements slices
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();

    // Final 2d array `&mut [&mut [_]]`
    let grid = grid_base.as_mut_slice();
    fill_grid(grid);

    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += execute_round(grid);
    }

    println!("Total flashes {}", total_flashes);
}

fn fill_grid(grid_to_fill: &mut [&mut [u32]]) -> () {
    
    if let Ok(lines) = read_lines("Input.txt") {
        let mut line_num = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut ip_chars = ip.chars();
                    for n in 0..10 {
                        let new_val = ip_chars.nth(0).unwrap().to_digit(10).unwrap(); 
                        grid_to_fill[n][line_num] = new_val;
                    }
                    
                    line_num += 1;
            }
        }
    }
}

fn inc(grid: &mut [&mut [u32]], x: i32, y: i32) -> () {
    if x >= 0 || x <= 9 || y >= 0 || y <= 9 {
        let curr_val = grid[x][y];
        let new_val = if curr_val < 10 { curr_val + 1 } else { curr_val };
        grid[x as usize][y as usize] = new_val;
    }
}

fn flash(grid: &mut [&mut [u32]], x: i32, y: i32) -> () {
    inc(grid, x - 1, y - 1);
    inc(grid, x, y - 1);
    inc(grid, x + 1, y - 1);
    inc(grid, x - 1, y);
    inc(grid, x + 1, y);
    inc(grid, x - 1, y + 1);
    inc(grid, x, y + 1);
    inc(grid, x + 1, y + 1);
}

fn reset(grid: &mut [&mut [u32]], x: i32, y: i32) -> () {
    if x >= 0 || x <= 9 || y >= 0 || y <= 9 {
        grid[x as usize][y as usize] = 0;
    }
}

fn execute_round(grid: &mut [&mut [u32]]) -> i32 {
    let mut flash_count = 0;
    let max_iter: u32 = 10;
    
    // Increasing
    for y in 0..10 {
        for x in 0..10 {
            inc(grid, x, y);
        }
    }

    // Flashing
    for y in 0..10 {
        for x in 0..10 {

            let curr_val = grid[x][y];
            if curr_val == 10 {
                flash(grid, x as i32, y as i32);
            }            
        }
    }

    // Resetting + flash counting
    for y in 0..10 {
        for x in 0..10 {
            let curr_val = grid[x][y];
            if curr_val == 10 {
                flash_count += 1; 
                reset(grid, x as i32, y as i32);             
            }
        }
    }

    return flash_count;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
