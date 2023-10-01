use std::fs;
use itertools::Itertools;
use regex::Regex;
use tuple_map::*;

const FILE_PATH: &str = "day05/input.txt";

type Containers = Vec<Vec<char>>;

fn initialize_scheme(scheme: & str) -> Containers
{
    let vec_size = scheme.split(' ').rev().filter(|x| !x.is_empty()).next();
    let mut result: Vec<Vec<char>> = Vec::new();
    if let Some(size) = vec_size
    {
        let size = size.parse::<usize>().unwrap();
        result.resize_with(size, Default::default);
        for line in scheme.split('\n').rev().skip(1)
        {
            let line_of_chars: Vec<_> = line.chars().collect();
            for i in 0..size
            {
                let position = i * 4 + 1;
                let container = line_of_chars[position];
                if container != ' '
                {
                    result[i].push(container);
                }
            }
        }
    }
    return result;
}

fn apply_action(container_positions: &mut Containers, amount: usize, from: usize, to: usize)
{
    for _i in 0..amount
    {
        if let Some(item) = container_positions[from].pop(){
            container_positions[to].push(item);   
        }
    }
}

fn apply_action_2(container_positions: &mut Containers, amount: usize, from: usize, to: usize)
{
    let first_elem = container_positions[from].len() - amount;
    let from_iter: Vec<_> = container_positions[from].drain(first_elem ..).collect();
    container_positions[to].extend(from_iter);
}

fn part_1(contents: & String){
    let mut result: String = Default::default();
    let split_input: Option<(_,_)> = contents.split("\n\n").next_tuple();
    if let Some((scheme, actions)) = split_input
    {
        let mut container_positions = initialize_scheme(scheme);
        for line in actions.split("\n").filter(|x| !x.is_empty())
        {
            let re = Regex::new(r".*move (?<number>[0-9]+) from (?<from>[0-9]+) to (?<to>[0-9]+).*").unwrap();
            if let Some(values) = re.captures(line)
            {
                if let (Ok(number), Ok(from), Ok(to)) = (&values["number"], &values["from"], &values["to"]).map(|x| x.parse::<usize>())
                {
                   apply_action(&mut container_positions,  number, from - 1, to - 1); 
                }
            }  
        }
        result = container_positions.iter().map(|x| if x.len() == 0 {' '} else { x[x.len()-1] }).join("");
    } 
    println!("{}", result);
}

fn part_2(contents: & String) {
    let mut result: String = Default::default();
    let split_input: Option<(_,_)> = contents.split("\n\n").next_tuple();
    if let Some((scheme, actions)) = split_input
    {
        let mut container_positions = initialize_scheme(scheme);
        for line in actions.split("\n").filter(|x| !x.is_empty())
        {
            let re = Regex::new(r".*move (?<number>[0-9]+) from (?<from>[0-9]+) to (?<to>[0-9]+).*").unwrap();
            if let Some(values) = re.captures(line)
            {
                if let (Ok(number), Ok(from), Ok(to)) = (&values["number"], &values["from"], &values["to"]).map(|x| x.parse::<usize>())
                {
                   apply_action_2(&mut container_positions,  number, from - 1, to - 1); 
                }
            }  
        }
        result = container_positions.iter().map(|x| if x.len() == 0 {' '} else { x[x.len()-1] }).join("");
    } 
    println!("{}", result);
}

fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    part_1(&contents);
    part_2(&contents);
}