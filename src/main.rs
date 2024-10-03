// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
mod day14;
// mod day15;
// mod day18;
mod day16;

mod input;
mod matrix;

use input::Day;
use matrix::Matrix;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let days = vec![
        // Day::new("Day1", day1::day1),
        // Day::new("Day2", day2::day2),
        // Day::new("Day3", day3::day3),
        // Day::new("Day4", day4::day4),
        // Day::new("Day5", day5::day5),
        // Day::new("Day6", day6::day6),
        // Day::new("Day7", day7::day7),
        // Day::new("Day8", day8::day8),
        // Day::new("Day9", day9::day9),
        // Day::new("Day10", day10::day10),
        // Day::new("Day11", day11::day11),
        // Day::new("Day12", day12::day12),
        // Day::new("Day13", day13::day13),

        // Day::new("Day14", day14::day14),
        Day::new("Day16", day16::day16),
        // Day::new("Day15", day15::day15),
        // Day::new("Day18", day18::day18),
        // other days...
    ];

    if args.len() > 1 {
        let day_arg = &args[1];
        for day in days.iter() {
            if day.name == *day_arg {
                day.run();
                break;
            }
        }
    } else {
        for day in days.iter() {
            day.run();
        }
    }
}
