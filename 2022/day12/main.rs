use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::Reverse;
use std::{fs, println};

const FILE_PATH: &str = "day12/input.txt";
const TEST_FILE_PATH: &str = "day12/test_input.txt";

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum Cell
{
    Start,
    End,
    FreeCell,
}

type Height = i64;
type HeightMap = Vec<Vec<(Cell, Height)>>;
type Position = (usize, usize);
type Distance = i64;

fn read_input(contents: & String) -> HeightMap
{
    contents.split("\n").filter(|map_row| !map_row.is_empty()).map(|map_row|
    {
        map_row.chars().map(|cell|
            {
                match cell
                {
                    'S' => (Cell::Start, 0),
                    'E' => (Cell::End, ('z' as i64) - ('a' as i64)),
                    _ => (Cell::FreeCell, (cell as i64) - ('a' as i64)),
                }
            }).collect()
    }).collect()
}

#[derive(Clone, Copy)]
enum NeighbourType
{
    DiffersByPositiveUnit,
    DiffersByNegativeUnit,
}

fn calculate_neighbours(height_map: & HeightMap, pos: & Position, neighbour_type: NeighbourType) -> Option<Vec<Position>>
{
    let (pos_x, pos_y) = pos;
    let pos_xi = *pos_x as isize;
    let pos_yi = *pos_y as isize;
    let len_x = height_map.len() as isize;
    let len_y = height_map.get(0)?.len() as isize;
    let direction = 
        match neighbour_type
        {
            NeighbourType::DiffersByPositiveUnit => 1,
            NeighbourType::DiffersByNegativeUnit => -1
        };

    Some([(-1, 0), (1, 0), (0, -1), (0, 1)].iter().map(|(dx, dy)| {(pos_xi + dx, pos_yi + dy)})
        .filter(|(new_x, new_y)| { new_x >= &0 && new_x < &len_x && new_y >= &0 && new_y < &len_y})
        .filter(|(x, y)| { direction * (height_map[*x as usize][*y as usize].1 - height_map[*pos_x][*pos_y].1) <= 1})
        .map(|(x, y)| {(x as usize, y as usize)})
        .collect()
    )
}

fn find_all(height_map: & HeightMap, value: (Cell, Height)) -> Vec<Position>
{
    height_map.iter().enumerate().map(|(index_row, row)|{ 
        row.iter().enumerate().filter_map(move |(index_column, (cell, height))| 
            {
                if *cell == value.0 && *height == value.1
                {
                    Some((index_row, index_column))
                }
                else {
                    None
                }
            })
    }).flatten().collect()
}

fn dijkstra(height_map: & HeightMap, start_vertice: &Position, neighbour_type: NeighbourType) -> Option<HashMap<Position, Distance>>
{
    let mut marked_set: HashSet<Position> = HashSet::new();
    let mut distance_map: HashMap<Position, Distance> = HashMap::new();
    let mut priority_queue: BinaryHeap<(Reverse<Distance>, Position)> = BinaryHeap::new();
    priority_queue.push((Reverse(0), *start_vertice));
    distance_map.insert(*start_vertice, 0);

    while let Some((Reverse(distance), current_position)) = priority_queue.pop()
    {
        if marked_set.contains(&current_position)
        {
            continue;
        }
        marked_set.insert(current_position.clone()); // TODO: удалить clone()
        for neighbour in calculate_neighbours(& height_map, & current_position, neighbour_type)?.iter()
        {
            if !distance_map.contains_key(neighbour)
            {
                distance_map.insert(*neighbour, distance + 1);
                priority_queue.push((Reverse(distance + 1), *neighbour));
            }
            else {
                if distance_map.get(neighbour)? > &(distance + 1)
                {
                    distance_map.insert(*neighbour, distance + 1);
                    priority_queue.push((Reverse(distance + 1), *neighbour));
                }
            }
        }
    }
    Some(distance_map)
}

fn part_1(contents: &String)
{
    let height_map = read_input(contents);
    let start_vertice = *find_all(&height_map, (Cell::Start, 0)).first().unwrap();
    let distance_map = dijkstra(& height_map, & start_vertice, NeighbourType::DiffersByPositiveUnit).unwrap();
    let end_vertice = *find_all(& height_map, (Cell::End, 25)).first().unwrap();

    println!("Part 1");
    println!("The fewest possible steps from S to E: {:?}", distance_map.get(& end_vertice));
}

fn find_closest_a_from_e_distance(height_map: & HeightMap, distance_map: &HashMap<Position, Distance>) -> Distance
{
    let start_vertice = *find_all(&height_map, (Cell::Start, 0)).first().unwrap();
    let mut a_vertices = find_all(&height_map, (Cell::FreeCell, 0));
    a_vertices.push(start_vertice);
    *a_vertices.iter().filter_map(|position|{Some(distance_map.get(position)?)}).min().unwrap()
}

fn part_2(contents: & String)
{
    let height_map = read_input(contents);
    let start_vertice = *find_all(& height_map, (Cell::End, 25)).first().unwrap();
    let distance_map = dijkstra(& height_map, & start_vertice, NeighbourType::DiffersByNegativeUnit).unwrap();

    println!("Part 2");
    println!("The fewest possible steps fron E to a: {:?}", find_closest_a_from_e_distance(& height_map, & distance_map));
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