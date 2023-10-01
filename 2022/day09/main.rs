use itertools::Itertools;
use std::{fs, vec};
use std::collections::HashSet;

const FILE_PATH: &str = "day09/input.txt";

#[derive(Clone, Copy, Debug)]
enum Direction
{
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug)]
struct Instruction
{
    direction: Direction,
    number_of_steps: usize
}

#[derive(Debug)]
struct State
{
    rope_position: Vec<(i32, i32)>,
    positions_visited_by_tail: HashSet<(i32, i32)>
}

#[derive(Debug)]
struct InstructionProcessor<'a>
{
    state: &'a mut State
}

fn calculate_head_position(position: (i32, i32), direction: & Direction) -> (i32, i32)
    {
        match direction
        {
            Direction::Up => return (position.0, position.1 + 1),
            Direction::Right => return (position.0 + 1, position.1),
            Direction::Down => return (position.0, position.1 - 1),
            Direction::Left => return (position.0 - 1, position.1)
        };
    }

fn change_knot(previous_knot: &(i32, i32), knot: &(i32, i32)) -> Option<(i32, i32)>
{
    let x_diff = previous_knot.0 - knot.0;
    let y_diff = previous_knot.1 - knot.1;
    if x_diff.abs() < 2 && y_diff.abs() < 2
    {
        return None;
    }

    let mut new_position: (i32, i32) = knot.clone();
    let x_direction = if x_diff == x_diff.abs(){1} else {-1};
    let y_direction = if y_diff == y_diff.abs(){1} else {-1};
    if x_diff.abs() > 0
    {
        new_position.0 = knot.0 + x_direction;
    }
    if y_diff.abs() > 0
    {
        new_position.1 = knot.1 + y_direction;
    }
    return Some(new_position);
}

impl InstructionProcessor<'_>
{
    fn change_rope_positions(&mut self, direction: &Direction)
    {
        self.state.rope_position[0] = calculate_head_position(self.state.rope_position[0], &direction);

        let rope_size = self.state.rope_position.len();
        let mut tail_moved = false || rope_size == 1;
        for i in 1..rope_size
        {
            if let Some(new_position) = change_knot(&self.state.rope_position[i - 1], &self.state.rope_position[i])
            {
                self.state.rope_position[i] = new_position.clone();
                if i == rope_size - 1
                {
                    tail_moved = true;
                }
            }
            else {
                break;
            }
        }

        if tail_moved
        {
            self.state.positions_visited_by_tail.insert(self.state.rope_position[rope_size - 1]);
        }
    }

    fn apply_instruction(&mut self, instruction: &Instruction)
    {
        for _i in 0..instruction.number_of_steps
        {
            self.change_rope_positions(&instruction.direction);
        }
    }
}

fn process_input_direction(input: &str)-> Direction {
    return match input
    {
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        _ => panic!("{:?}", "Unexpected command")
    };
}

fn simulate(contents: & String, knots_number: usize) -> usize
{
    let mut state =
        State
        {
            rope_position: vec![(0, 0); knots_number],
            positions_visited_by_tail: HashSet::new()
        };
    state.positions_visited_by_tail.insert((0, 0));

    let mut instruction_processor =
        InstructionProcessor
        {
            state: &mut state
        };

    for l in contents.split("\n").filter(|x| !x.is_empty()){
        let (direction_input, number_of_steps_input) = l.split(" ").next_tuple().unwrap();
        let instruction =
            Instruction
            {
                direction: process_input_direction(direction_input),
                number_of_steps: number_of_steps_input.parse::<usize>().unwrap()
            };
        instruction_processor.apply_instruction(&instruction);
    }

    return instruction_processor.state.positions_visited_by_tail.len();
}

fn part_1(contents: & String)
{
    println!("Part 1");
    println!("Sum of positions visited by tail: {}", simulate(contents, 2));
}

fn part_2(contents: & String)
{
    println!("Part 2");
    println!("Sum of positions visited by tail: {}", simulate(contents, 10));
}

fn main()
{
    let test = false;
    let contents = if test {
                "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2".into()
            } else {
                fs::read_to_string(FILE_PATH).expect("Should have been able to read the file")
            };
    part_1(&contents);
    part_2(&contents);
}