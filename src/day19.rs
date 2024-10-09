use crate::input;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::max;
use std::cmp::min;



pub fn day19() -> input::Result<()> {
    let contents = input::load_day_file("day19.txt");

    let parsed_content = parse2(contents);

    solve_part2(parsed_content);
    Ok(())
}

fn parse(content: String)->(HashMap<String, Vec<Box<dyn Fn(HashMap<char, isize>)-> Option<String>>>>, Vec<HashMap<char, isize>>){
        
        let mut sections = content.split("\r\n\r\n");
        let rules = sections.next().unwrap_or("");
        let items = sections.next().unwrap_or("");
        
        let mut vec_of_hashmaps = Vec::new();

        for item in items.lines() {
            let item = item.trim_matches(|c| c == '{' || c == '}');
            let mut hashmap: HashMap<char, isize> = HashMap::new();
    
            for pair in item.split(',') {
                let mut kv = pair.split('=');
                let key = kv.next().unwrap().trim().to_string();
                let value = kv.next().unwrap().trim().parse::<isize>().unwrap();
                hashmap.insert(key.chars().next().unwrap(), value);
            }

            vec_of_hashmaps.push(hashmap);
        }

        let mut rules_map: HashMap<String, Vec<Box<dyn Fn(HashMap<char, isize>)-> Option<String>>>> = HashMap::new();

        for rule in rules.lines() {
            if let Some(pos) = rule.find('{') {
                let key = rule[..pos].to_string();
                let rulelist = rule[pos..].trim_matches(|c| c == '{' || c == '}').to_string();

                let mut lambda_vec:Vec<Box<dyn Fn(HashMap<char, isize>)-> Option<String>>> = Vec::new();

                for elem in rulelist.split(','){
                    
                    let result: Vec<&str> = elem.split(':').collect();
                    let count = result.len();


                    if count == 1 {
                        let r: String = result.last().unwrap().to_string();

                        lambda_vec.push(Box::new(
                            move |_:HashMap<char, isize>| {
                                Some(r.clone())
                            }
                        ))
                    }

                    else{
                        let elem = result[0].to_string();
                        let r = result[1].to_string();
                        if elem.contains('>'){

                            let mut split = elem.split('>');
                            let target = split.next().unwrap().to_string();
                            let check =split.next().unwrap().to_string();

                            lambda_vec.push(Box::new(
                                move |c:HashMap<char, isize>| {
                                    if c.get(&target.chars().next().unwrap()) > Some(&check.parse::<isize>().unwrap()) {
                                        Some(r.clone())
                                    }
                                    else{
                                        None
                                    }
                                }
                            ))

                        }

                        else if elem.contains('<'){

                            let mut split = elem.split('<');
                            let target = split.next().unwrap().to_string();
                            let check =split.next().unwrap().to_string();

                            // dbg!(target, check, r);

                            lambda_vec.push(Box::new(
                                move |c:HashMap<char, isize>| {
                                    if c.get(&target.chars().next().unwrap()) < Some(&check.parse::<isize>().unwrap()) {
                                        Some(r.clone())
                                    }
                                    else{
                                        None
                                    }
                                }
                            ))

                        }
                    }
                }

                rules_map.insert(key, lambda_vec);
            }
        }

        (rules_map, vec_of_hashmaps)

}

fn solve_part1(
    rules: HashMap<String, Vec<Box<dyn Fn(HashMap<char, isize>) -> Option<String>>>>,
    items: Vec<HashMap<char, isize>>,
) {
    let mut total: isize = 0;
    for item in items {
        let mut rl = rules.get("in").unwrap().iter();
        // dbg!(&item);

        let mut result_str: String = "P".to_string(); // Use String instead of &str
        while result_str != "A" && result_str != "R" {
            let rule = rl.next().unwrap();

            let result = rule(item.clone());

            if let Some(r) = result {
                result_str = r.clone(); // Now result_str is a String
                match result_str.as_str() {
                    "A" => {
                        total += item.clone().into_values().sum::<isize>();
                    }
                    "R" => {}
                    _ => {
                        rl = rules.get(&result_str).unwrap().iter(); // Use &result_str to look up the rules
                    }
                }
            }
        }
    }

    dbg!(total);
}
#[derive(Debug, PartialEq, Eq)]
enum Cond{
    bigger,
    smaller,
    None
}
#[derive(Debug)]
struct Rule{
    target: char,
    cond: Cond,
    lim: isize,
    res: String,
}


#[derive(Debug, Clone)]
struct Range{
    x:(isize,isize),
    m:(isize,isize),
    a:(isize,isize),
    s:(isize,isize),
    loc: String,
    loc_hist:Vec<String>
}

impl Range {
    // Method to get an immutable reference to the range
    fn get_range(&self, c: char) -> Option<&(isize, isize)> {
        match c {
            'x' => Some(&self.x),
            'm' => Some(&self.m),
            'a' => Some(&self.a),
            's' => Some(&self.s),
            _ => None,
        }
    }

