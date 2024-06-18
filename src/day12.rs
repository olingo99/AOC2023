use itertools::Itertools;

use crate::input;
use std::{collections::HashMap, mem, usize};

#[derive(Debug, Clone)]
struct Record {
    groups: Vec<usize>,
    springs: String,
}

pub fn day12() -> input::Result<()> {
    let contents = input::load_day_file("day12.txt");
    let records = parse_input_part_two(&contents);

    let total = solve_records_top_down(records, &mut HashMap::new());

    println!("Total number of ways to fill the springs: {}", total);

    Ok(())
}

fn solve_records_top_down(records: Vec<Record>, memo: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    let mut total = 0;
    for record in records {
        total += solve_record_top_down(&record.springs, &record.groups , memo);
    }
    total
}

fn solve_record_top_down(springs: &str, groups: &[usize], memo: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    let mut total = 0;

    if groups.is_empty() {
       if springs.contains("#")
       {
            return 0;
       }
       else 
       {
            return 1;
       }
    }
    if let Some(value) = memo.get(&(springs.to_string(), groups.to_vec())) {
        return *value;
    }

    let min_remaining_length = groups.iter().sum::<usize>() + groups.len() - 1;
    if springs.len() < min_remaining_length {
        return 0;
    }

    if &springs[0..1] == "."
    {
        return solve_record_top_down(&springs[1..], groups, memo)
    }

    let cur_group = groups[0];

    let all_springs_valid = springs[0..cur_group].chars().all(|c| c != '.');
    let last_char_valid = springs.len() == cur_group || springs[cur_group..cur_group+1].chars().all(|c| c != '#');

    if all_springs_valid && last_char_valid {
        let max_idx = springs.len().min(cur_group+1);
        total += solve_record_top_down(&springs[max_idx..], &groups[1..], memo)
    }

    if &springs[0..1] != "#" {
        total += solve_record_top_down(&springs[1..], groups, memo);
    }

    memo.insert((springs.to_string(), groups.to_vec()), total);
    return total;
}

fn parse_input(contents: &str) -> Vec<Record> {
    let mut records: Vec<Record> = Vec::new();
    for line in contents.lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = springs.to_string();
        let count:Vec<usize> = groups.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        records.push(Record { groups: count, springs });
    }
    records
}

fn parse_input_part_two(contents: &str) -> Vec<Record> {
    let mut records: Vec<Record> = Vec::new();
    for line in contents.lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = std::iter::repeat(springs.to_string())
            .take(5)
            .collect::<Vec<_>>()
            .join("?");
        let count:Vec<usize> = groups.split(',').map(|c| c.parse::<usize>().unwrap()).collect_vec().repeat(5);
        records.push(Record { groups: count, springs });
    }
    records
}