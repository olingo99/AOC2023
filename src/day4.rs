use crate::input;
use std::collections::HashMap;



pub fn day4() -> input::Result<()> {
    let contents = input::load_day_file("day4.txt");
    let mut map: HashMap<u32, u32> = HashMap::new();
    let lines = contents.lines().collect::<Vec<&str>>();
    for line in lines.iter().rev() {
        let mut split: std::str::Split<'_, &str> = line.split(":").last().unwrap().split("|");
        let mut my_numbers: Vec<u32> = split.next().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let winning_numbres = split.next().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let cardId= line.split(":").next().unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();
        // dbg!(cardId);
        my_numbers.retain(|x| winning_numbres.contains(x));
        let res = my_numbers.len() as u32;

        let value = (cardId..cardId+res+1).map(|i| map.get(&i).unwrap_or(&0)).sum::<u32>();
        map.insert(cardId, value+1);
        // .get(&res).unwrap_or(&0);

        // count += 1;
        
    }
    dbg!(&map.values().sum::<u32>());
    // dbg!(tot);
    Ok(())
}




pub fn day4part1() -> input::Result<()> {
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
