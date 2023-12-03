use crate::input;
use std::{collections::HashMap, vec};
use regex::Regex;







pub fn day3() -> input::Result<()> {
    let contents = input::load_day_file("day3.txt");

    // let re = Regex::new(r"[*]").unwrap();

    let mut tot = 0;

    let grid: Vec<Vec<char>> = contents
    .lines()
    .map(|line| line.chars().collect())
    .collect();

    // let mut indices:Vec<Vec<usize>> = vec![];
    // for(row_index, row) in grid.iter().enumerate() {
    //     for mat in re.find_iter(&row.iter().collect::<String>()) {
    //         let index = mat.start();
    //         indices.push(vec![row_index, index]);
    //     }
    // }

    let mut map: HashMap<(usize, usize, usize), u32> = HashMap::new();

    for (row_index, row) in grid.iter().enumerate() {
        let mut start_col = None;
        let mut digits = String::new();
    
        for (col_index, &ch) in row.iter().enumerate() {
            if ch.is_digit(10) {
                digits.push(ch);
                if start_col.is_none() {
                    start_col = Some(col_index);
                }
            } else if !digits.is_empty() {
                let number: u32 = digits.parse().unwrap();
                map.insert((row_index, start_col.unwrap(), col_index - 1), number);
                digits.clear();
                start_col = None;
            }
        }
    
        if !digits.is_empty() {
            let number: u32 = digits.parse().unwrap();
            map.insert((row_index, start_col.unwrap(), row.len() - 1), number);
        }
    }

    let mut gears:HashMap<(usize, usize),Vec<u32>> = HashMap::new();
    //dbg!(&map);
    //dbg!(map.len());
    for (key, value) in map.iter(){
        let mut neighbours: Vec<(usize, usize)> = vec![];
        if key.1 > 0 {
            neighbours.push((key.0, key.1 - 1));
            if key.0 > 0  {
                neighbours.push((key.0 - 1, key.1 - 1));
            }
            if key.0 + 1 < grid.len() {
                neighbours.push((key.0 + 1, key.1 - 1));
            }
        }
        if key.2 + 1 < grid[0].len() {
            neighbours.push((key.0, key.2 + 1));
            if key.0 > 0 {
                neighbours.push((key.0 - 1, key.2 + 1));
            }
            if key.0 + 1 < grid.len(){
                neighbours.push((key.0 + 1, key.2 + 1));
            }
        }
        for i in key.1..=key.2 {
            if key.0 > 0 {
                neighbours.push((key.0 - 1, i));
            }
            if key.0 + 1 < grid.len()  {
                neighbours.push((key.0 + 1, i));
            }
        }
        neighbours.retain(|&c| grid[c.0][c.1] =='*');
        if neighbours.len() !=0{
            //dbg!(value);
            //dbg!(&neighbours);
        }

        if neighbours.len() !=0{
            for neighbour in neighbours.iter(){
                if let Some(values) = gears.get_mut(neighbour) {
                    values.push(*value);
                } else {
                    gears.insert(*neighbour, vec![*value]);
                }
            }
        }
    }
    //dbg!(&gears);
    gears.retain(|_, v| v.len() > 1);
    tot = gears.values().map(|v| v.iter().product::<u32>()).sum::<u32>();
    dbg!(tot);
    Ok(())
}



























pub fn day3part1() -> input::Result<()> {
    let contents = input::load_day_file("day3.txt");

    let re = Regex::new(r"[^a-zA-Z0-9.]").unwrap();

    let mut tot = 0;

    let grid: Vec<Vec<char>> = contents
    .lines()
    .map(|line| line.chars().collect())
    .collect();

    let mut indices:Vec<Vec<usize>> = vec![];
    for(row_index, row) in grid.iter().enumerate() {
        for mat in re.find_iter(&row.iter().collect::<String>()) {
            let index = mat.start();
            indices.push(vec![row_index, index]);
        }
    }

    let mut map: HashMap<(usize, usize, usize), u32> = HashMap::new();

    for (row_index, row) in grid.iter().enumerate() {
        let mut start_col = None;
        let mut digits = String::new();
    
        for (col_index, &ch) in row.iter().enumerate() {
            if ch.is_digit(10) {
                digits.push(ch);
                if start_col.is_none() {
                    start_col = Some(col_index);
                }
            } else if !digits.is_empty() {
                let number: u32 = digits.parse().unwrap();
                map.insert((row_index, start_col.unwrap(), col_index - 1), number);
                digits.clear();
                start_col = None;
            }
        }
    
        if !digits.is_empty() {
            let number: u32 = digits.parse().unwrap();
            map.insert((row_index, start_col.unwrap(), row.len() - 1), number);
        }
    }


    for (key, value) in map.iter(){
        let mut neighbours: Vec<char> = vec![];
        ////dbg!(key);
        if key.1 > 0 {
            neighbours.push(grid[key.0][key.1 - 1]);
            if key.0 > 0 {
                neighbours.push(grid[key.0 - 1][key.1 - 1]);
            }
            if key.0 + 1 < grid.len() {
                neighbours.push(grid[key.0 + 1][key.1 - 1]);
            }
        }
        if key.2 + 1 < grid[0].len() {
            neighbours.push(grid[key.0][key.2 + 1]);
            if key.0 > 0 {
                neighbours.push(grid[key.0 - 1][key.2 + 1]);
            }
            if key.0 + 1 < grid.len() {
                neighbours.push(grid[key.0 + 1][key.2 + 1]);
            }
        }
        for i in key.1..=key.2 {
            if key.0 > 0 {
                neighbours.push(grid[key.0 - 1][i]);
            }
            if key.0 + 1 < grid.len() {
                neighbours.push(grid[key.0 + 1][i]);
            }
        }
        ////dbg!(&neighbours);
        neighbours.retain(|&c| c != '.' && !c.is_digit(10));
        ////dbg!(&neighbours);
        if neighbours.len() != 0 {
            tot += value;
        }
    }

    //dbg!(tot);
    Ok(())
}
