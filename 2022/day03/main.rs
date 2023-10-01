use std::fs;
use itertools::Itertools;

const FILE_PATH: &str = "day03/input.txt";

fn cast_priority_to_int(letter: char) -> u32
{
    if letter.is_lowercase()
    {
        return u32::from(letter) - u32::from('a') + 1;
    }
    if letter.is_uppercase()
    {
        return u32::from(letter) - u32::from('A') + 27;
    }
    return 0;
}

fn part_1(contents: & String){
    let mut sum: u32 = 0;
    for l in contents.split("\n").filter(|x| !x.is_empty()){
        let rucksack_left = l.get(0..l.len()/2);
        let rucksack_right = l.get(l.len()/2 .. l.len());
        if let Some(rucksack_left_str) = rucksack_left {
            if let Some(rucksack_right_str) = rucksack_right {
                let mut priority_iter = rucksack_left_str.chars().filter(|letter| rucksack_right_str.contains(*letter));
                let priority_opt = priority_iter.next();
                if let Some(priority) = priority_opt {
                    sum += cast_priority_to_int(priority);
                }
            }
        }        
        
    }
    println!("{}", sum);
}

fn part_2(contents: & String) {
    let mut sum: u32 = 0;
    for (first_rucksack, second_rucksack, third_rucksack) in (contents.split("\n").into_iter()).tuples()
    {
        let mut priority_iter  = first_rucksack.chars().filter(|letter| second_rucksack.contains(*letter))
            .filter(|letter| third_rucksack.contains(*letter));
        let priority_opt = priority_iter.next();
        if let Some(priority) = priority_opt {
            sum += cast_priority_to_int(priority);
        }
    }
    println!("{}", sum);
}


fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    part_1(&contents);
    part_2(&contents);
}