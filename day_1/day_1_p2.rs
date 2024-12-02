use std::fs;
use std::collections::HashMap;

fn main() {
    let data = fs::read_to_string("input.txt").expect("No open");

    let numbers: Vec<i32> = data.split_whitespace().map(|s| s.parse().expect("parse error")).collect();

    // println!("{}", numbers.len());

    let mut group1: HashMap<i32, i32> = HashMap::with_capacity(1000);
    let mut group2: HashMap<i32, i32> = HashMap::with_capacity(1000);

    let mut bool_val = true;

    for num in numbers {
        if bool_val {
            if group1.contains_key(&num) {
                group1.entry(num).and_modify(|val| *val += 1).or_insert(1);
            } else {
                group1.insert(num, 1);
            }
        } else {
            if group2.contains_key(&num) {
                group2.entry(num).and_modify(|val| *val += 1).or_insert(1);
            } else {
                group2.insert(num, 1);
            }
        }
        
        bool_val = !bool_val;
    }

    let mut acc = 0;

    for (_key, value) in group2.clone().into_iter() {
        acc += value;
    }

    println!("{}", acc);

    let mut acc = 0;

    for (_key, value) in group1.clone().into_iter() {
        acc += value;
    }

    println!("{}", acc);

    let mut acc = 0;

    for (key, value) in group2.clone().into_iter() {
        if group1.contains_key(&key) {
            acc += key * value;
            // println!("Key: {} Group1: {} Group2: {}", key, group1[&key], value);
        }
    }

    println!("{}", acc);
}
