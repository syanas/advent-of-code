use std::fs;
use itertools::Itertools;

const FILE_PATH: &str = "day02/input.txt";

fn cast_options(choice: &str)-> i32 {
    return match choice{
            "X" => 0,
            "Y" => 1,
            "Z" => 2,
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => 0
        };
}

fn cast_outcome(choice: &str)-> i32 {
    return match choice{
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0
        };
}

fn part_1(contents: & String){
    let mut sum: i32 = 0;
    for l in contents.split("\n").filter(|x| !x.is_empty()){
        let (elf_choice_input, my_choice_input) = l.split(" ").next_tuple().unwrap();
        let elf_choice = cast_options(elf_choice_input);
        let my_choice = cast_options(my_choice_input);
        let s1 = my_choice +1;
        let s2 = match (my_choice - elf_choice + 3) % 3
        {
            1 => 6,
            0 => 3,
            2 => 0,
            _ => 0
        };
        let round_score = s1 + s2;
        sum += round_score;
    }
    println!("{}", sum);
}

fn part_2(contents: & String) {
    let mut sum: i32 = 0;
    for l in contents.split("\n").filter(|x| !x.is_empty()){
        let (elf_choice_input, outcome_input) = l.split(" ").next_tuple().unwrap();
        let elf_choice = cast_options(elf_choice_input);
        let outcome = cast_outcome(outcome_input);
        let shift = match outcome
        {
            6 => 1,
            3 => 0,
            0 => 2,
            _ => 0
        };
        let my_choice = (elf_choice + shift) % 3;
        let round_score = outcome + my_choice + 1;
        sum += round_score;
    }
    println!("{}", sum);
}


fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    part_1(&contents);
    part_2(&contents);
}