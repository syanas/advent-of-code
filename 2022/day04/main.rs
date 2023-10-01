use std::fs;
use itertools::Itertools;
use tuple_map::*;

const FILE_PATH: &str = "day04/input.txt";

fn is_inside(elem: u32, range: (u32, u32)) -> bool
{
    return elem >= range.0 && elem <= range.1;
}

fn part_1(contents: & String){
    let mut sum: u32 = 0;
    for line in contents.split("\n").filter(|x| !x.is_empty()){
        let pair_of_elves = line.split(",").next_tuple();
        if let Some((first_range, second_range)) = pair_of_elves
        {
            let first_borders_opt: Option<(&str, &str)> = first_range.split("-").next_tuple();
            let second_borders_opt: Option<(&str, &str)> = second_range.split("-").next_tuple();
            if let (Some(first_borders), Some(second_borders)) = (first_borders_opt, second_borders_opt)
            {
                let first_borders = first_borders.map(|x| x.parse::<u32>().unwrap());
                let second_borders = second_borders.map(|x| x.parse::<u32>().unwrap());
                
                if first_borders.all(|x| is_inside(x, second_borders)) || second_borders.all(|x| is_inside(x, first_borders))
                {
                    sum += 1;
                }
            }
        }       
        
    }
    println!("{}", sum);
}

fn part_2(contents: & String) {
    let mut sum: u32 = 0;
    for line in contents.split("\n").filter(|x| !x.is_empty()){
        let pair_of_elves = line.split(",").next_tuple();
        if let Some((first_range, second_range)) = pair_of_elves
        {
            let first_borders_opt: Option<(&str, &str)> = first_range.split("-").next_tuple();
            let second_borders_opt: Option<(&str, &str)> = second_range.split("-").next_tuple();
            if let (Some(first_borders), Some(second_borders)) = (first_borders_opt, second_borders_opt)
            {
                let first_borders = first_borders.map(|x| x.parse::<u32>().unwrap());
                let second_borders = second_borders.map(|x| x.parse::<u32>().unwrap());
                
                if first_borders.any(|x| is_inside(x, second_borders)) || second_borders.any(|x| is_inside(x, first_borders))
                {
                    sum += 1;
                }
            }
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