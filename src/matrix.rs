use std::fs;
use std::path::PathBuf;
use std::time::Instant;

pub trait FromChar {
    fn from_char(c: char) -> Self;
}

impl FromChar for i32 {
    fn from_char(c: char) -> Self {
        c.to_digit(10).unwrap() as i32
    }
}

impl FromChar for char {
    fn from_char(c: char) -> Self {
        c
    }
}

#[derive(Clone)]
pub struct Matrix<T> {
    pub matrix: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: FromChar> Matrix<T> {
    pub fn parse_input(contents: &str) -> Matrix<T> {
        let mut matrix: Vec<Vec<T>> = Vec::new();
        for line in contents.lines() {
            let row: Vec<T> = line.chars().map(T::from_char).collect();
            matrix.push(row);
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        Matrix { matrix, rows, cols }
    }
}

use std::fmt;

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in &self.matrix {
            for item in row {
                write!(f, "{:?} ", item)?; // Use {:?} for Debug formatting
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in &self.matrix {
            for item in row {
                write!(f, "{} ", item)?; // Use {} for Display formatting
            }
            writeln!(f)?;
        }
        Ok(())
    }
}