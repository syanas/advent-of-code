use std::fs;

const FILE_PATH: &str = "day08/input.txt";

#[derive(Debug)]
struct Input
{
    tree_height_map: Vec<Vec<i32>>,
    columns: usize,
    rows: usize
}

#[derive(Clone, Copy)]
enum ProcessingType
{
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl Input
{
    fn read_input(contents: & String) -> Self
    {
        let mut data: Vec<Vec<i32>> = Vec::new();
            
        for line in contents.split("\n").filter(|line| !line.is_empty())
        {
            let mut row_vector: Vec<i32> = Vec::new();
            for ch in line.chars()
            {
                row_vector.push(ch.to_digit(10).unwrap() as i32);
            }
            data.push(row_vector);
        }
        Input
        {
            rows: data.len(),
            columns: if data.len() > 0 {data[0].len()} else {0},
            tree_height_map: data,
        }
    }

    fn get_columns(&self, processing_type: ProcessingType) -> usize {
        match processing_type 
        {
            ProcessingType::TopToBottom => self.columns,
            ProcessingType::RightToLeft => self.rows,
            ProcessingType::BottomToTop => self.columns,
            ProcessingType::LeftToRight => self.rows
        }
    }

    fn get_rows(&self, processing_type: ProcessingType) -> usize {
        match processing_type 
        {
            ProcessingType::TopToBottom => self.rows,
            ProcessingType::RightToLeft => self.columns,
            ProcessingType::BottomToTop => self.rows,
            ProcessingType::LeftToRight => self.columns
        }
    }

    fn get_idx(&self, i: usize, j: usize, processing_type: ProcessingType) -> [usize; 2]
    {
        match processing_type 
        {
            ProcessingType::TopToBottom => [i, j],
            ProcessingType::RightToLeft => [self.rows - j - 1, i],
            ProcessingType::BottomToTop => [self.rows - i - 1, self.columns - j - 1],
            ProcessingType::LeftToRight => [j, self.columns - i - 1]
        }
    }
}

fn is_visible_from_edge(elem: i32, max_from_edge_side: i32) -> bool
{
    return elem > max_from_edge_side;
}


fn process_visibility_by_rotation(input: &Input, result: &mut Vec<Vec<bool>>, processing_type: ProcessingType)
{
    let mut max_front: Vec<i32> = vec![-1; input.get_columns(processing_type)];
    for row in 0..input.get_rows(processing_type)
    {
        for column in 0..input.get_columns(processing_type)
        {
            let [real_row, real_column] = input.get_idx(row, column, processing_type);
            let map_item = input.tree_height_map[real_row][real_column];
            if  is_visible_from_edge(map_item, max_front[column]) {
                result[real_row][real_column] = true;
                max_front[column] = map_item;
            }
        }
    }
}

fn calculate_visibility(input: &Input) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; input.columns]; input.rows];
    process_visibility_by_rotation(input, &mut result, ProcessingType::TopToBottom);
    process_visibility_by_rotation(input, &mut result, ProcessingType::RightToLeft);
    process_visibility_by_rotation(input, &mut result, ProcessingType::BottomToTop);
    process_visibility_by_rotation(input, &mut result, ProcessingType::LeftToRight);

    result
}

fn part_1(contents: & String)
{
    let input = Input::read_input(contents);
    let visibility_map: Vec<Vec<bool>> = calculate_visibility(&input);
    let sum_of_visible: i32 = visibility_map.iter().map(|line| line.iter().map(|x| if *x {1} else {0}).sum::<i32>()).sum();

    println!("Part 1");
    println!("Sum of visible trees: {}", sum_of_visible);
}

#[derive(Clone, Copy)]
enum Direction
{
    ToTheLeft,
    ToTheRight,
    ToTheTop,
    ToTheBottom,
}

fn calculate_scentic_score(input: &Input, row: usize, column: usize, direction: Direction) -> i32 {
    let mut scentic_score: i32 = 0;
    let tree_height = input.tree_height_map[row][column];
    match direction 
        {
            Direction::ToTheLeft => 
            {
                for j in (0 .. column).rev()
                {
                    scentic_score += 1;
                    if tree_height <= input.tree_height_map[row][j]
                    {
                        break;
                    }
                }
            },
            Direction::ToTheRight => {
                for j in column + 1 .. input.columns
                {
                    scentic_score += 1;
                    if tree_height <= input.tree_height_map[row][j]
                    {
                        break;
                    }
                }
            },
            Direction::ToTheTop => {
                for i in (0 .. row).rev()
                {
                    scentic_score += 1;
                    if tree_height <= input.tree_height_map[i][column]
                    {
                        break;
                    }
                }
            },
            Direction::ToTheBottom => {
                for i in row + 1 .. input.rows
                {
                    scentic_score += 1;
                    if tree_height <= input.tree_height_map[i][column]
                    {
                       break; 
                    }
                }
            }
        }

    scentic_score
}

fn tree_is_on_the_edge(input: &Input, row: usize, column: usize) -> bool
{
    row == 0 || row == input.rows - 1 || column == 0 || column == input.columns - 1
}

fn part_2(contents: & String)
{
    let input = Input::read_input(contents);
    let mut max_scentic_score: i32 = 0;
    let mut row_max: usize = 0;
    let mut col_max: usize = 0;

    for row in 0..input.rows
    {
        for column in 0..input.columns
        {
            if !tree_is_on_the_edge(&input, row, column) {
                let current_scentic_score = [Direction::ToTheLeft, Direction::ToTheRight, Direction::ToTheTop, Direction::ToTheBottom].iter().fold(
                    1, |score, &direction| score * calculate_scentic_score(&input, row, column, direction));
                if max_scentic_score < current_scentic_score
                {
                    row_max = row;
                    col_max = column;
                    max_scentic_score = current_scentic_score;
                }
            }
        }
    }

    println!("Part 2");
    println!("Row: {}, Column: {}", row_max, col_max);
    println!("The highest scenic score: {}", max_scentic_score);
}

fn main()
{
    let test = false;
    let contents = if test {
            // "30373\n25512\n65332\n33549\n35390".into()
            "8894474474\n0438266351\n0786166682\n0622082266".into()
            } else {
                fs::read_to_string(FILE_PATH).expect("Should have been able to read the file")
            };

    part_1(&contents);
    part_2(&contents);
}