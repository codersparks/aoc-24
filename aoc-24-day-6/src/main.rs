use std::fs::read_to_string;
use aoc_24_day_6::maze::Maze;

fn main() {
    let input = read_to_string("aoc-24-day-6/test_input/day6.txt").unwrap();
    let maze = &mut Maze::from_str(&input);

    println!("Maze: {:#?}", maze);
}