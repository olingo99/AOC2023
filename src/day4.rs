use crate::input;
use std::collections::HashMap;

pub fn day4() -> input::Result<()> {
    let contents = input::load_day_file("day4.txt");
    let mut tot = 0;
    for line in contents.lines() {
        // println!("{}", line);
        let mut split: std::str::Split<'_, &str> = line.split(":").last().unwrap().split("|");
        let mut my_numbers: Vec<u32> = split.next().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let winning_numbres = split.next().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        //dbg!(&my_numbers);
        //dbg!(&winning_numbres);
        my_numbers.retain(|x| winning_numbres.contains(x));
        // //dbg!(&ok);
        //dbg!(&my_numbers);
        if my_numbers.len() == 0 {
            continue;
        }
        //dbg!(my_numbers.len());
        let count = 2_u32.pow((my_numbers.len() as u32)-1);
        //dbg!(&count);
        tot += count;
    }
    dbg!(tot);
    Ok(())
}
