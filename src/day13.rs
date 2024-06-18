use itertools::Itertools;

use crate::input;
use std::{collections::HashMap, mem, usize};

#[derive(Clone)]
struct Matrix
{
    matrix: Vec<Vec<char>>,
}
impl Matrix {
    fn transpose(&self) -> Self {
        let rows = self.matrix.len();
        let cols = self.matrix[0].len();
        let mut transposed = vec![vec![' '; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                transposed[j][i] = self.matrix[i][j];
            }
        }
        Self { matrix: transposed }
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

pub fn day13() -> input::Result<()> {
    let contents = input::load_day_file("day13.txt");
    let matrixes: Vec<Matrix> = parse_input(&contents);
    let total = solve_part1(&matrixes);
    // dbg!(&matrixes[0]);
    println!("Total number: {}", total);

    Ok(())
}

fn parse_input(contents: &str) -> Vec<Matrix> {
    let mut matrixes = Vec::new();
    let mut matrix = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            matrixes.push(Matrix { matrix });
            matrix = Vec::new();
            continue;
        }
        matrix.push(line.chars().collect());
    }
    matrixes.push(Matrix { matrix });
    matrixes
}

fn solve_part1(matrixes: &Vec<Matrix>) -> usize {
    let mut total = 0;
    for matrix in matrixes {
        total += solve_matrix(&matrix, false);
        // dbg!(total);
    }
    total
}

fn solve_matrix(matrix: &Matrix, rec:bool) -> usize {
    let mut total = 0;
    let nb_rows = matrix.matrix.len();
    for index in 0..nb_rows{
        let mut flag = true;
        for i in 0..index+1 {
            let target = index*2+1-i;
            if i >= nb_rows || target >= nb_rows {
                continue;
            }
            if matrix.matrix[i] != matrix.matrix[target]
            {
                flag = false;
                break;
            }
        }
        if flag && index != nb_rows-1
        {
            // dbg!(index,rec,1*(index+1));
            total += 100*(index+1);
        }
    }
    if !rec
    {
        total += solve_matrix(&matrix.transpose(), true)/100;

    }
    total
}