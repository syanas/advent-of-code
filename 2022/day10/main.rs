use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::{fs, vec};

const FILE_PATH: &str = "day10/input.txt";
const TEST_FILE_PATH: &str = "day10/test_input.txt";

fn calculate_sum_of_signal_strengths(contents: & String, cycle_numbers: HashSet<i32>) -> i32
{
    let noop_pattern = Regex::new(r"noop").unwrap();
    let addx_pattern = Regex::new(r"addx\s+(?<number>-?\d+)").unwrap();
    let mut sum_of_signal_strength = 0;
    let mut cycle_number = 0;
    let mut x = 1;
    for line in contents.split("\n").filter(|x| !x.is_empty())
    {
        if let Some(_noop_instruction) = noop_pattern.captures(line)
        {
            cycle_number += 1;
            if let Some(target_number) = cycle_numbers.get(&cycle_number)
            {
                sum_of_signal_strength += target_number * x;
            }
        }
        if let Some(addx_instruction) = addx_pattern.captures(line)
        {
            if let Some(target_number) = cycle_numbers.get(&(cycle_number + 1))
            {
                sum_of_signal_strength += target_number * x;
            }
            if let Some(target_number) = cycle_numbers.get(&(cycle_number + 2))
            {
                sum_of_signal_strength += target_number * x;
            }
            if let Ok(number) = &addx_instruction["number"].parse::<i32>()
            {
               x += number;
            }
            cycle_number += 2;
        }
    }

    return sum_of_signal_strength;
}

fn determine_symbol(sprite_position: i32, cycle_number: i32, width: i32) -> char
{
    let pixel_position = cycle_number - 1;
    if sprite_position - 1 <= (pixel_position % width) && sprite_position + 1 >= (pixel_position % width)
    {
        return '#';
    }
    else {
        return '.';
    }
}

fn draw_picture(contents: & String, width: usize, height: usize)
{
    let noop_pattern = Regex::new(r"noop").unwrap();
    let addx_pattern = Regex::new(r"addx\s+(?<number>-?\d+)").unwrap();
    let mut picture: Vec<char> = vec![];
    let mut cycle_number = 0;
    let mut sprite_position: i32 = 1;
    for line in contents.split("\n").filter(|x| !x.is_empty())
    {
        if let Some(_noop_instruction) = noop_pattern.captures(line)
        {
            cycle_number += 1;
            picture.push(determine_symbol(sprite_position, cycle_number, width as i32));
        }
        if let Some(addx_instruction) = addx_pattern.captures(line)
        {
            picture.push(determine_symbol(sprite_position, cycle_number + 1, width as i32));
            picture.push(determine_symbol(sprite_position, cycle_number + 2, width as i32));
            if let Ok(number) = &addx_instruction["number"].parse::<i32>()
            {
               sprite_position += number;
            }
            cycle_number += 2;
        }
    }
    for i in 0..height
    {
        println!("{:?}", &picture[i*width..(i+1)*width]);
    }
}

fn part_1(contents: & String)
{
    let cycle_numbers: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    println!("Part 1");
    println!("Sum of signal strengths: {}", calculate_sum_of_signal_strengths(contents, HashSet::from_iter(cycle_numbers.iter().cloned())));
}

fn part_2(contents: & String)
{
    let width: usize = 40;
    let height: usize = 6;
    println!("Part 2");
    draw_picture(contents, width, height);
}

fn main()
{
    let test = false;
    let contents = if test {
                fs::read_to_string(TEST_FILE_PATH).expect("Should have been able to read the file")
            } else {
                fs::read_to_string(FILE_PATH).expect("Should have been able to read the file")
            };
    part_1(&contents);
    part_2(&contents);
}