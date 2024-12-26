use std::fs::read_to_string;
use aoc_visualisation::grid::GridVisualiser;
use aoc_visualisation::grid::grid_utils::DisplayRowColumnNumber;
use aoc_visualisation::traits::ratatui::RatatuiStylised;
use clap::Parser;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use tracing::debug;
use aoc_24_day_6::maze::Maze;
use aoc_24_day_6::maze::maze_cell::MazeCell;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "false")]
    show_numbers: bool,

    #[clap(short, long, default_value = None)]
    max: Option<u8>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {


    let args = Args::parse();
    println!("{:?}", args);


    let display_row_column_numbers;
    if args.show_numbers {
        display_row_column_numbers = DisplayRowColumnNumber::Always;
    } else {
        display_row_column_numbers = DisplayRowColumnNumber::Never;
    }


    // let input = read_to_string("aoc-24-day-6/test_input/day6.txt").unwrap();
    let input = read_to_string("aoc-24-day-6/input/day6.txt").unwrap();
    let maze = &Maze::from_str(&input);

    debug!("Maze dimensions: {}x{}", maze.get_height(), maze.get_width());


    let mut terminal = ratatui::init();
    let area = terminal.get_frame().area();
    debug!("Area: {}", area);
    debug!("Widgth: {}, Height: {}", area.width, area.height);


    let (cell_content_width, cell_content_height) = MazeCell::get_cell_content_max_dimensions();
    debug!("Cell content width: {}, cell content height: {}", cell_content_width, cell_content_height);

    let mut visualiser;
    if let Some(max) = args.max {
        visualiser = GridVisualiser::new_with_limits(&mut terminal, display_row_column_numbers, max as usize, max as usize);
    } else {
        visualiser = GridVisualiser::new(&mut terminal, display_row_column_numbers);
    }

    visualiser.set_numbers_style(ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray));
    loop {


        if let Ok((no_rows, no_cols)) = visualiser.calculate_viewable_grid_size(cell_content_height, cell_content_width) {
            debug!("Viewable grid size: {} rows, {} cols", no_rows, no_cols);
            let (view, row_offset, col_offset) = maze.to_view_sized(no_rows, no_cols);
            visualiser.draw_ref(&view, row_offset, col_offset)?;
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

    let guard = maze.get_guard();

    println!("Guard position: {:?}", guard.get_position());
    println!("Guard direction: {:?}", guard.get_direction());
    println!("Guard history:   {:?}", guard.get_history());
    Ok(())
}