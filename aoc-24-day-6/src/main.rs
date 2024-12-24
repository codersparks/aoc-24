use std::fs::read_to_string;
use aoc_visualisation::grid::GridVisualiser;
use aoc_visualisation::grid::grid_utils::DisplayRowColumnNumber;
use aoc_visualisation::traits::ratatui::RatatuiStylised;
use ratatui::backend::TestBackend;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::Terminal;
use ratatui::widgets::Clear;
use tracing::debug;
use aoc_24_day_6::maze::Maze;
use aoc_24_day_6::maze::maze_cell::MazeCell;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt::init();
    // let input = read_to_string("aoc-24-day-6/test_input/day6.txt").unwrap();
    let input = read_to_string("aoc-24-day-6/input/day6.txt").unwrap();
    let maze = &Maze::from_str(&input);

    debug!("Maze dimensions: {}x{}", maze.get_height(), maze.get_width());


    let mut terminal = ratatui::init();
    let area = terminal.get_frame().area();
    debug!("Area: {}", area);
    debug!("Widgth: {}, Height: {}", area.width, area.height);
    // let mock_backend = TestBackend::new(20, 10); // Mock terminal with 20x10 size
    // let mut terminal = Terminal::new(mock_backend).unwrap();

    let (cell_content_width, cell_content_height) = MazeCell::get_cell_content_max_dimensions();
    debug!("Cell content width: {}, cell content height: {}", cell_content_width, cell_content_height);
    let mut visualiser = GridVisualiser::new_with_limits(&mut terminal, 15, 15);
    loop {


        if let Ok((no_rows, no_cols)) = visualiser.calculate_viewable_grid_size(cell_content_width, cell_content_height,DisplayRowColumnNumber::Never) {
            debug!("Viewable grid size: {} rows, {} cols", no_rows, no_cols);
            let view = maze.to_view_sized(no_rows, no_cols);
            visualiser.draw_ref(&view)?;
        } else {
            panic!("Could not calculate viewable grid size");
        }


        if let Event::Key(key_event) = event::read()? {
            if key_event.code == event::KeyCode::Char('q') {
                break;
            }
        }
    }
    ratatui::restore();
    Ok(())
}