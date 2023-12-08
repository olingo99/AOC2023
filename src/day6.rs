use crate::input;
use core::time;
use std::collections::HashMap;

pub fn day6() -> input::Result<()> {
    let contents = input::load_day_file("day6.txt");

    
    let line1 = contents.lines().nth(0).unwrap().split(":").last().unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let line2 = contents.lines().nth(1).unwrap().split(":").last().unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let time_distance = line1.iter().zip(line2.iter()).collect::<Vec<(&u64, &u64)>>();

    let tot = time_distance.iter().fold(1, |acc, (T, D)| acc * root(**T, **D));

    dbg!(tot);
    Ok(())
}

fn root(T: u64, D:u64)->u64{
    let delta = (T*T-4*D);
    dbg!(delta);
    //let delta = 9000000000_f64;
    let mut x1 = (T as f64 + (delta as f64).sqrt())/2.0_f64;
    let mut x2 = (T as f64 - (delta as f64).sqrt())/2.0_f64;
    // if x1 != (x1 as u64) as f64 { x1 = x1.floor(); } else { dbg!(x1.fract());x1 -= 1.0; }
    // if x2 != (x2 as u64) as f64 { x2 = x2.ceil(); } else { dbg!(x2.fract());x2 += 1.0; }

    x2 += 1_f64;
    dbg!(x1, x2, x2 as u64, x1 as u64+x2 as u64, (x1+x2) as u64);
    (x1-x2) as u64
}