use itertools::Itertools;

use crate::input;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::any::Any;
#[derive(Debug)]
struct Signal{
 dest : String,
 source : String,
 value : bool
}

trait Module: Any + Debug {
    fn process(&mut self, signal: Signal)->Option<Vec<Signal>>;
    fn get_dest(&self)->&Vec<String>;
    fn get_name(&self)->String;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn reset(&mut self);
}

// impl<T: Any> Module for T {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
#[derive(Debug)]
struct Flipflop{
    name: String,
    state: bool,
    dest: Vec<String>
}

impl Module for Flipflop {
    fn process(&mut self, signal:Signal)->Option<Vec<Signal>> {
        if !signal.value{
            self.state = !self.state;
            return Some(self.dest.iter().map(|dest| Signal{dest: dest.trim().to_string(), source: self.name.trim().to_string(), value: self.state}).collect::<Vec<Signal>>())
        }
        None
    }

    fn get_dest(&self)->&Vec<String> {
        return &self.dest
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_name(&self)->String {
        self.name.to_string()
    }

    fn reset(&mut self) {
        self.state = false;
    }
}
#[derive(Debug)]
struct Conj{
    name: String,
    state: bool,
    inputs: HashMap<String, bool>,
    dest:Vec<String>
}

impl Module for Conj{
    fn process(&mut self, signal: Signal)->Option<Vec<Signal>> {
        // dbg!(&self);
        // dbg!(&signal);
        if let Some(value) = self.inputs.get_mut(&signal.source) {
            *value = signal.value;
        }
        // dbg!(&self);

        if self.inputs.values().all(|a| *a) {
            return Some(self.dest.iter().map(|dest| Signal{dest: dest.trim().to_string(), source: self.name.trim().to_string(), value:false}).collect::<Vec<Signal>>())
        }
        return Some(self.dest.iter().map(|dest| Signal{dest: dest.trim().to_string(), source: self.name.trim().to_string(), value: true}).collect::<Vec<Signal>>())
    }
    fn get_dest(&self)->&Vec<String> {
        return &self.dest
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn get_name(&self)->String {
        self.name.to_string()
    }

    fn reset(&mut self) {
        self.state = false;
        for value in self.inputs.values_mut() {
            *value = false;
        }
    }
}
#[derive(Debug)]
struct Broadcast{
    name: String,
    dest : Vec<String>
}

impl Module for Broadcast {
    fn process(&mut self, signal: Signal)->Option<Vec<Signal>> {
        return Some(self.dest.iter().map(|dest| Signal{dest: dest.trim().to_string(), source: self.name.trim().to_string(), value: signal.value}).collect::<Vec<Signal>>())
    }
    fn get_dest(&self)->&Vec<String> {
        return &self.dest
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn get_name(&self)->String {
        self.name.to_string()
    }

    fn reset(&mut self) {
    }
}
#[derive(Debug)]
struct Output{
    name: String,
    dest : Vec<String>
}

impl Module for Output {
    fn process(&mut self, signal: Signal)->Option<Vec<Signal>> {
        return None
    }
    fn get_dest(&self)->&Vec<String> {
        return &self.dest
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn get_name(&self)->String {
        self.name.to_string()
    }

    fn reset(&mut self) {
    }
}

// #[derive(Debug)]
// struct Rx{
//     name: String,
//     dest : Vec<String>
// }

// impl Module for Rx {
//     fn process(&mut self, signal: Signal)->Option<Vec<Signal>> {
//         if !signal.value{
//             return Some(Vec::new());
//         }
//         return None
//     }
//     fn get_dest(&self)->&Vec<String> {
//         return &self.dest
//     }

//     fn as_any(&self) -> &dyn Any {
//         self
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }
//     fn get_name(&self)->String {
//         self.name.to_string()
//     }
// }

fn parse(content:String)->HashMap<String,Box<dyn Module>>{
    let mut conjvec: Vec<String> = Vec::new();
    let mut modules: HashMap<String,Box<dyn Module>> = HashMap::new();
    for line in content.lines(){

        let mut split = line.split("->");
        let mut module = split.next().unwrap();
        let dest: Vec<String> = split.next().unwrap().split(',').map(|a| a.trim().to_string()).collect();

        if module.contains("broadcaster"){
            modules.insert(module.trim().to_string(), Box::new(Broadcast{
                name: module.trim().to_string(),
                dest: dest
            }));
        }

        // else if module.contains("rx"){
        //     modules.insert(module.trim().to_string(), Box::new(Rx{
        //         name: module.trim().to_string(),
        //         dest: dest
        //     }));
        // }

        else if module.contains('%'){
            module = module.strip_prefix('%').unwrap();
            modules.insert(module.trim().to_string(), Box::new(Flipflop{
                name: module.trim().to_string(),
                state:false,
                dest: dest
            }));
        }

        else if module.contains('&'){
            module = module.strip_prefix('&').unwrap();
            modules.insert(module.trim().to_string(), Box::new(Conj{
                name: module.trim().to_string(),
                state:false,
                dest: dest,
                inputs: HashMap::new()
            }));
            conjvec.push(module.trim().to_string());

        }

    }

    dbg!(&conjvec);

    let mut updates = Vec::new();
    let mut outputs = Vec::new();


    for (name, module_box) in &modules {
        let module = module_box.as_ref();
        for dest in module.get_dest() {
            // dbg!(&dest);
            if conjvec.contains(dest) {
                updates.push((dest.clone(), module.get_name()));
            }
            if !modules.keys().contains(dest){
                outputs.push(dest.to_string());
            }
        }
    }

    dbg!(&outputs);
    // dbg!(&updates);

    for (dest, name) in updates {
        if let Some(conj) = modules.get_mut(&dest).and_then(|m| m.as_any_mut().downcast_mut::<Conj>()) {
            conj.inputs.insert(name, false);
        }
    }

    for m in outputs{
        modules.insert(m.clone(), Box::new(Output{
            name: m.clone(),
            dest: Vec::new()
        }));
    }

    modules
}

fn solve_part1(mut modules: HashMap<String,Box<dyn Module>>){

    let mut high = 0;
    let mut low = 0;

    dbg!(&modules);
    let mut queue: VecDeque<Signal> = VecDeque::new();
    for i in 0..1000{
        let mut start = Signal{
            dest:"broadcaster".to_string(),
            source:"button".to_string(),
            value:false
        };
        low+=1;

        queue.push_back(start);

        while let Some(signal) = queue.pop_front(){
            // dbg!(&signal);
            let mut module = modules.get_mut(&signal.dest).unwrap();
            if let Some(new_signal) = module.process(signal){
                for sig in new_signal{
                    if sig.value{
                        high +=1;
                    }
                    else{
                        low +=1;
                    }
                    queue.push_back(sig);
                }
            }
        }
    // dbg!(high, low);

    }

    dbg!(high, low);
    dbg!(high*low);

}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn lcm_of_vec(numbers: Vec<u64>) -> u64 {
    numbers.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
}

fn reset_state(modules: &mut HashMap<String,Box<dyn Module>>){
    for (name, mut module) in modules{
        module.reset();
    }
}

fn solve_part2(mut modules: HashMap<String,Box<dyn Module>>){



    let mut conjsig_trigger = Vec::new();
    for target in ["bh", "sh", "jf", "mz"]{         // the 4 conj modules taht are setup as inversors before the conj taht is only connected to rx
        let mut i=0;
        let mut running = true;
        let mut queue: VecDeque<Signal> = VecDeque::new();
        reset_state(&mut modules);

        while running{
            let mut start = Signal{
                dest:"broadcaster".to_string(),
                source:"button".to_string(),
                value:false
            };
            // low+=1;

            queue.push_back(start);
            // dbg!("main");

            while let Some(signal) = queue.pop_front(){
                // dbg!(&signal);
                let mut module = modules.get_mut(&signal.dest).unwrap();
                if let Some(new_signal) = module.process(signal){
                    for sig in new_signal{
                        // dbg!(&sig.dest);
                        if sig.dest == target.to_string() && !sig.value{
                            running = false;
                        }
                        queue.push_back(sig);
                    }
                }
            }
            i+=1;
        }
        conjsig_trigger.push(i);
    }

    dbg!(&conjsig_trigger);

    dbg!(lcm_of_vec(conjsig_trigger));

}

pub fn day20() -> input::Result<()> {
    let contents = input::load_day_file("day20.txt");
    let a = parse(contents);
    solve_part2(a);

    Ok(())
}
