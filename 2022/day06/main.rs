use std::fs;
use std::collections::{BTreeMap, VecDeque};

const FILE_PATH: &str = "day06/input.txt";

type Marker = BTreeMap<char, i32>;
type Window = VecDeque<char>;



fn add_symbol_to_state(window: &mut Window, marker: &mut Marker, symbol: char){
    window.push_back(symbol);
    if marker.contains_key(&symbol)
    {
        if let Some(symbol) = marker.get_mut(&symbol){
              *symbol += 1;
            }
    }
    else {
        marker.insert(symbol, 1);
    }
}

fn delete_symbol_from_state(window: &mut Window, marker: &mut Marker){
    let deleted_elem = window.pop_front();
    if let Some(elem) = deleted_elem
    {
        if marker.contains_key(&elem)
        {
            if let Some(elem) = marker.get_mut(&elem){
              *elem -= 1;
            }
            if marker[&elem] == 0 {
                marker.remove(&elem);
            }
        }
    }
}

fn algorithm(contents: & String, window_size: usize){
    let mut sliding_window = Window::new();
    let mut marker = Marker::new();
    let mut start_of_message: i32 = 0;
    let mut str_iter = contents.chars();
    for _i in 0..window_size {
        if let Some(letter) = str_iter.next(){
            add_symbol_to_state(& mut sliding_window, & mut marker, letter);   
        }
        start_of_message += 1;
    }
    
    while let Some(letter) = str_iter.next() {
        if marker.len() == window_size{
            break;
        }
        delete_symbol_from_state(& mut sliding_window, & mut marker);
        add_symbol_to_state(& mut sliding_window, & mut marker, letter);
        start_of_message += 1;
    }
    println!("{}", start_of_message);
}

fn part_1(contents: & String){
    algorithm(contents, 4);
}

fn part_2(contents: & String) {
    algorithm(contents, 14);
}

fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    part_1(&contents);
    part_2(&contents);
}