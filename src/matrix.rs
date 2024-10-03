use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Clone)]
pub struct Matrix
{
    pub matrix: Vec<Vec<char>>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix{
    pub fn parse_input(contents: &str) -> Matrix {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        for line in contents.lines() {
            matrix.push(line.chars().collect());
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        Matrix { matrix, rows, cols }
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