    // Method to get a mutable reference to the range
    fn get_range_mut(&mut self, c: char) -> Option<&mut (isize, isize)> {
        match c {
            'x' => Some(&mut self.x),
            'm' => Some(&mut self.m),
            'a' => Some(&mut self.a),
            's' => Some(&mut self.s),
            _ => None,
        }
    }
}

fn parse2(content: String)->HashMap<String, Vec<Rule>>{
        
    let mut sections = content.split("\r\n\r\n");
    let rules = sections.next().unwrap_or("");

    let mut rules_map:HashMap<String, Vec<Rule>> = HashMap::new();


    for rule in rules.lines() {
        let mut rules_vec :Vec<Rule> = Vec::new();

        if let Some(pos) = rule.find('{') {
            let key = rule[..pos].to_string();
            let rulelist = rule[pos..].trim_matches(|c| c == '{' || c == '}').to_string();

            for elem in rulelist.split(','){
                
                let result: Vec<&str> = elem.split(':').collect();
                let count = result.len();


                if count == 1 {
                    let r: String = result.last().unwrap().to_string();

                    rules_vec.push(Rule { target: 'o', cond: Cond::None, lim: 0, res: r });
                }

                else{
                    let elem = result[0].to_string();
                    let r = result[1].to_string();
                    if elem.contains('>'){

                        let mut split = elem.split('>');
                        let target_str = split.next().unwrap().to_string();
                        let target = target_str.chars().next().unwrap();
                        let lim = split.next().unwrap().parse::<isize>().unwrap();

                        rules_vec.push(
                            Rule{
                                target: target,
                                cond: Cond::bigger,
                                lim,
                                res: r
                            }
                        );
                    }

                    else if elem.contains('<'){
                        let mut split = elem.split('<');
                        let target_str = split.next().unwrap().to_string();
                        let target = target_str.chars().next().unwrap();
                        let lim = split.next().unwrap().parse::<isize>().unwrap();

                        rules_vec.push(
                            Rule{
                                target: target,
                                cond: Cond::smaller,
                                lim,
                                res: r
                            }
                        );

                    }
                }
            }

            rules_map.insert(key, rules_vec);
        }
    }

    rules_map

}

fn solve_part2(rules: HashMap<String, Vec<Rule>>){
    let mut ranges: VecDeque<Range> = VecDeque::new();
    ranges.push_back(Range { x: (1,4000), m: (1,4000), a: (1,4000), s: (1,4000), loc: "in".to_string(), loc_hist:vec!["in".to_string()] });

    let mut done:Vec<Range> = Vec::new();

    while !ranges.is_empty(){
        // dbg!(&ranges);

        let mut range = ranges.pop_front().unwrap();

        if range.loc == "A"{
            done.push(range);
            continue;
        }
        if range.loc == "R"{
            continue;
        }


        for rule in rules.get(&range.loc).unwrap(){

            if rule.cond == Cond::bigger && range.get_range(rule.target).unwrap().1 >= rule.lim{
                let mut new_range = range.clone();
                // dbg!(&new_range);
                new_range.get_range_mut(rule.target).unwrap().0 = max(rule.lim , range.get_range(rule.target).unwrap().0)+1;
                range.get_range_mut(rule.target).unwrap().1 = min(rule.lim, range.get_range(rule.target).unwrap().1);
                
                // dbg!(&new_range);
                new_range.loc_hist.push(rule.res.clone());
                new_range.loc = rule.res.clone();
                ranges.push_back(new_range);
            }

            else if rule.cond == Cond::smaller && range.get_range(rule.target).unwrap().0 <= rule.lim {
                let mut new_range = range.clone();
            
                new_range.get_range_mut(rule.target).unwrap().1 = min(rule.lim, range.get_range(rule.target).unwrap().1)-1;
                range.get_range_mut(rule.target).unwrap().0 = max(rule.lim , range.get_range(rule.target).unwrap().0);

                new_range.loc = rule.res.clone();
                new_range.loc_hist.push(rule.res.clone());
                ranges.push_back(new_range);
            }

            else if rule.cond == Cond::None{
                let mut new_range = range.clone();
                new_range.loc = rule.res.clone();
                new_range.loc_hist.push(rule.res.clone());
                ranges.push_back(new_range);
            }

        }
    }

    // dbg!(&done.len());
    // dbg!(&done);


    let mut tot = 0;
    for range in &done{
        tot+= (range.x.1-range.x.0 +1) * (range.m.1-range.m.0 +1) * (range.a.1-range.a.0 +1) * (range.s.1-range.s.0+1) ;
    }

    // for range in &done{
    //     println!("{:?}{:?}{:?}{:?}", range.x, range.m, range.a, range.s)
    // }

    dbg!(tot);
}

