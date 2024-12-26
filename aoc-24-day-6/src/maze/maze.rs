use crate::guard::{Direction, Guard};
use crate::maze::maze_cell::MazeCell;
use ndarray::{s, Array2, ArrayView2};
use std::cmp::{max, min};
use tracing::{debug, info};

#[derive(Debug)]
pub struct Maze {
    guard: Guard,
    map: Array2<MazeCell>
}

impl Maze {
    pub fn from_str(input: &str) -> Maze {
        // Trim in case it has spurious new line
        // To form the array2 we need to calculate the row and colum lengths
        let input = input.trim();
        let row_count = input.lines().count();
        let chars = input.lines().map(|l| l.chars()).flatten().collect::<Vec<_>>();
        let row_length = (chars.len()) / row_count;


        let mut maze_map = Array2::<MazeCell>::default((row_count, row_length));

        let mut guard : Option<Guard> = None;

        for i in 0..chars.len() {

            // we get the row number by integer division of
            let row = i / row_length;
            // we get the col number by modules of
            let col = i % row_length;

            let c = chars[i];
            // Now we generate the maze cells

            match c {
                '#' => maze_map[[row,col]] = MazeCell::Obstacle,
                '.' => { }, // we do nothing as default value is Empty
                '^' | 'v' | '<' |  '>' => {

                    if guard.is_none() {
                        let g = Guard::new(Direction::from_char(c), row, col);
                        maze_map[[row,col]] = MazeCell::Guard(*g.get_direction());
                        guard = Some(g);
                    } else {
                        // This should be unreachable as there should only be one guard on a map
                        unreachable!()
                    }


                },
                _ => { unreachable!(); }
            }
        };



        Self {
            guard: guard.unwrap(),
            map: maze_map,
        }

    }

    pub fn to_view_sized(&self, rows: usize, cols: usize) -> (ArrayView2<MazeCell>, usize, usize) {

        let map_height = self.map.shape()[0];
        let map_width = self.map.shape()[1];

        // If the max rows and columns are equal to the height and width we just return the entire view
        if rows >= map_height && cols >= map_width {
            return (self.map.view(), 0, 0);
        }

        // Guard's current position
        let guard_pos = self.guard.get_position();
        let guard_row = guard_pos.row;
        let guard_col = guard_pos.col;

        let row_start = max(0, guard_row as isize - (rows as isize / 2)) as usize;
        let col_start = max(0, guard_col as isize - (cols as isize / 2)) as usize;

        let row_end = min(map_height, row_start + rows);
        let col_end = min(map_width, col_start + cols);

        (self.map.slice(s![row_start..row_end, col_start..col_end]), row_start, col_start)
    }

    pub fn get_height(&self) -> usize {
        self.map.shape()[1]
    }

    pub fn get_width(&self) -> usize {
        self.map.shape()[0]
    }

    pub fn get_guard(&self) -> &Guard {
        &self.guard
    }

    pub fn move_guard(&mut self) -> bool {

        info!("Moving guard");

        let starting_guard_location = self.guard.get_position().clone();

        let starting_row = starting_guard_location.row;
        let starting_col = starting_guard_location.col;
        let starting_direction = starting_guard_location.direction;

        debug!("Starting row: {}, starting col: {}, starting direction: {:#?}", starting_row, starting_col, starting_direction);

        // Plane of travel is the row/col that the guard will be moving in e.g if facing upwards it will be a col
        // direction mod will be if the guard moves in a positive or negative direction in the row/col
        let (plane_of_travel, position, direction_mod) = match starting_direction {
            Direction::Up =>    { (self.map.column(starting_col), starting_row, -1)  }
            Direction::Down =>  { (self.map.column(starting_col), starting_row, 1) }
            Direction::Left =>  { (self.map.row(starting_row), starting_col, -1) }
            Direction::Right => { (self.map.row(starting_row), starting_col, 1) }
        };

        debug!("Plane of travel: {:#?}", plane_of_travel);

        let new_position = position as isize + direction_mod;
        debug!("Position: {}, new position: {}", position, new_position);

        if new_position < 0 || new_position >= plane_of_travel.len() as isize {
            self.guard.update_history_with_current_position();
            self.map[[starting_row, starting_col]] = MazeCell::Visited;
            info!("Guard has reached the end of the maze");
            info!("Visited Cell Count: {}", self.guard.get_unique_history_count());
            return true
        }

        match plane_of_travel[new_position as usize] {
            MazeCell::Obstacle => {
                info!("Guard has hit an obstacle, location {}", new_position);
                self.guard.change_direction();
                self.map[[starting_row, starting_col]] = MazeCell::Guard(*self.guard.get_direction());
                debug!("Visited Cell Count: {}", self.guard.get_unique_history_count());

            },
            MazeCell::Empty | MazeCell::Visited => {
                info!("Guard can move to empty cell, location {}", new_position);
                let (new_row, new_col) = match starting_direction {
                    Direction::Up => {(new_position as usize, starting_col)}
                    Direction::Down => {(new_position as usize, starting_col)}
                    Direction::Left => {(starting_row, new_position as usize)}
                    Direction::Right => {(starting_row, new_position as usize)}
                };

                debug!("New row: {}, new col: {}", new_row, new_col);

                self.map[[starting_row, starting_col]] = MazeCell::Visited;
                self.guard.update_history_with_current_position();
                self.guard.update_position(new_row, new_col);
                self.map[[new_row, new_col]] = MazeCell::Guard(*self.guard.get_direction());
                debug!("Visited Cell Count: {}", self.guard.get_unique_history_count());

            },
            _ => {unreachable!()}
        }

        false



    }


}

#[cfg(test)]
mod tests {
    use crate::maze::Maze;
    use std::fs;

    #[test]
    fn test_new_maze() {



        let input = fs::read_to_string("test_input/day6.txt").unwrap();

        println!("Input: {}", input);

        let maze = Maze::from_str(input.as_str());

        println!("Maze: {:#?}", maze);
    }
}
