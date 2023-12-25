use crate::input;
use std::collections::HashMap;

pub fn day15() -> input::Result<()> {
    let contents = input::load_day_file("day15.txt");
    let tot: u64 = contents
    .split(',')
    .map(|line| {
        line.chars()
            .map(|char| (char as u8) as u64)
            .fold(0, |subtot, char_val| (char_val + subtot) * 17)
    })
    .fold(0, |tot, subtot| tot + subtot % 256);
    dbg!(tot);
    Ok(())
}







pub fn day15part1map() -> input::Result<()> {
    let contents = input::load_day_file("day15.txt");
    let tot: u64 = contents
    .split(',')
    .map(|line| {
        line.chars()
            .map(|char| (char as u8) as u64)
            .fold(0, |subtot, char_val| (char_val + subtot) * 17)
    })
    .fold(0, |tot, subtot| tot + subtot % 256);
    dbg!(tot);
    Ok(())
}

pub fn day15part1for() -> input::Result<()> {
    let contents = input::load_day_file("day15.txt");
    let mut tot : u64 = 0;
    for line in contents.split(','){
        let mut subtot:u64 = 0;
        for char in line.chars(){
            subtot = ((char as u8) as u64 + subtot)*17;
        }
        tot += subtot%256;
    }
    dbg!(tot);
    Ok(())
}
