use crate::input;
use std::collections::HashMap;


// struct Box{
//     id: u16,
//     lenses: Vec<u16>,
// }


pub fn day15() -> input::Result<()> {
    let mut boxes: Vec<Vec<String>> = vec![Vec::new(); 256];
    let contents = input::load_day_file("day15.txt");
    'exte: for line in contents.split(','){
        let mut split = line.split(|c| c == '=' || c == '-');
        let label = split.next().unwrap();
        
        let lens_box:usize = get_box_id(label);
        // dbg!(lens_box);
        if line.contains('-'){
            boxes[lens_box].retain(|lens| lens.split_whitespace().next().unwrap() != label);
        }
        else{
            let label_with_focal = label.to_string() + " " + split.next().unwrap();
            for item in boxes[lens_box].iter_mut() {
                if item.split_whitespace().next().unwrap() == label {
                    *item = label_with_focal.to_string();
                    continue 'exte;
                }
            }
            boxes[lens_box].push(label_with_focal.to_string());
        }
    }
    // boxes.retain(|lens| lens.len() >= 1);
    // dbg!(boxes);
    let mut tot:u64 = 0;
    for (bo_index, bo) in boxes.iter().enumerate() {
        if bo.len() >= 1 {
            for (lens_index, lens) in bo.iter().enumerate() {
                let sub = (bo_index as u64+1)*(lens_index as u64+1)*lens.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap();
                // dbg!(sub);
                tot+= sub;
            }
        }
    }
    dbg!(tot);
    Ok(())
}


fn get_box_id(label: &str) -> usize{
    label.chars()
            .map(|char| (char as u8) as usize)
            .fold(0, |subtot, char_val| (char_val + subtot) * 17)%256
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
