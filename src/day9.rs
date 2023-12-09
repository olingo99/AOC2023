use crate::input;
use std::collections::HashMap;

pub fn day9() -> input::Result<()> {
    let contents = input::load_day_file("day9.txt");
    let histories = contents.lines()
    .map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>())
    .collect::<Vec<Vec<i32>>>();

    let mut tot = 0;
    for p_history in histories.iter() {
        let mut history = p_history.clone(); 
        history.reverse();
        let mut res: i32 = history.last().unwrap().clone();
        let mut next_line = history.clone();
        while next_line.iter().any(|x| x!= &0) {
            let clone = next_line.clone();
            next_line.clear();
            for window in clone.windows(2) {
                next_line.push(window[1] - window[0]);
            }
            res+= next_line.last().unwrap();
        }
        // dbg!(res);
        tot += res;
    }

    dbg!(tot);
    // dbg!(histories);
    Ok(())
}
