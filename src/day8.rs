use crate::input;
use std::collections::HashMap;
use num::integer::lcm;

pub fn day8() -> input::Result<()> {
    let contents = input::load_day_file("day8.txt");
    let instructions = contents.lines().nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in contents.lines().skip(2) {
        let mut split = line.split("=");
        let start = split.nth(0).unwrap().trim().to_string();
        let end = split.last().unwrap().to_string();
        let end = end.replace("(", "").replace(")", "");
        let res = end.split(",").map(|s| s.trim().to_string()).collect::<Vec<String>>();
        map.insert(start, (res[0].clone(), res[1].clone()));
    }

    let mut currents:Vec<String> = Vec::new();

    for k in map.keys(){
        if k.ends_with("A") {
           currents.push(k.clone());
        }
    }

    // let mut current = "AAA".to_string();
    let mut index = 0;
    let mut counter = 0;

    let mut res :Vec<usize>= Vec::new();
    for current in currents.iter(){
        let mut current = current.clone();
        while !current.ends_with("Z") {
            // dbg!(&current, instructions[index], &map.get(&current).unwrap());
            if instructions[index] == 'L' {
                current = map.get(&current).unwrap().0.clone();
            } else {
                current = map.get(&current).unwrap().1.clone();
            }
            index += 1;
            if index == instructions.len() {
                index = 0;
            }
            counter += 1;
        }
        res.push(counter);
        counter = 0;
        index = 0;
    }
    dbg!(&res);
    let mut iter = res.iter();
    let mut result = *iter.next().unwrap();
    // let mut result = 1; 
    for &num in iter {
        
        result = lcm(result, num);
        // dbg!(&num, &result);
    }
    
    dbg!(result);
    // // dbg!(&map);
    // while !currents.iter().all(|c| c.ends_with("Z")) {
    //     // dbg!(&current, instructions[index], &map.get(&current).unwrap());
    //     for c in currents.iter_mut() {
    //         if instructions[index] == 'L' {
    //             *c = map.get(c).unwrap().0.clone();
    //         } else {
    //             *c = map.get(c).unwrap().1.clone();
    //         }
    //     }
    //     index += 1;
    //     if index == instructions.len() {
    //         index = 0;
    //     }
    //     counter += 1;
    // }
    // dbg!(counter);
    Ok(())
}




















pub fn day8part1() -> input::Result<()> {
    let contents = input::load_day_file("day8.txt");
    let instructions = contents.lines().nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in contents.lines().skip(2) {
        let mut split = line.split("=");
        let start = split.nth(0).unwrap().trim().to_string();
        let end = split.last().unwrap().to_string();
        let end = end.replace("(", "").replace(")", "");
        let res = end.split(",").map(|s| s.trim().to_string()).collect::<Vec<String>>();
        map.insert(start, (res[0].clone(), res[1].clone()));
    }

    let mut current = "AAA".to_string();
    let mut index = 0;
    let mut counter = 0;
    // dbg!(&map);
    while current != "ZZZ"{
        // dbg!(&current, instructions[index], &map.get(&current).unwrap());
        if instructions[index] == 'L' {
            current = map.get(&current).unwrap().0.clone();
        } else {
            current = map.get(&current).unwrap().1.clone();
        }
        index += 1;
        if index == instructions.len() {
            index = 0;
        }
        counter += 1;
    }
    dbg!(counter);
    Ok(())
}
