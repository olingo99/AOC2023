use crate::input;
use core::time;
use std::collections::HashMap;

pub fn day6() -> input::Result<()> {
    let contents = input::load_day_file("day6.txt");

    
    let line1 = contents.lines().nth(0).unwrap().split(":").last().unwrap().split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>();
    let line2 = contents.lines().nth(1).unwrap().split(":").last().unwrap().split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>();
    let time_distance = line1.iter().zip(line2.iter()).collect::<Vec<(&f32, &f32)>>();

    let tot = time_distance.iter().fold(1, |acc, (T, D)| acc * root(**T, **D));

    dbg!(tot);
    Ok(())
}

fn root(T: f32, D:f32)->u32{
    let delta = (T.powf(2.0)-4.0*D).sqrt();
    let mut x1 = (T + delta)/2.0;
    let mut x2 = (T - delta)/2.0;
    if x1.fract() != 0.0 { x1 = x1.floor(); } else { dbg!(x1);x1 -= 1.0; }
    if x2.fract() != 0.0 { x2 = x2.ceil(); } else { dbg!(x2);x2 += 1.0; }
    (x1-x2+1.0) as u32
}