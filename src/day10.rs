use crate::input;
use std::collections::HashMap;

pub fn day10() -> input::Result<()> {
    let contents = input::load_day_file("day10.txt");
    let mut pipe2dir = HashMap::new();
    pipe2dir.insert('|', ('N', 'S'));
    pipe2dir.insert('-', ('E', 'W'));
    pipe2dir.insert('L', ('N', 'E'));
    pipe2dir.insert('7', ('S', 'W'));
    pipe2dir.insert('J', ('N', 'W'));
    pipe2dir.insert('F', ('S', 'E'));

    let mut grid = Vec::new();

    for line in contents.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    let mut start = (0, 0);

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 'S' {
                start = (x, y);
                break;
            }
        }
    };

    let mut mapDir: HashMap<char,(i32,i32)> = HashMap::new();
    mapDir.insert('N', (-1, 0));
    mapDir.insert('S', (1, 0));
    mapDir.insert('E', (0, 1));
    mapDir.insert('W', (0, -1));

    let mut inverseDir: HashMap<char,char> = HashMap::new();
    inverseDir.insert('N', 'S');
    inverseDir.insert('S', 'N');
    inverseDir.insert('E', 'W');
    inverseDir.insert('W', 'E');


    let mut startdirs: Vec<char> = Vec::new();

    let mut neighbor:char;
    for (dir, (x, y)) in mapDir.iter(){
        neighbor = grid[(start.0 as i32 + x) as usize][(start.1 as i32 + y) as usize];
        ////dbg!(neighbor, dir);
        if neighbor != '.'{
            ////dbg!(pipe2dir.get(&neighbor).unwrap().0, pipe2dir.get(&neighbor).unwrap().1);
        }
        if neighbor != '.' &&  (*inverseDir.get(dir).unwrap() == pipe2dir.get(&neighbor).unwrap().0 || *inverseDir.get(dir).unwrap() == pipe2dir.get(&neighbor).unwrap().1){
            ////dbg!("ok");
            startdirs.push(*dir);
        }
    }
    let mut poss = Vec::new();
    poss.push(start.clone());
    poss.push(start.clone());
    let mut dist = 0;
    let mut dir_history = startdirs.clone();
    let mut vertex1:Vec<(usize, usize)> = Vec::new();
    let mut vertex2:Vec<(usize, usize)> = Vec::new();
    vertex1.push(start.clone());
    loop{
        ////dbg!(&poss, &startdirs);
        for (pos, startdir) in poss.iter_mut().zip(&startdirs) {
            ////dbg!(&pos);
            // pos.0 += mapDir.get(startdir).unwrap().0 as usize;
            // pos.1 += mapDir.get(startdir).unwrap().1 as usize;
            pos.0 = (pos.0 as i32 + mapDir.get(startdir).unwrap().0) as usize;
            pos.1 = (pos.1 as i32 + mapDir.get(startdir).unwrap().1) as usize;
        }
        let pipe1 = grid[poss[0].0][poss[0].1];
        let pipe2 = grid[poss[1].0][poss[1].1];
        ////dbg!(pipe1, pipe2);
        let dir1 = pipe2dir.get(&pipe1).unwrap();
        let dir2 = pipe2dir.get(&pipe2).unwrap();
        let dir1 = if dir1.0 != *inverseDir.get(&startdirs[0]).unwrap() { dir1.0 } else { dir1.1 };
        let dir2 = if dir2.0 != *inverseDir.get(&startdirs[1]).unwrap() { dir2.0 } else { dir2.1 };
        if dir1 != dir_history[0]{
            dir_history[0] = dir1;
            vertex1.push(poss[0].clone());
        }
        if dir2 != dir_history[1]{
            dir_history[1] = dir2;
            vertex2.push(poss[1].clone());
        }
        startdirs.clear();
        startdirs.push(dir1);
        startdirs.push(dir2);
        dist+=1;
        if &poss[0] == &poss[1]{break;}

    }
    //dbg!(&vertex1, &vertex2);
    // vertex2.pop();
    vertex2.reverse();
    let mut vertex = vertex1.clone();
    vertex.append(&mut vertex2);
    vertex.reverse();
    //dbg!(&vertex);
    let mut area = 0.0;
    for i in 0..vertex.len() {
        //dbg!(vertex[i], vertex[(i + 1) % vertex.len()]);
        let j = (i + 1) % vertex.len();
        area += (vertex[i].0 * vertex[j].1) as f64;
        area -= (vertex[i].1 * vertex[j].0) as f64;
    }
    area = area.abs() / 2.0;

    //dbg!(area);
    // let b = vertex.len()-1; // Number of boundary points
    let b = dist*2;
    let i = area + 1.0 - (b as f64 / 2.0);
    dbg!(b,i);


    // //dbg!(vertex);
    ////dbg!(poss);
    //dbg!(dist);
    Ok(())
}
