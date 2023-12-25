use crate::input;
use std::collections::HashMap;

pub fn day18() -> input::Result<()> {
    let contents = input::load_day_file("day18.txt");
    let mut pos:(i64,i64) = (0, 0);
    let mut vertex: Vec<(i64, i64)> = Vec::new();
    let mut area:i64 = 0;

    for line in contents.lines() {
        let mut data :Vec<char>= line.split_whitespace().last().unwrap().chars().collect();
        data.retain(|&c| c != '(' && c != ')' && c != '#');
        let dir = data.pop().unwrap();
        //dbg!(&data);
        let hex_string: String = data.into_iter().collect(); // Convert the Vec<char> to a String
        //dbg!(&hex_string);
        let dist:i64 = i64::from_str_radix(&hex_string, 16).unwrap(); // Convert the hexadecimal string to a decimal number
        area += dist as i64;
        match dir.to_digit(10).unwrap() {
            0 => {
                pos.0 += dist;
            }
            2 => {
                pos.0 -= dist;
            }
            1 => {
                pos.1 += dist;
            }
            3 => {
                pos.1 -= dist;
            }
            _ => {
                println!("unknown");
            }
        }
        vertex.push(pos);
    }

    if vertex.last() == vertex.first() {
        vertex.pop();
    }

    for i in 0..vertex.len() {
        ////dbg!(vertex[i], vertex[(i + 1) % vertex.len()]);
        let j = (i + 1) % vertex.len();
        area += (vertex[i].0 * vertex[j].1) as i64;
        area -= (vertex[i].1 * vertex[j].0) as i64;
    }
    area = area / 2;
    area += 1;
    dbg!(area);
    Ok(())
}




// pub fn day18part1() -> input::Result<()> {
//     let contents = input::load_day_file("day18.txt");
//     let mut pos = (0, 0);
//     let mut vertex: Vec<(i64, i64)> = Vec::new();
//     let mut area = 0.0;

//     for line in contents.lines() {
//         let mut split = line.split_whitespace();
//         let dir = split.next().unwrap();
//         let dist = split.next().unwrap().parse::<i64>().unwrap();
//         area += dist as i64;
//         match dir {
//             "R" => {
//                 pos.0 += dist;
//             }
//             "L" => {
//                 pos.0 -= dist;
//             }
//             "D" => {
//                 pos.1 += dist;
//             }
//             "U" => {
//                 pos.1 -= dist;
//             }
//             _ => {
//                 println!("unknown");
//             }
//         }
//         vertex.push(pos);
//     }

//     if vertex.last() == vertex.first() {
//         vertex.pop();
//     }

//     for i in 0..vertex.len() {
//         ////dbg!(vertex[i], vertex[(i + 1) % vertex.len()]);
//         let j = (i + 1) % vertex.len();
//         area += (vertex[i].0 * vertex[j].1) as i64;
//         area -= (vertex[i].1 * vertex[j].0) as i64;
//     }
//     area = area.abs() / 2.0;
//     area += 1.0;
//     dbg!(area);
//     Ok(())
// }
