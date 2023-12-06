use crate::input;
use std::{collections::HashMap, hash};


pub fn day5() -> input::Result<()> {
    let contents = input::load_day_file("day5.txt");
    let mut lines: Vec<&str> = contents.split("\r\n\r\n").collect();
    let seeds = lines.remove(0);
    let maps = lines;
    let seeds : Vec<u64> = seeds.split(":").last().unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    let seeds: Vec<(u64, u64)> = seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    dbg!(&seeds);
    let mut res: Vec<u64> = Vec::new();
    let maps_tuples: Vec<Vec<(u64, u64, u64)>> = maps.iter()
    .map(|map| {
        map.split("\r\n")
            .skip(1)
            .map(|line| {
                let nums: Vec<u64> = line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect();
                (nums[0], nums[1], nums[2])
            })
            .collect()
    })
    .collect();
    let mut tempsres: Vec<u64> = Vec::new();
    for seedrange in seeds{
        tempsres.clear();
        for mut seed in seedrange.0..=seedrange.1+seedrange.0{
            'maploop : for map in &maps_tuples{
                for (dst, src, n) in map{
                    if &seed >= src && &seed <= &(src + n){
                        seed = &seed-src+dst;
                        continue 'maploop;
                    }
                }
            }
            tempsres.push(seed);
            // dbg!(&tempsres);
        }
        res.push(*tempsres.iter().min().unwrap());
    }

    dbg!(res.iter().min());

    Ok(())
}



pub fn day5part1() -> input::Result<()> {
    let contents = input::load_day_file("day5.txt");
    let mut lines: Vec<&str> = contents.split("\r\n\r\n").collect();
    let seeds = lines.remove(0);
    let maps = lines;
    let seeds : Vec<u64> = seeds.split(":").last().unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut res: Vec<u64> = Vec::new();
    let maps_tuples: Vec<Vec<(u64, u64, u64)>> = maps.iter()
    .map(|map| {
        map.split("\r\n")
            .skip(1)
            .map(|line| {
                let nums: Vec<u64> = line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect();
                (nums[0], nums[1], nums[2])
            })
            .collect()
    })
    .collect();
    for mut seed in seeds{
        'maploop : for map in &maps_tuples{
            for (dst, src, n) in map{
                if &seed >= src && &seed <= &(src + n){
                    seed = &seed-src+dst;
                    continue 'maploop;
                }
            }
        }
        res.push(seed);
    }
    dbg!(res.iter().min());

    Ok(())
}
