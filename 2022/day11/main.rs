use regex::Regex;
use std::{fs, vec, println};

const FILE_PATH: &str = "day11/input.txt";
const TEST_FILE_PATH: &str = "day11/test_input.txt";
const MONKEY_PATTERN: &str = 
r"\s*?Monkey (?<number>\d+):
(\s)*?Starting items: (?<starting_items>[,\s\d]+)
(\s)*?Operation: new = old (?<operation>\+|\*) (?<second_operand>(old)|\d+)
(\s)*?Test: divisible by (?<denominator>\d+)
(\s)*?If true: throw to monkey (?<true_option>\d+)
(\s)*?If false: throw to monkey (?<false_option>\d+)";

type WorryLevel = i64;

fn apply_relief(x: WorryLevel) -> WorryLevel
{
    return x / 3;
}

fn apply_relief_by_chineese_theorem_(product: WorryLevel, x: WorryLevel) -> WorryLevel
{
    return x % product;
}

#[derive(Debug)]
struct Test
{
    number: i64,
    true_option: usize,
    false_option: usize,
}

impl Test
{
    fn test(&self, input: WorryLevel) -> usize {
        if input % self.number == 0
        {
            return self.true_option;
        }
        else
        {
            return self.false_option;
        }
    }
}

struct Monkey
{

    number: usize,
    items: Vec<WorryLevel>,
    operation: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    test: Test
}

impl Monkey
{
    fn inspect_items(&self) -> Vec<WorryLevel>
    {
        return self.items.iter().map(|x| (self.operation)(*x)).collect();
    }
}

impl std::fmt::Debug for Monkey
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.debug_struct("Monkey")
         .field("number", &self.number)
         .field("items", &self.items)
         .field("test", &self.test)
         .finish()
    }
}

#[derive(Debug)]
struct State
{
    monkeys: Vec<Monkey>,
    inspected_items_by_monkeys: Vec<usize>
}

fn read_input(contents: & String) -> Vec<Monkey>
{
    let monkey_pattern = Regex::new(MONKEY_PATTERN).unwrap();
    let mut data: Vec<Monkey> = Vec::new();
    for monkey_lines in contents.split("\n\n").filter(|monkey_lines| !monkey_lines.is_empty())
    {
        if let Some(monkey_input) = monkey_pattern.captures(monkey_lines)
        {
            if let (Ok(number), starting_items,
                    operation, second_operand,
                    Ok(denominator), Ok(true_option), Ok(false_option))
                = (&monkey_input["number"].parse::<usize>(), &monkey_input["starting_items"],
                    &monkey_input["operation"], &monkey_input["second_operand"],
                    &monkey_input["denominator"].parse::<WorryLevel>(), &monkey_input["true_option"].parse::<usize>(), &monkey_input["false_option"].parse::<usize>())
            {
                let parsed_function:  Box<dyn Fn(WorryLevel)->WorryLevel> = if second_operand == "old" 
                {
                    match operation
                    {
                       "+" => Box::new(|x| x + x),
                       "*" => Box::new(|x| x * x),
                       &_ => panic!()
                    }
                } 
                else
                {
                    if let Ok(operand) = second_operand.parse::<WorryLevel>()
                    {
                        match operation
                        {
                           "+" => Box::new(move |x| x + operand),
                           "*" => Box::new(move |x| x * operand),
                           &_ => panic!()
                        }
                    }
                    else 
                    {
                        panic!();
                    }
                };
                let monkey: Monkey = Monkey {
                    number: number.clone(),
                    items: starting_items.split(", ").into_iter().map(|x| x.parse::<WorryLevel>().unwrap()).collect(),
                    operation: parsed_function,
                    test: Test{number: denominator.clone(), true_option: true_option.clone(), false_option: false_option.clone()}
                };
                data.push(monkey);
            }
        }
    }
    return data;
}

fn simulate(state: &mut State, steps: usize, fun: Box<dyn Fn(WorryLevel) -> WorryLevel>)
{
    for _round in 0..steps
    {
        for i in 0..state.monkeys.len()
        {
            let monkey = &mut state.monkeys[i];
            let new_worry_levels: Vec<WorryLevel> = monkey.inspect_items().iter().map(|x| fun(*x)).collect();
            let destinations: Vec<usize> = new_worry_levels.iter().map(|x| monkey.test.test(*x)).collect();
            monkey.items.clear();
            state.inspected_items_by_monkeys[i] += new_worry_levels.len();
            for (worry_level, destination) in new_worry_levels.iter().zip(destinations.iter())
            {
                state.monkeys[*destination].items.push(*worry_level);
            }
        }
    }
}

fn calculate_monkey_business(state: &State) -> usize
{
    let mut inspected_items_sorted = state.inspected_items_by_monkeys.clone();
    inspected_items_sorted.sort();
    return inspected_items_sorted[inspected_items_sorted.len() - 1] * inspected_items_sorted[inspected_items_sorted.len() - 2];
}


fn part_1(contents: &String)
{
    let monkeys = read_input(contents);
    let monkeys_len = monkeys.len();
    let mut state: State = State{monkeys: monkeys, inspected_items_by_monkeys: vec![0; monkeys_len]};

    simulate(&mut state, 20,  Box::new(|x| apply_relief(x)));

    println!("Part 1");
    println!("The level of monkey business: {:?}", calculate_monkey_business(&state));
}

fn part_2(contents: & String)
{
    let monkeys = read_input(contents);
    let monkeys_len = monkeys.len();
    let mut state: State = State{monkeys: monkeys, inspected_items_by_monkeys: vec![0; monkeys_len]};

    let product: WorryLevel = state.monkeys.iter().fold(1, |product, x| product * x.test.number.clone());
    println!("{:?}", product);
    simulate(&mut state, 10000,  Box::new(move |x| apply_relief_by_chineese_theorem_(product, x)));

    println!("Part 2");
    println!("The level of monkey business: {:?}", calculate_monkey_business(&state));
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