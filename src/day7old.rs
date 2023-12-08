use crate::input;
use std::{collections::HashMap, vec};
use itertools::{Itertools, Combinations};
use regex::Regex;

pub fn day7() -> input::Result<()> {
    let contents = input::load_day_file("day7.txt");
    let input: HashMap<String, u32> = contents.lines().map(|x| {
        let mut split = x.split_whitespace();
        let key = split.next().unwrap().to_string();
        let value = split.last().unwrap().parse().unwrap();
        (key, value)
    }).collect();
    dbg!(&input);
    let map : HashMap<Vec<&u32>, u32> = HashMap::from([
        (vec![&1,&1,&1,&1,&1],0),
        (vec![&1,&1,&1,&2],1),
        (vec![&1,&2,&2],2),
        (vec![&1,&1,&3],3),
        (vec![&2,&3],4),
        (vec![&1,&4],5),
        (vec![&5],6),
    ]);

    let mut tot = 0;

    let mut counts = HashMap::new();
    for (combination,val) in input.iter(){
        counts.clear();
        for c in combination.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut v = counts.values().collect::<Vec<&u32>>();
        v.sort();
        // dbg!(&v);
        dbg!(map.get(&v).unwrap(), val, map.get(&v).unwrap() * val);
        tot += map.get(&v).unwrap() * val;
    }
    dbg!(tot);
    Ok(())
}



    // let mut all_combinations: Vec<String> = vec![
    //     "KT9A2".to_string(), 
    //     "J8T6A".to_string(), 
    //     "AAAAA".to_string(),
    //     "AA8AA".to_string(),
    //     "23332".to_string(),
    //     "TTT98".to_string(),
    //     "23432".to_string(),
    //     "A23A4".to_string(),
    //     "23456".to_string(),
    //     // Add more strings here...
    // ];