use std::fs;
use std::collections::BinaryHeap;

const FILE_PATH: &str = "day01/input.txt";

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn part_1(contents: & String){
    let mut max: i32 = 0;
    for l in contents.split("\n\n"){
        let sum = l.split("\n").filter(|x| !x.is_empty()).fold(0, |acc, x| acc + parse_input!(x, i32));
        if sum > max{
            max = sum;
        }  
    }
    println!("{}", max);
}

fn part_2(contents: & String) {
    let mut heap = BinaryHeap::new();
    for l in contents.split("\n\n"){
        let sum = l.split("\n").filter(|x| !x.is_empty()).fold(0, |acc, x| acc + parse_input!(x, i32));
        heap.push(sum);
    }
    assert!(heap.len() >= 3);
    println!("{}", heap.pop().unwrap_or(0) + heap.pop().unwrap_or(0) + heap.pop().unwrap_or(0));
}


fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    part_1(&contents);
    part_2(&contents);
}