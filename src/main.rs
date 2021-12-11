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

    println!("Hello!");
    println!("{}", grid[0][0]);
    println!("{}", grid[1][0]);
    println!("{}", grid[2][0]);
    println!();

    println!("{}", grid[9][6]);
    println!("{}", grid[9][7]);
    println!("{}", grid[9][8]);
    println!("{}", grid[9][9]);
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
