use crate::{input, matrix::Matrix};
use std::collections::{HashMap, HashSet};

pub fn day21() -> input::Result<()> {
    let contents = input::load_day_file("day21.txt");
    let matrix :Matrix<char>= Matrix::parse_input(&contents);
    solve_part1(matrix);
    Ok(())
}

fn solve_part1(mut grid:Matrix<char>){
    // dbg!(&grid);
    let mut start: (usize, usize) = (0,0);
    for (i, y) in grid.matrix.iter().enumerate() {
        for (j, x) in y.iter().enumerate() {
            if *x == 'S' {
                start = (j, i);
            }
        }
    }

    grid.matrix[start.1][start.0] = 'O';

    // dbg!(&grid);
    let mut modifications: HashSet<(usize,usize)> = HashSet::new();
    for _ in 0..202{
        for (i, y) in grid.matrix.iter().enumerate() {
            for (j, x) in y.iter().enumerate() {
                if *x == 'O' {
                    modifications.insert((j,i));
                    if j+1< grid.cols && grid.matrix[i][j+1]!='#'{
                        modifications.insert((j+1,i));
                    }
                    if j>0 && grid.matrix[i][j-1]!='#'{
                        modifications.insert((j-1,i));
                    }
                    if i+1< grid.rows && grid.matrix[i+1][j]!='#'{
                        modifications.insert((j,i+1));
                    }
                    if i>0 && grid.matrix[i-1][j]!='#'{
                        modifications.insert((j,i-1));
                    }

                }
            }
        }

        for (x,y) in &modifications{
            if grid.matrix[*y][*x] == '.' {
                grid.matrix[*y][*x] = 'O';
            } else if grid.matrix[*y][*x] == 'O' {
                grid.matrix[*y][*x] = '.';
            }
        }
    }
    // dbg!(&grid);
    let mut c = 0;
    for (i, y) in grid.matrix.iter().enumerate() {
        for (j, x) in y.iter().enumerate() {
            if *x == 'O' {
                c+=1;
            }
        }
    }
    dbg!(c);

}
