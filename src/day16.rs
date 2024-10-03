use crate::input;
use crate::matrix;
use std::collections::HashMap;
use std::collections::HashSet;

use matrix::Matrix;

use once_cell::sync::Lazy;

enum BeamAction {
    Add(Beam),
    Remove,
    None,
    Error,
}

static DIR: Lazy<HashMap<char, Vec<i32>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert('R', vec![1, 0]);
    m.insert('L', vec![-1, 0]);
    m.insert('D', vec![0, 1]);
    m.insert('U', vec![0, -1]);
    m
});

static DIRMIR: Lazy<HashMap<Vec<char>, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(vec!['R', '/'], 'U');
    m.insert(vec!['L', '/'], 'D');
    m.insert(vec!['D', '/'], 'L');
    m.insert(vec!['U', '/'], 'R');
    m.insert(vec!['R', '\\'], 'D');
    m.insert(vec!['L', '\\'], 'U');
    m.insert(vec!['D', '\\'], 'R');
    m.insert(vec!['U', '\\'], 'L');
    m
});

fn print_dirmir() {
    for (key, value) in DIRMIR.iter() {
        let key_str: String = key.iter().collect();
        println!("Key: {}, Value: {:?}", key_str, value);
    }
}

#[derive(Debug)]
struct Beam {
    direction: char,
    x: i32,
    y: i32,
}

impl Beam {
    fn step(
        &mut self,
        matrix: &Matrix,
        energized_cells: &mut HashSet<(i32, i32)>,
        walked: &mut HashSet<(i32, i32, char)>,
    ) -> BeamAction {
        let change = &DIR[&self.direction];
        //dbg!("step");
        //dbg!(self.direction, self.x, self.y, &change);
        let newx = self.x + change[0];
        let newy = self.y + change[1];

        if newx >= matrix.cols as i32
            || newx < 0
            || newy >= matrix.rows as i32
            || newy < 0
            || walked.contains(&(newx, newy, self.direction))
        {
            return BeamAction::Remove;
        }

        walked.insert((newx, newy, self.direction));

        self.x = newx;
        self.y = newy;

        energized_cells.insert((self.x, self.y));

        //dbg!(self.x, self.y);

        let tile: char = matrix.matrix[self.y as usize][self.x as usize];
        let mut matrix2 = matrix.clone();
        matrix2.matrix[self.y as usize][self.x as usize] = self.direction;
        //dbg!(matrix2);

        if tile == '/' || tile == '\\' {
            self.direction = DIRMIR[&vec![self.direction, tile]];
            return BeamAction::None;
        }

        if tile == '.'
            || (tile == '-' && (self.direction == 'L' || self.direction == 'R'))
            || (tile == '|' && (self.direction == 'U' || self.direction == 'D'))
        {
            return BeamAction::None;
        }

        if tile == '-' && (self.direction == 'U' || self.direction == 'D') {
            self.direction = 'L';
            let new_beam = Beam {
                direction: 'R',
                x: self.x,
                y: self.y,
            };
            return BeamAction::Add(new_beam);
        }

        if tile == '|' && (self.direction == 'L' || self.direction == 'R') {
            self.direction = 'U';
            let new_beam = Beam {
                direction: 'D',
                x: self.x,
                y: self.y,
            };
            return BeamAction::Add(new_beam);
        }

        return BeamAction::Error;
    }
}

pub fn day16() -> input::Result<()> {
    let contents = input::load_day_file("day16.txt");
    let mut matrix: Matrix = Matrix::parse_input(&contents);

    //dbg!(&matrix);
    print_dirmir();

    solve_part2(matrix);

    Ok(())
}

fn solve_part1(matrix: Matrix) {
    let mut energized_cells: HashSet<(i32, i32)> = HashSet::new();
    let mut walked: HashSet<(i32, i32, char)> = HashSet::new();
    let mut beamVec: Vec<Beam> = Vec::new();
    beamVec.push(Beam {
        direction: 'R',
        x: -1,
        y: 0,
    });
    while !&beamVec.is_empty() {
        //dbg!(&beamVec.len());
        let mut new_beams: Vec<Beam> = Vec::new();
        beamVec.retain_mut(|beam| {
            match beam.step(&matrix, &mut energized_cells, &mut walked) {
                BeamAction::Add(new_beam) => {
                    new_beams.push(new_beam);
                    true
                }
                BeamAction::Remove => false,
                BeamAction::None => true,
                BeamAction::Error => {
                    //dbg!("aled error");
                    true
                }
            }
        });
        beamVec.extend(new_beams);
    }

    dbg!(energized_cells.len());
    // let mut matrix3 = matrix.clone();
    // for elem in energized_cells{
    //     matrix3.matrix[elem.1 as usize][elem.0 as usize] = '#';
    // }
    // dbg!(matrix3);
}

fn solve_part2(matrix: Matrix) {

    let mut energylevels :Vec<u32> = Vec::new();
    let starterBeams = generate_starting_beam(&matrix);
    for beam in starterBeams{
        let mut beamVec: Vec<Beam> = Vec::new();
        let mut energized_cells: HashSet<(i32, i32)> = HashSet::new();
        let mut walked: HashSet<(i32, i32, char)> = HashSet::new();
        beamVec.push(beam);
        while !&beamVec.is_empty() {
            let mut new_beams: Vec<Beam> = Vec::new();
            beamVec.retain_mut(
                |beam| match beam.step(&matrix, &mut energized_cells, &mut walked) {
                    BeamAction::Add(new_beam) => {
                        new_beams.push(new_beam);
                        true
                    }
                    BeamAction::Remove => false,
                    BeamAction::None => true,
                    BeamAction::Error => true,
                },
            );
            beamVec.extend(new_beams);
        }
        // dbg!(energized_cells.len() as u32);
        energylevels.push(energized_cells.len() as u32);
    }

    dbg!(energylevels.iter().max());
}

fn generate_starting_beam(matrix: &Matrix) -> Vec<Beam>{
    let mut starterBeams : Vec<Beam> = Vec::new();
    
    for i in 0..matrix.cols as i32{
        starterBeams.push(Beam { direction: 'D', x: i, y: -1 });
        starterBeams.push(Beam { direction: 'U', x: i, y: matrix.rows as i32});
    }

    for j in 0..matrix.rows as i32{
        starterBeams.push(Beam { direction: 'R', x: -1, y: j });
        starterBeams.push(Beam { direction: 'L', x: matrix.cols as i32, y: j});
    }
    // dbg!(&starterBeams);
    // dbg!(&starterBeams.len());

    return starterBeams;
}
