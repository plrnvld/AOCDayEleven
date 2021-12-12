use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use colored::*;

const SIDE: usize = 10;
const ROUNDS: usize = 500;

fn main() {
    let mut grid_raw = vec![0; SIDE * SIDE];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(SIDE).collect();
    let grid = grid_base.as_mut_slice();

    fill_grid(grid);

    print_grid(grid, "Grid before any steps");

    let mut total_flashes = 0;
    for round_minus_one in 0..ROUNDS {
        let step = round_minus_one + 1;
        total_flashes += execute_round(grid, step);
        
        let mut owned = "After step ".to_owned();
        owned.push_str(&(step).to_string());
        // print_grid_string(grid, owned);

        let mut all_zero = true;
        for y in 0..SIDE {
            for x in 0..SIDE {
                if grid[x][y] != 0 {
                    all_zero = false;
                }
            }
        }

        if all_zero {
            println!("Everthing zero after step {}", step)
        }
    }

    println!("Total flashes {}", total_flashes);
}

fn fill_grid(grid_to_fill: &mut [&mut [u32]]) -> () {
    if let Ok(lines) = read_lines("Input.txt") {

        let mut line_num = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut ip_chars = ip.chars();
                    for n in 0..SIDE {
                        let new_val = ip_chars.nth(0).unwrap().to_digit(10).unwrap(); 
                        grid_to_fill[n][line_num] = new_val;
                    }
                    
                    line_num += 1;
            }
        }
    }
}

fn inc(grid: &mut [&mut [u32]], x: usize, y: usize) -> bool {
    if x < SIDE && y < SIDE {
        let curr_val = grid[x][y];
        let new_val = if curr_val < 11 { curr_val + 1 } else { curr_val };
        grid[x][y] = new_val;

        return curr_val == 9;
    }
    else {
        return false;
    }
}

fn inc_for_flash(grid: &mut [&mut [u32]], x: usize, y: usize) -> bool {
    if x < SIDE && y < SIDE {
        let curr_val = grid[x][y];
        let new_val = if curr_val < 10 { curr_val + 1 } else { curr_val };
        grid[x][y] = new_val;

        return curr_val == 9;
    }
    else {
        return false;
    }
}

fn flash(grid: &mut [&mut [u32]], x: usize, y: usize) -> bool {
    // println!("  Flashing for {},{}", x, y);
    
    set(grid, x, y, 11);
      
    let mut reflash = false;
    
    let flash_pos = vec![(-1i8, -1i8), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    for pos in flash_pos {
        if !(pos.0 < 0 && x == 0) && !(pos.0 > 0 && x > SIDE - 1) {
            if !(pos.1 < 0 && y == 0) && !(pos.1 > 0 && y > SIDE - 1) {
                let pos_x_to_inc = (x as i8 + pos.0) as usize;
                let pos_y_to_inc = (y as i8 + pos.1) as usize;

                if inc_for_flash(grid, pos_x_to_inc, pos_y_to_inc) {
                    reflash = true;
                }

                if pos_x_to_inc == 2 && pos_y_to_inc == 2 {
                    // println!("    increasing 2,2 because of {},{} to {}", x, y, grid[2][2]);
                }
            }
        }
    }

    return reflash;
}

fn reset(grid: &mut [&mut [u32]], x: usize, y: usize) -> () {
    set(grid, x, y, 0);
}

fn set(grid: &mut [&mut [u32]], x: usize, y: usize, val: u32) -> () {
    grid[x as usize][y as usize] = val;
}

fn print_grid_string(grid: &mut [&mut [u32]], title: String) -> () {
    print_grid(grid, &title);
}

fn print_grid(grid: &mut [&mut [u32]], title: &str) -> () {
    println!("{}", title);
    for y in 0..SIDE {
        for x in 0..SIDE {
            let cell = grid[x][y];
            let display = if cell == 10 { 
                "x".to_string().color("red") 
                } else if cell == 11 { 
                    "T".to_string().color("green")  
                } else if cell == 9 { 
                    cell.to_string().color("blue") 
                } else if cell == 0 { 
                    cell.to_string().color("cyan") 
                } else { 
                    cell.to_string().color("white") 
                };

            print!("{}", display);            
        }

        println!();
    }
    
    println!();
}

fn flash_grid(grid: &mut [&mut [u32]], step: usize) -> bool {
    let mut reflash = false;
    let mut to_flash: Vec<(usize, usize)> = Vec::new();
    
    for y in 0..SIDE {
        for x in 0..SIDE {
            let curr_val = grid[x][y];
            if curr_val == 10 {
                to_flash.push((x, y));  
            }            
        }
    }

    if to_flash.len() == SIDE * SIDE {
        println!("Everything flashing at step {}!.", step);
    }

    for pos in to_flash {
        if flash(grid, pos.0, pos.1) {
            reflash = true;
        }
    }

    return reflash;
}

fn execute_round(grid: &mut [&mut [u32]], step: usize) -> i32 {
    let mut flash_count = 0;
    
    // Increasing
    for y in 0..SIDE {
        for x in 0..SIDE {
            inc(grid, x, y);
        }
    }

    // print_grid(grid, "After increasing");

    // Flashing
    let mut keep_flashing = true;
    while keep_flashing {
        keep_flashing = flash_grid(grid, step);
        // println!("FLASH");
    }
        

    // print_grid(grid, "After flashing neighbors");   

    // Resetting + flash counting
    for y in 0..SIDE {
        for x in 0..SIDE {
            let curr_val = grid[x][y];
            if curr_val == 11 {
                flash_count += 1; 
                reset(grid, x, y);             
            }
        }
    }

    // print_grid(grid, "After resetting"); 

    return flash_count;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
