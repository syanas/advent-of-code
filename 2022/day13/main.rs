use core::cmp::Ordering;
use std::{fs, println};
use itertools::Itertools;
use peg;

const FILE_PATH: &str = "day13/input.txt";
const TEST_FILE_PATH: &str = "day13/test_input.txt";

#[derive(Debug)]
#[derive(Eq)]
#[derive(Clone)]
pub enum Value
{
    List(Vec<Value>),
    Digit(i64),
}

type Packet = Value;
type PacketPair = Vec<Packet>;
type Data = Vec<PacketPair>;

peg::parser!{
    grammar list_parser() for str {
        rule number() -> Value
          = n:$(['0'..='9']+) {? n.parse().map_or(Err("i64"), |x| Ok(Value::Digit(x))) }

        pub rule list() -> Value
          = "[" l:((number()/list()) ** ",") "]" { Value::List(l) }
    }
}

fn read_input(contents: & String) -> Data
{
    contents.split("\n\n").filter(|line_pair| !line_pair.is_empty()).map(|line_pair|
    {
        line_pair.split("\n").filter(|line| !line.is_empty()).map(|line|
            {
                list_parser::list(line).unwrap()
            }).collect()
    }).collect()
}

impl PartialEq for Value
{
    fn eq(&self, other: &Self) -> bool
    {
        match self.cmp(other)
        {
            Ordering::Equal => true,
            _ => false
        }
    }
}

impl Ord for Value
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        match (self, other)
        {
            (Value::Digit(digit1), Value::Digit(digit2)) => digit1.cmp(&digit2),
            (Value::Digit(_), Value::List(_)) => {
                Value::List(vec![self.clone()]).cmp(other)
            },
            (Value::List(_), Value::Digit(_)) => {
                self.cmp(&Value::List(vec![other.clone()]))
            },
            (Value::List(list1), Value::List(list2)) => {
                list1.iter().zip(list2.iter()).fold(
                Ordering::Equal, |acc, (elem1, elem2)| acc.then_with(|| elem1.cmp(elem2))
                ).then(list1.len().cmp(&list2.len()))
            }
        }
    }
}

impl PartialOrd for Value
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

fn calculate_sum_ordered_lists_indices(input: &Data) -> usize
{
    input.iter().enumerate().fold(0, |acc, (index, pair)| 
        {
            acc + (index + 1) * match pair[0].cmp(&pair[1])
            {
                Ordering::Less => 1,
                _ => 0
            }
        }
    )
}

fn part_1(contents: &String)
{
    let input = read_input(contents);
    println!("Part 1");
    println!("Answer: {:?}", calculate_sum_ordered_lists_indices(&input));
}

fn find_decoder_key(input: &Data) -> usize
{
    let list_2 = Value::List(vec![Value::List(vec![Value::Digit(2)])]);
    let list_6 = Value::List(vec![Value::List(vec![Value::Digit(6)])]);
    input.iter().flatten().chain(vec![list_2.clone(), list_6.clone()].iter()).sorted().enumerate().fold(1, |acc, (index, list)| 
        {
            acc * if list == &list_2 || list == &list_6 {index + 1} else {1}

        }
    )
}

fn part_2(contents: & String)
{
    let input = read_input(contents);
    println!("Part 2");
    println!("Answer: {:?}", find_decoder_key(&input));
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