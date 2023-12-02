use crate::input;
use std::collections::HashMap;
use lazy_static::lazy_static;
lazy_static! {
    static ref MAP: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("red", 12);
        m.insert("green", 13);
        m.insert("blue", 14);
        m
    };
}

// pub fn day2() -> input::Result<()> {
//     let contents = input::load_day_file("day2.txt");
//     let mut tot:u32 = 0;
//     'outer :for line in contents.lines() {
//         println!("{}", line);
//         let mut split = line.split(":");
//         let id:u32 = split.next().unwrap().split_whitespace().last().unwrap().parse()?;
//         let game = split.next().unwrap();
//         dbg!(id);
//         dbg!(game);
//         for set in game.split(";"){
//             dbg!(set);
//             for pull in set.split(","){
//                 dbg!(pull);
//                 let mut pull = pull.split_whitespace();
//                 let nb:u32 = pull.next().unwrap().parse()?;
//                 let color = pull.next().unwrap();
//                 dbg!(nb);
//                 dbg!(color);
//                 if nb > *MAP.get(color).unwrap() {
//                     continue 'outer;
//                 }
//             }
//         }
//         tot += id;
//         dbg!(id);
//         dbg!(tot);
//     }
//     dbg!(tot);
//     Ok(())
// }



pub fn day2() -> input::Result<()> {
    let contents = input::load_day_file("day2.txt");
    let mut tot:u32 = 0;
    'outer :for line in contents.lines() {
        let mut min_cubes: HashMap<String, u32> = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);
        println!("{}", line);
        let mut split = line.split(":");
        let id:u32 = split.next().unwrap().split_whitespace().last().unwrap().parse()?;
        let game = split.next().unwrap();
        dbg!(id);
        dbg!(game);
        for set in game.split(";"){
            // dbg!(set);
            for pull in set.split(","){
                // dbg!(pull);
                let mut pull = pull.split_whitespace();
                let nb:u32 = pull.next().unwrap().parse()?;
                let color = pull.next().unwrap();
                if nb > *min_cubes.get(color).unwrap() {
                    min_cubes.insert(color.to_string(), nb);
                }
            }
        }
        dbg!(&min_cubes);
        let mult:u32 = min_cubes.values().product();
        tot +=mult;
    }
    dbg!(tot);
    Ok(())
}