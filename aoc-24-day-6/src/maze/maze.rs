use ndarray::Array2;
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};
use crate::guard::{Direction, Guard};
use crate::maze::maze_cell::MazeCell;

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
                        let d = g.get_direction().clone();
                        guard = Some(g);
                        maze_map[[row,col]] = MazeCell::Guard(d);
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

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &Maze {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = self.map.nrows();
        let cols = self.map.ncols();

        // Determine the dimensions for each cell block
        let cell_width = 3;  // Each cell block will have a width of 3
        let cell_height = 3; // Each cell block will have a height of 3

        // Figure out how many cells we can render within the allocated `area`
        let max_cols = usize::min(area.width as usize / cell_width, cols); // Max number of columns
        let max_rows = usize::min(area.height as usize / cell_height, rows); // Max number of rows

        // Iterate through each cell in the visible map range
        for row in 0..max_rows {
            for col in 0..max_cols {
                // Get the content of the MazeCell at [row, col]
                let cell = &self.map[[row, col]];
                let cell_symbol = match cell {
                    MazeCell::Obstacle => "#",    // Obstacle symbol
                    MazeCell::Empty => ".",       // Empty space symbol
                    MazeCell::Visited => "X",     // Visited symbol
                    MazeCell::Guard(_) => "@",    // Guard symbol
                };

                // Calculate the cell's drawing position
                let block_x = area.x + col as u16 * cell_width as u16;
                let block_y = area.y + row as u16 * cell_height as u16;

                // Define the block widget for the current cell
                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(
                        match cell {
                            MazeCell::Obstacle => Style::default().fg(Color::Red),
                            MazeCell::Empty => Style::default().fg(Color::White),
                            MazeCell::Visited => Style::default().bg(Color::Cyan),
                            MazeCell::Guard(_) => Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        }
                    );

                // Render this cell's block
                block.render(Rect::new(block_x, block_y, cell_width as u16, cell_height as u16), buf);

                // Calculate the center position inside the block
                let centered_x = block_x + (cell_width / 2) as u16;
                let centered_y = block_y + (cell_height / 2) as u16;

                // Write the cell value to the center position
                buf.set_string(centered_x, centered_y, cell_symbol, Style::default());
            }
        }
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