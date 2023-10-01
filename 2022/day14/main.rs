#![feature(btree_cursors)]
use core::cmp::{max, min};
use std::{fs, println};
use std::collections::{BTreeMap, HashMap};
use peg;

use std::ops::Bound;

const FILE_PATH: &str = "day14/input.txt";
const TEST_FILE_PATH: &str = "day14/test_input.txt";

#[derive(Debug)]
#[derive(Clone)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Value
{
    Rock,
    Sand,
}

type Coord = i64;
type X = Coord;
type Y = Coord;
type Point = (X, Y);
type Line = Vec<Point>;
type Data = Vec<Line>;

type SparseMap = HashMap<X, BTreeMap<Y, Value>>;

peg::parser!{
    grammar line_parser() for str {
        rule coord() -> Coord
          = n:$(['0'..='9']+) { n.parse().unwrap() }
        rule point() -> Point
          = x:coord() "," y:coord() { (x,y) }
        pub rule line() -> Line
          = l:(point() ** " -> ") { l }
    }
}

fn read_input(contents: & String) -> Data
{
    contents.split("\n").filter(|line| !line.is_empty()).map(|line|
    {
        line_parser::line(line).unwrap()
    }).collect()
}

fn input_to_rock_map(data: &Data) -> SparseMap
{
    let mut rock_map = SparseMap::new();
    let mut fill_map = |x_start, x_end, y_start, y_end| {
        for x_i in x_start..=x_end
        {
            let y_values: BTreeMap<_, _> = (y_start..=y_end).into_iter().map(|elem| (elem, Value::Rock)).collect();
            match rock_map.contains_key(&x_i)
            {
                true => {rock_map.get_mut(&x_i).unwrap().extend(y_values);},
                false => {rock_map.insert(x_i, y_values);}
            };
        }
    };

    for line in data.into_iter()
    {
        for i in 0..line.len()-1
        {
            let (x1, y1) = line[i];
            let (x2, y2) = line[i + 1];
            let x_start = min(x1, x2);
            let x_end = max(x1, x2);
            let y_start = min(y1, y2);
            let y_end = max(y1, y2);
            if (x_start == x_end) || (y_start == y_end)
            {
                fill_map(x_start, x_end, y_start, y_end);
            }
        }
    }
    rock_map
}

fn display_field(map: &SparseMap) -> String {
    let bbox = map.iter().fold((i64::MAX, i64::MAX, i64::MIN, i64::MIN), |(left, top, right, bottom), (x, y_values)| {
        (min(left, *x), min(top, *y_values.first_key_value().unwrap().0), max(right, *x), max(bottom, *y_values.last_key_value().unwrap().0))
    });

    let width = bbox.2 - bbox.0 + 1;
    let height = bbox.3 - bbox.1 + 1;

    println!("size: (x: {:?}, y: {:?})", height, width);

    let mut mat = vec![vec!['.'; width.try_into().unwrap()]; height.try_into().unwrap()];
    for (x_pos, el) in map.iter() {
        let x: usize = (x_pos - bbox.0).try_into().unwrap();
        for (y_pos, value) in el.iter() {
            let y: usize = (y_pos - bbox.1).try_into().unwrap();
            mat[y][x] = match value {
                Value::Rock => '#',
                Value::Sand => 'o'
            };
        }
    }

    let res: String = mat.iter().map(|x| { x.iter().collect::<String>() + "\n"} ).collect();
    res
}

fn find_min_greater_y(map: &SparseMap, grain_position: &Point, floor_position: &Option<Y>) -> Option<Y>
{
    if map.contains_key(&grain_position.0)
    {
        return map[&grain_position.0].lower_bound(Bound::Excluded(&grain_position.1)).key().copied().or(floor_position.clone());
    }
    else
    {
        return floor_position.clone();
    }
}

fn has_position(map: & SparseMap, position: &Point, floor_position: &Option<Y>) -> bool
{
    if let Some(border) = floor_position
    {
        if border == &position.1
        {
            return true;
        }
    }
    map.get(&position.0).map_or(false, |elem| elem.contains_key(&position.1))
}

fn simulate_sand_fall(map: &mut SparseMap, floor_position: &Option<Y>) -> usize
{
    let mut grain_number: usize = 0;
    let start_sand_position = (500, 0);
    loop
    {
        let mut grain_position = start_sand_position.clone();
        if floor_position.is_some() && has_position(&map, &grain_position, &floor_position)
        {
            return grain_number
        }
        loop
        {
            let down_pos = (grain_position.0, grain_position.1 + 1);
            let has_down_move = !has_position(&map, &down_pos, &floor_position);
            if has_down_move
            {
                if let Some(y_surface) = find_min_greater_y(&map, &grain_position, &floor_position){
                    grain_position = (grain_position.0, y_surface - 1);
                }
                else
                {
                    return grain_number
                }
            }
            let left_diagonal_pos = (grain_position.0 - 1, grain_position.1 + 1);
            let right_diagonal_pos = (grain_position.0 + 1, grain_position.1 + 1);
            let has_left_move = !has_position(&map, &left_diagonal_pos, &floor_position);
            let has_right_move = !has_position(&map, &right_diagonal_pos, &floor_position);
            if has_left_move
            {
                grain_position = left_diagonal_pos;
            }
            else if has_right_move
            {
                grain_position = right_diagonal_pos;
            }
            else
            {
                let mut new_value = BTreeMap::<Y,Value>::new();
                new_value.insert(grain_position.1, Value::Sand);
                match map.contains_key(&grain_position.0)
                {
                    true => {map.get_mut(&grain_position.0).unwrap().extend(new_value);},
                    false => {map.insert(grain_position.0, new_value);}
                };
                break;
            }
        }
        grain_number += 1;
    }
}

fn part_1(contents: &String)
{
    let input = read_input(contents);
    let mut sparce_map = input_to_rock_map(&input);
    println!("Part 1");
    println!("Answer: {:?}", simulate_sand_fall(&mut sparce_map, &None));
    println!("{:}", display_field(&sparce_map));
}

fn calculate_floor_position(map: & SparseMap) -> Y
{
    map.iter().fold(i64::MIN, |bottom, (_x, y_values)| {
        max(bottom, *y_values.last_key_value().unwrap().0)
    }) + 2
}

fn part_2(contents: & String)
{
    let input = read_input(contents);
    let mut sparce_map = input_to_rock_map(&input);
    let floor_position = calculate_floor_position(& sparce_map);
    println!("Part 2");
    println!("Answer: {:?}", simulate_sand_fall(&mut sparce_map, &Some(floor_position)));
    println!("{:}", display_field(&sparce_map));
}

fn main()
{
    let test = true;
    let contents = if test {
                fs::read_to_string(TEST_FILE_PATH).expect("Should have been able to read the file")
            } else {
                fs::read_to_string(FILE_PATH).expect("Should have been able to read the file")
            };
    part_1(&contents);
    part_2(&contents);
}