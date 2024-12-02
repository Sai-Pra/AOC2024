use std::fs;

fn main () {
    let data = fs::read_to_string("/home/sai/Desktop/git-repos/Advent_of_code/day_1/input.txt").expect("No open");
    // println!("{}", data);

    let numbers: Vec<i32> = data.split_whitespace().map(|s| s.parse().expect("parse error")).collect();

    let mut group1: Vec<i32> = vec![];
    let mut group2: Vec<i32> = vec![];

    let mut bool_val = true;

    for num in numbers {
        if bool_val {
            group1.push(num);
            // bool_val = false;
        } else {
            group2.push(num);
            // bool_val = true;
        }

        bool_val = !bool_val;
    }

    group1.sort();
    group2.sort();

    let mut acc = 0;

    for i in 0..1000 {
        acc += (group1[i] - group2[i]).abs();
    }

    println!("{}",acc);
}