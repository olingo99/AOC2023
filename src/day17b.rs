use crate::input;
use std::collections::HashMap;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::env::current_exe;
use std::iter::Successors;

use crate::Matrix;

#[derive(Copy, Clone, Eq, Hash)]
struct Node {
    position: (usize, usize),
    f_score: usize,
    last_direction: Option<char>,
    dir_count:usize
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

pub fn day17() -> input::Result<()> {
    let contents = input::load_day_file("day17.txt");
    let matrix : Matrix<i32> = Matrix::parse_input(&contents);
    dbg!(&matrix);
    solve_part1(matrix);
    Ok(())
}

fn solve_part1(matrix:Matrix<i32>){
    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    let mut close_list: HashSet<(usize,usize)> = HashSet::new();
    let mut g_score: HashMap<(usize,usize), usize> = HashMap::new();
    let mut came_from: HashMap<(usize,usize), (usize,usize)> = HashMap::new();

    let start = Node{position:(0, 0), f_score:0, last_direction:None, dir_count:0};
    let goal = Node{position:(matrix.cols-1, matrix.rows-1), f_score:0,last_direction:None, dir_count:0};

    open_list.push(start);
    g_score.insert(start.position, 0);

    while let Some(current) = open_list.pop(){

        if current == goal{
            let path = reconstruct_path(&matrix,came_from, current.position);
            let mut m2 = matrix.clone();
            for &(x, y) in &path {
                m2.matrix[y][x] =0;
            }
            dbg!(m2);

            return
        }

        close_list.insert(current.position);

        let successors = get_neighbours(current, matrix.rows, matrix.cols);

        for (mut successor, direction) in successors{

            if close_list.contains(&successor.position)
            {
                continue;
            }

            let spos = successor.position;

            let temp_g_score = g_score[&current.position] + matrix.matrix[spos.1][spos.0] as usize;

            if !g_score.contains_key(&spos) || temp_g_score < g_score[&spos] {

                if current.last_direction == Some(direction){
                   if current.dir_count>=3 {
                        continue;
                    }
                    successor.dir_count = current.dir_count+1;
                }
                else{
                    successor.dir_count = 1;
                }
                successor.last_direction = Some(direction);

                came_from.insert(spos, current.position);
                g_score.insert(spos, temp_g_score);
                let temp_f_score = temp_g_score + heuristic(spos, goal.position);
                // f_score.insert(spos, temp_f_score);

                successor.f_score = temp_f_score;

            
                open_list.push(successor);
            }

        }
    }

}

fn get_neighbours(node: Node, rows: usize, cols: usize) -> Vec<(Node, char)> {
    let (sx, sy) = node.position;
    let mut neighbours = Vec::new();

    if sx > 0 {
        neighbours.push((
            Node {
                position: (sx - 1, sy),
                f_score: 0,
                last_direction: node.last_direction,
                dir_count: node.dir_count,
            },
            'L',
        ));
    }
    if sx < cols - 1 {
        neighbours.push((
            Node {
                position: (sx + 1, sy),
                f_score: 0,
                last_direction: node.last_direction,
                dir_count: node.dir_count,
            },
            'R',
        ));
    }
    if sy > 0 {
        neighbours.push((
            Node {
                position: (sx, sy - 1),
                f_score: 0,
                last_direction: node.last_direction,
                dir_count: node.dir_count,
            },
            'U',
        ));
    }
    if sy < rows - 1 {
        neighbours.push((
            Node {
                position: (sx, sy + 1),
                f_score: 0,
                last_direction: node.last_direction,
                dir_count: node.dir_count,
            },
            'D',
        ));
    }

    neighbours
}
fn heuristic(node: (usize,usize), goal:(usize,usize))->usize{
//    (node.0 as isize - goal.0 as isize).abs() as usize + (node.1 as isize - goal.1 as isize).abs() as usize
    0
}

fn reconstruct_path(matrix:&Matrix<i32>, came_from: HashMap<(usize,usize), (usize,usize)>, mut current:  (usize,usize))->Vec<(usize, usize)>{
    let mut total_path = vec![current];
    let mut cost = 0;
    while let Some(&parent) = came_from.get(&current) {
        current = parent;
        total_path.push(current);
        cost += matrix.matrix[current.1][current.0];
    }
    dbg!(cost);
    total_path.reverse();
    total_path
}