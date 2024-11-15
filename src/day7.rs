use crate::input;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::fmt;

// #[derive(Debug,PartialEq, Eq)]
#[derive(PartialEq, Eq)]

struct hand {
    cards :Vec<u32>,
    bid: u32,
    order: u32,
}


lazy_static! {
    static ref MAP: HashMap<Vec<u32>, u32> = {
        let mut m = HashMap::new();
        m.insert(vec![1,1,1,1,1],0);
        m.insert(vec![1,1,1,2],1);
        m.insert(vec![1,2,2],2);
        m.insert(vec![1,1,3],3);
        m.insert(vec![2,3],4);
        m.insert(vec![1,4],5);
        m.insert(vec![5],6);
        m
    };
}

impl hand {
    fn new(data: &str) -> hand {

        let mut l = data.split_whitespace();
        let a  = l.next().unwrap().chars().map(|x| match x {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 0,
            'T' => 10,
            _ => {x.to_digit(10).unwrap()}
        }).collect::<Vec<u32>>();
        let mut counts = HashMap::new();
        let mut jokers = 0;
        for &c in a.iter() {
            if c == 0 {
                jokers += 1;
            }
            else{
                *counts.entry(c).or_insert(0) += 1;
            }
            // *counts.entry(c).or_insert(0) += 1;

        }
        // dbg!(&counts, jokers);
        let max_key = counts.iter().max_by_key(|(_, &v)| v).map(|(&k, _)| k);
        if let Some(key) = max_key {
            if let Some(count) = counts.get_mut(&key) {
                *count += jokers;
            }
        }
        if counts.is_empty() {
            counts.insert(0, jokers);
        }
        // dbg!(&counts);
        let mut v = counts.values().map(|&n| n).collect::<Vec<u32>>();
        v.sort();
        
        hand {
            cards: a,
            bid:  l.last().unwrap().parse::<u32>().unwrap(),
            order: MAP.get(&v).unwrap().clone(),
        }
    }
}

impl PartialOrd for hand{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.order == other.order {
            for (a,b) in self.cards.iter().zip(other.cards.iter()) {
                if a > b {
                    return Some(std::cmp::Ordering::Greater);
                } else if a < b {
                    return Some(std::cmp::Ordering::Less);
                }
            }
            Some(std::cmp::Ordering::Equal)
        } else {
            self.order.partial_cmp(&other.order)
        }
    }
}

impl Ord for hand{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.order == other.order {
            for (a,b) in self.cards.iter().zip(other.cards.iter()) {
                if a > b {
                    return std::cmp::Ordering::Greater;
                } else if a < b {
                    return std::cmp::Ordering::Less;
                }
            }
            std::cmp::Ordering::Equal
        } else {
            self.order.cmp(&other.order)
        }
    }
}

impl fmt::Debug for hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",self.bid)
    }
}

pub fn day7() -> input::Result<()> {
    let contents = input::load_day_file("day7.txt");
    let mut hands:Vec<hand> = contents.lines().map(|x| hand::new(x)).collect();
    hands.sort();
    // dbg!(&hands);
    let tot = hands.iter().enumerate().fold(0, |acc, (index, x)| {
        acc + x.bid*(index as u32 + 1)
    });
    dbg!(tot);
    Ok(())
}
