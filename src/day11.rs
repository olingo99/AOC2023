use crate::input;
use std::collections::HashMap;
use itertools::Itertools;

pub fn day11() -> input::Result<()> {
    let contents = input::load_day_file("day11.txt");
    let mut grid : Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    // dbg!(&grid);
    let mut indicesRow = Vec::new();
    let mut indicesCol = Vec::new();
    for (index, line) in grid.iter().enumerate(){
        if line.iter().all(|c| c == &'.'){
            indicesRow.push(index);
        }
    }
    let num_columns = grid[0].len();
    let mut col: Vec<char> = Vec::new();
    let mut galaxies:Vec<(usize, usize)> = Vec::new();
    for j in 0..num_columns {
        col.clear();
        for i in 0..grid.len() {
            let cell = grid[i][j];
            if cell == '#'{ 
                galaxies.push((i,j));
            }
            col.push(cell)
            // Do something with cell
        }
        if col.iter().all(|c| c == &'.'){
            indicesCol.push(j);
        }
    }
    // dbg!(&galaxies);
    // for i in 0..galaxies.len() {
    //     for j in i+1..galaxies.len() {
    //         let galaxy1 = galaxies[i];
    //         let galaxy2 = galaxies[j];
    //         // Do something with galaxy1 and galaxy2
    //     }
    // }
    let mut tot = 0;
    for pair in galaxies.iter().combinations(2) {
        let galaxy1 = pair[0];
        let galaxy2 = pair[1];
        let mut dist = (galaxy1.0 as i64 - galaxy2.0 as i64).abs() + (galaxy1.1 as i64- galaxy2.1 as i64).abs();
        if galaxy1.0>galaxy2.0{
            for index in galaxy2.0..galaxy1.0 {
                if indicesRow.contains(&index){
                    dist +=999999;
                }
            }
        }
        else{
            for index in galaxy1.0..galaxy2.0 {
                if indicesRow.contains(&index){
                    dist +=999999;
                }
            }
        }
        if galaxy1.1>galaxy2.1{
            for index in galaxy2.1..galaxy1.1 {
                if indicesCol.contains(&index){
                    dist +=999999;
                }
            }
        }
        else{
            for index in galaxy1.1..galaxy2.1 {
                if indicesCol.contains(&index){
                    dist +=999999;
                }
            }
        }
        tot += dist;
        // dbg!(galaxy1, galaxy2);
        // dbg!(dist);
    }

    // dbg!(indicesRow);
    // dbg!(indicesCol);
    // dbg!(galaxies);

    dbg!(tot);
    Ok(())
}
