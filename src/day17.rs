use crate::input;
use std::collections::HashMap;
use std::hash::Hash;
use crate::Matrix;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, Hash, Debug)]
struct Node{
    row: isize,
    column: isize,
    cost: isize,
    dir: (isize,isize)
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}


pub fn day17() -> input::Result<()> {
    let contents = input::load_day_file("day17.txt");
    let grid : Matrix<i32> = Matrix::parse_input(&contents);
    solve_part2(grid);

    Ok(())
}

fn solve_part1(grid:Matrix<i32>){
    dijkstra(grid, 1, 3);
}

fn solve_part2(grid:Matrix<i32>){
    dijkstra(grid, 4, 10);
}

fn dijkstra(grid:Matrix<i32>, minstep:isize,maxstep:isize){
    let mut q = BinaryHeap::new();
    let mut dists = HashMap::new();

    let start = Node{
        row:0,
        column:0,
        cost:0,
        dir: (0,0)
    };

    let goal = Node{
        row:grid.rows as isize-1,
        column:grid.cols as isize -1,
        cost:0,
        dir: (0,0)
    };

    q.push(start);

    while let Some(current) = q.pop(){

        if current == goal{
            dbg!(current.cost);
            return;
        }

        if dists.get(&(current.row, current.column, current.dir)).is_some_and(|&c| current.cost > c){
            continue
        }

        for (dr, dc) in [(-1,0),(1,0),(0,-1),(0,1)] {
            if current.dir == (dr,dc) || current.dir ==  (-dr,-dc){
                continue;
            }

            let mut next_cost = current.cost;

            for dist in 1..=maxstep{
                
                let next_row = current.row + dr * dist;

                let next_column = current.column + dc * dist;

                if next_row >= grid.rows as isize || next_row < 0 || next_column >= grid.cols as isize || next_column < 0 {
                    break;
                }

                next_cost += grid.matrix[next_row as usize][next_column as usize] as isize;

                if dist < minstep {
                    continue;
                }

                let key = (next_row, next_column, (dr,dc));;

                if next_cost < *dists.get(&key).unwrap_or(&isize::MAX){
                    dists.insert(key, next_cost);
                    q.push(Node{
                        row: next_row,
                        column:next_column,
                        cost:next_cost,
                        dir:(dr,dc)
                    })
                }

            }
            
        }

    }
}
