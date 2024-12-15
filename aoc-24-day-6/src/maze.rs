use crate::guard::{Direction, Guard};
use ndarray::Array2;
use std::fmt::Display;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph, Widget};

#[derive(Debug, Default)]
pub enum MazeCell {
    #[default]
    Empty,
    Obstacle,
    Visited,
}

impl MazeCell {

    pub fn representation(&self) -> char {
        match self {
            MazeCell::Empty => { '.' }
            MazeCell::Obstacle => { '#' }
            MazeCell::Visited => { 'X' }
        }
    }
}


impl Widget for MazeCell {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let block = Block::bordered()
            .border_set(border::THICK);

        Paragraph::new(self.to_string())
            .centered()
            .block(block)
            .render(area, buf);
    }
}

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
                            guard = Some(g);
                        } else {
                            // This should be unreachable as there should only be one guard on a map
                            unreachable!()
                        }

                        maze_map[[row,col]] = MazeCell::Visited;
                },
                _ => { unreachable!(); }
            }
        };



        Self {
            guard: guard.unwrap(),
            map: maze_map,
        }

    }
}

impl Widget for Maze {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        todo!()
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