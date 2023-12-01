use lazy_static::lazy_static;

use crate::input;
use std::collections::HashMap;


lazy_static! {
    static ref DIC: HashMap<String, u32> = {
        let mut m = HashMap::new();
        m.insert("one".to_string(), 1);
        m.insert("two".to_string(), 2);
        m.insert("three".to_string(), 3);
        m.insert("four".to_string(), 4);
        m.insert("five".to_string(), 5);
        m.insert("six".to_string(), 6);
        m.insert("seven".to_string(), 7);
        m.insert("eight".to_string(), 8);
        m.insert("nine".to_string(), 9);
        m.insert("1".to_string(), 1);
        m.insert("2".to_string(), 2);
        m.insert("3".to_string(), 3);
        m.insert("4".to_string(), 4);
        m.insert("5".to_string(), 5);
        m.insert("6".to_string(), 6);
        m.insert("7".to_string(), 7);
        m.insert("8".to_string(), 8);
        m.insert("9".to_string(), 9);
        m
    };
}
pub fn day1() -> input::Result<()> {
    let contents = input::load_day_file("day1.txt");
    let mut score : u32 = 0;
    for line in contents.lines() {
        score += findRes(line, false)*10+findRes(line, true);
    }
    dbg!(score);
    Ok(())
}

fn findRes(inp: &str, reverse: bool) -> u32{
    //dbg!(inp);
    let mut p:u32 = 999;
    let mut res:u32 = 0;
    let mut lookupKey: String;
    let mut input = inp.to_string();
    if reverse { input = inp.chars().rev().collect();}
    for (key, value) in DIC.iter() {
        lookupKey = key.clone();
        if reverse {lookupKey = key.chars().rev().collect();}
        if let Some(index) = input.find(&lookupKey) {
            if (index as u32) < p {
                p = index as u32;
                res = *value;
            }
        }
    }
    return res;

}