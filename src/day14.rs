use itertools::Itertools;

use crate::input;
use std::{collections::HashMap, mem, usize};

#[derive(Clone)]
struct Matrix
{
    matrix: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}
impl Matrix {

    fn spin_cycle(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                // self.go_north(row, col);
                self.go_south(row, col);
                // self.go_east(row, col);
                // self.go_west(row, col);
            }
        }
    }

    fn go_north(&mut self, mut row: usize, col: usize) -> usize {
        if self.matrix[row][col] != 'O' {
            return 0;
        }
        
        while row >= 1 && self.matrix[row-1][col] == '.' {
            self.matrix[row][col] = '.';
            self.matrix[row-1][col] = 'O';
            row -= 1;
        }

        return self.rows - row;
    }

    fn go_south(&mut self, mut row: usize, col: usize){
        if self.matrix[row][col] != 'O' {
            return;
        }
        
        while row < self.rows - 1 && self.matrix[row+1][col] == '.' {
            self.matrix[row][col] = '.';
            self.matrix[row+1][col] = 'O';
            row += 1;
        }
    }

    fn go_east(&mut self, row: usize, mut col: usize){
        if self.matrix[row][col] != 'O' {
            return;
        }
        
        while col < self.cols - 1 && self.matrix[row][col+1] == '.' {
            self.matrix[row][col] = '.';
            self.matrix[row][col+1] = 'O';
            col += 1;
        }
    }

    fn go_west(&mut self, row: usize, mut col: usize){
        if self.matrix[row][col] != 'O' {
            return;
        }
        
        while col >= 1 && self.matrix[row][col-1] == '.' {
            self.matrix[row][col] = '.';
            self.matrix[row][col-1] = 'O';
            col -= 1;
        }
    }

    fn north_load(&self)->usize{
        let mut count = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.matrix[row][col] == 'O' {
                    count += self.rows - row;
                }
            }
        }
        count
    }
}

use std::fmt;

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in &self.matrix {
            for &item in row {
                write!(f, "{} ", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in &self.matrix {
            for &item in row {
                write!(f, "{} ", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn day14() -> input::Result<()> {
    let contents = input::load_day_file("day14.txt");
    let matrix: Matrix = parse_input(&contents);
    let total = solve_part2(matrix);
    println!("Total number: {}", total);

    Ok(())
}

fn parse_input(contents: &str) -> Matrix {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        matrix.push(line.chars().collect());
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    Matrix { matrix, rows, cols }
}

fn solve_part1(mut matrix: Matrix) -> usize {
    let mut total = 0;
    for row in 0..matrix.rows {
        for col in 0..matrix.cols {
            if matrix.matrix[row][col] == 'O' {
                total += matrix.go_north(row, col);
            }
        }
    }
    total
}

fn solve_part2(mut matrix: Matrix) -> usize {
    for _ in 0..1{
        matrix.spin_cycle();
        dbg!(matrix.clone());
    }
    matrix.north_load()
}




















// fn spin_cycle(&mut self) {
//     let mut new_matrix = self.matrix.clone();
//     for row in 0..self.rows {
//         for col in 0..self.cols {
//             let mut adjacent = HashMap::new();
//             for i in -1..=1 {
//                 for j in -1..=1 {
//                     if i == 0 && j == 0 {
//                         continue;
//                     }
//                     let new_row = row as i32 + i;
//                     let new_col = col as i32 + j;
//                     if new_row >= 0 && new_row < self.rows as i32 && new_col >= 0 && new_col < self.cols as i32 {
//                         let count = adjacent.entry(self.matrix[new_row as usize][new_col as usize]).or_insert(0);
//                         *count += 1;
//                     }
//                 }
//             }
//             let count = adjacent.entry('O').or_insert(0);
//             if self.matrix[row][col] == 'O' {
//                 if *count == 2 || *count == 3 {
//                     new_matrix[row][col] = 'O';
//                 } else {
//                     new_matrix[row][col] = '.';
//                 }
//             } else {
//                 if *count == 3 {
//                     new_matrix[row][col] = 'O';
//                 } else {
//                     new_matrix[row][col] = '.';
//                 }
//             }
//         }
//     }
//     self.matrix = new_matrix;
// }