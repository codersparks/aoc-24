use std::cmp::{max, min};
use ndarray::{s, Array2};
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, WidgetRef};
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

        // Center the visible region in the terminal area
        let grid_height = (row_end - row_start) * cell_size;
        let grid_width = (col_end - col_start) * cell_size;

        let y_offset = area.y + ((area.height as usize - grid_height) / 2) as u16;
        let x_offset = area.x + ((area.width as usize - grid_width) / 2) as u16;

        // Render the visible region of the maze
        for (row_idx, map_row) in self.map.slice(s![row_start..row_end, col_start..col_end]).outer_iter().enumerate() {
            for (col_idx, cell) in map_row.iter().enumerate() {
                let y = y_offset + (row_idx * cell_size) as u16;
                let x = x_offset + (col_idx * cell_size) as u16;

                // Use the `MazeCell::render` function to render each cell
                let cell_area = Rect {
                    x,
                    y,
                    width: cell_size as u16,
                    height: cell_size as u16,
                };
                cell.clone().render(cell_area, buf);
            }
        }
    }
}
impl Widget for &mut Maze {
    fn render(self, area: Rect, buf: &mut Buffer) {
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

        // Center the visible region in the terminal area
        let grid_height = (row_end - row_start) * cell_size;
        let grid_width = (col_end - col_start) * cell_size;

        let y_offset = area.y + ((area.height as usize - grid_height) / 2) as u16;
        let x_offset = area.x + ((area.width as usize - grid_width) / 2) as u16;

        // Render the visible region of the maze
        for (row_idx, map_row) in self.map.slice(s![row_start..row_end, col_start..col_end]).outer_iter().enumerate() {
            for (col_idx, cell) in map_row.iter().enumerate() {
                let y = y_offset + (row_idx * cell_size) as u16;
                let x = x_offset + (col_idx * cell_size) as u16;

                // Use the `MazeCell::render` function to render each cell
                let cell_area = Rect {
                    x,
                    y,
                    width: cell_size as u16,
                    height: cell_size as u16,
                };
                cell.clone().render(cell_area, buf);
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