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


    let mut startdirs = Vec::new();

    let mut neighbor:char;
    for (dir, (x, y)) in mapDir.iter(){
        println!("{}: {}, {}", dir, x, y);
        dbg!(grid[(start.0 as i32 + x) as usize][(start.1 as i32 + y) as usize]);
        neighbor = grid[(start.0 as i32 + x) as usize][(start.1 as i32 + y) as usize];
        if neighbor != '.' &&  (*dir == pipe2dir.get(&neighbor).unwrap().0 || *dir == pipe2dir.get(&neighbor).unwrap().1){
            println!("found");
            dbg!(&neighbor);
            startdirs.push(*dir);
        }
    }
    dbg!(&startdirs);
    let mut poss = Vec::new();
    poss.push(start.clone());
    poss.push(start.clone());
    loop{
        for (pos, startdir) in poss.iter_mut().zip(&startdirs) {
            pos.0 += mapDir.get(startdir).unwrap().0 as usize;
            pos.1 += mapDir.get(startdir).unwrap().1 as usize;
        }
        if &poss[0] != &poss[1]{break;}
        break;
    }
    dbg!(poss);
    Ok(())
}
