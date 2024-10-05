use crate::input;
use std::collections::HashMap;


pub fn day19() -> input::Result<()> {
    let contents = input::load_day_file("day19.txt");

    let parsed_content = parse(contents);

    solve_part1(parsed_content.0, parsed_content.1);
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

