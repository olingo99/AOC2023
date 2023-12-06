mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day14;
// mod day15;

mod input;

use input::Day;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let days = vec![
        Day::new("Day1", day1::day1),
        Day::new("Day2", day2::day2),
        Day::new("Day3", day3::day3),
        Day::new("Day4", day4::day4),
        Day::new("Day5", day5::day5),
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
