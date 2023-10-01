use std::collections::BTreeSet;
use core::cmp::max;
use std::{fs, println};
use peg;

const FILE_PATH: &str = "day15/input.txt";
const TEST_FILE_PATH: &str = "day15/test_input.txt";

type X = i64;
type Y = i64;
type Coord = i64;
type Point = (X, Y);
type Sensor = Point;
type Beacon = Point;
#[derive(Debug)]
pub struct ClosestPair
{
    sensor: Sensor,
    beacon: Beacon
}
type Data = Vec<ClosestPair>;


peg::parser!{
    grammar line_parser() for str {
        rule coord() -> Coord
          = n:$("-"?['0'..='9']+) { n.parse().unwrap() }
        rule point() -> Point
          = "x=" x:coord() ", y=" y:coord() { (x,y) }
        pub rule line() -> ClosestPair
          = "Sensor at " sensor:point()": closest beacon is at " beacon:point() { ClosestPair{sensor: sensor, beacon: beacon} }
    }
}

fn read_input(contents: & String) -> Data
{
    contents.split("\n").filter(|line| !line.is_empty()).map(|line|
    {
        line_parser::line(line).unwrap()
    }).collect()
}

fn calculate_manhattan_distance(pair: &ClosestPair)-> i64
{
    (pair.sensor.0 - pair.beacon.0).abs() + (pair.sensor.1 - pair.beacon.1).abs()
}

fn aggregate_ranges(sorted_ranges: &Vec<(Coord,Coord)>) -> Vec<(Coord,Coord)>
{
    let mut result: Vec<(Coord,Coord)> = Vec::new();
    result.push(sorted_ranges[0]);
    for i in 1..sorted_ranges.len()
    {
        let left = result.last().unwrap().clone();
        let right = sorted_ranges[i];
        if left.1 + 1 >= right.0
        {
            result.pop();
            result.push((left.0, max(left.1, right.1)));
        }
        else
        {
            result.push(right);
        }
    }
    result
}

fn calculate_ranges_coverage(ranges: &Vec<(Coord,Coord)>) -> i64
{
    let mut result = 0;
    for (left, right) in ranges
    {
        result += right - left + 1;
    }
    result
}

fn find_ranges_of_detection(data: &Data, row_index: i64) -> Vec<(Coord,Coord)>
{
    let mut covering_ranges: Vec<(Coord,Coord)> = vec![];
    for pair in data
    {
        let distance = calculate_manhattan_distance(pair);
        let difference_by_y = (row_index - pair.sensor.1).abs() - distance;
        if difference_by_y <= 0
        {
            covering_ranges.push((pair.sensor.0 - difference_by_y.abs(), pair.sensor.0 + difference_by_y.abs()));
        }
        if row_index == pair.beacon.1
        {
            covering_ranges.push((pair.beacon.0, pair.beacon.0));
        }
    }
    covering_ranges.sort_by_key(|(k,_v)| k.clone());
    covering_ranges
}

fn calculate_beacons_in_row(data: &Data, row_index: i64) -> i64
{
    let mut beacons_in_row: BTreeSet<Beacon> = BTreeSet::new();
    for pair in data
    {
        if row_index == pair.beacon.1
        {
            beacons_in_row.insert(pair.beacon);
        }
    }
    beacons_in_row.len() as i64
}

fn part_1(contents: &String, row_index: i64)
{
    let input = read_input(contents);
    let aggregated_ranges = aggregate_ranges(&find_ranges_of_detection(&input, row_index));
    let answer = calculate_ranges_coverage(&aggregated_ranges)- calculate_beacons_in_row(&input, row_index);

    println!("Part 1");
    println!("Answer: {:?}", answer);
}

fn part_2(contents: & String, box_max: i64)
{
    let input = read_input(contents);
    let mut beacon: Point = (0, 0);
    for i in 0..=box_max
    {
        let ranges = aggregate_ranges(&find_ranges_of_detection(&input, i));
        if ranges.len() > 1
        {
            let beacon_x = match ranges.len()
            {
                2 => {ranges[0].1 + 1},
                1 => {if ranges[0].0 != 0 { 0} else { box_max}},
                _ => panic!("{:?}", ranges.len()),
            };
            beacon = (beacon_x, i);
            break;
        }
    }
    println!("Part 2");
    println!("Answer: {:?}", 4000000 * beacon.0 + beacon.1);
}

fn main()
{
    let test = true;
    let contents = if test {
                fs::read_to_string(TEST_FILE_PATH).expect("Should have been able to read the file")
            } else {
                fs::read_to_string(FILE_PATH).expect("Should have been able to read the file")
            };
    part_1(&contents, if test {10} else { 2000000 });
    part_2(&contents, if test {20} else { 4000000 });
}