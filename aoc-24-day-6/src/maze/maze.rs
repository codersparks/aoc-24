use std::cmp::{max, min};
use ndarray::{s, Array2};
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::{Layout, Rect};
use ratatui::style::{Style, Color};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, WidgetRef};
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

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    fn create_constraints(n: usize, cell_size: usize) -> Vec<ratatui::layout::Constraint> {
        (0..n).map(|_| ratatui::layout::Constraint::Length(cell_size as u16)).collect()
    }
}

impl WidgetRef for Maze {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let map_height = self.map.shape()[0];
        let map_width = self.map.shape()[1];
        let cell_size = 3; // Each cell is 3x3

        // Maximum number of cells that can fit in the terminal area
        let max_visible_rows = (area.height as usize) / cell_size;
        let max_visible_cols = (area.width as usize) / cell_size;

        // Guard's current position
        let guard_pos = self.guard.get_position();
        let guard_row = guard_pos.row;
        let guard_col = guard_pos.col;

        // Calculate the visible region centered on the guard
        let visible_rows = min(max_visible_rows, map_height);
        let visible_cols = min(max_visible_cols, map_width);

        let row_start = max(0, guard_row as isize - (visible_rows as isize / 2)) as usize;
        let col_start = max(0, guard_col as isize - (visible_cols as isize / 2)) as usize;

        let row_end = min(map_height, row_start + visible_rows);
        let col_end = min(map_width, col_start + visible_cols);

        let row_constraints = Self::create_constraints(row_end - row_start, cell_size);


        let rows = Layout::default().direction(ratatui::layout::Direction::Vertical)
            .constraints(row_constraints).split(area);


        let visible_map = self.map.slice(s![row_start..row_end, col_start..col_end]);

        for (row_idx, grid_row) in rows.iter().enumerate() {
            let col_constraints = Self::create_constraints(col_end - col_start, cell_size);
            let cols = Layout::default().direction(ratatui::layout::Direction::Horizontal)
                .constraints(col_constraints).split(*grid_row);

            for (col_idx, cell) in cols.iter().enumerate() {

                let block = Block::default().borders(Borders::ALL);

                let style = match visible_map[[row_idx, col_idx]] {
                    MazeCell::Empty => { Style::default() }
                    MazeCell::Obstacle => { Style::default().fg(Color::Red)}
                    MazeCell::Guard(_) => { Style::default().bg(Color::White).fg(Color::Black)}
                    MazeCell::Visited => {Style::default().bg(Color::Blue)}
                };

                let representation = format!("{}", visible_map[[row_idx, col_idx]].representation());
                let paragraph = Paragraph::new(representation).style(style)
                    .block(block).centered();
                paragraph.render(*cell, buf);

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