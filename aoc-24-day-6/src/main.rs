use std::cmp::max;
use std::fs::{read_to_string, OpenOptions};
use std::path::PathBuf;
use aoc_visualisation::grid::GridVisualiser;
use aoc_visualisation::grid::grid_utils::DisplayRowColumnNumber;
use aoc_visualisation::traits::ratatui::RatatuiStylised;
use clap::Parser;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use tracing::{debug, info, Level};
use tracing_subscriber::{filter, Layer, Registry};
use tracing_subscriber::layer::SubscriberExt;
use aoc_24_day_6::guard::Direction;
use aoc_24_day_6::maze::Maze;
use aoc_24_day_6::maze::maze_cell::MazeCell;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "false")]
    show_numbers: bool,

    #[clap(short, long, default_value = "false")]
    no_visualisation: bool,

    #[clap(short, long, default_value = None)]
    input_file: Option<PathBuf>,

    #[clap(short, long, default_value = None)]
    log_file: Option<PathBuf>,

    #[clap(short, long, default_value = None)]
    max: Option<u8>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {


    let args = Args::parse();

    if let Some(log_path) = &args.log_file {
        let log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_path)?;

        let subscriber = Registry::default()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(log_file)
                    .with_ansi(false)
                    .with_filter(filter::LevelFilter::from_level(Level::DEBUG))
            );

        tracing::subscriber::set_global_default(subscriber).unwrap();
        info!("===================================== New Run ===========================");
    }

    let input_path = args.input_file.unwrap_or_else(|| PathBuf::from("aoc-24-day-6/input/day6.txt"));

    let input = read_to_string(input_path).unwrap();
    let maze = &mut Maze::from_str(&input);


    if args.no_visualisation {
        while maze.move_guard() != true {

        }

    } else {
        let display_row_column_numbers;
        if args.show_numbers {
            display_row_column_numbers = DisplayRowColumnNumber::Always;
        } else {
            display_row_column_numbers = DisplayRowColumnNumber::Never;
        }




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

                if key_event.code == event::KeyCode::Char('m') {
                    let left_board = maze.move_guard();
                    if left_board {
                        break;
                    }
                }
            }
        }
        ratatui::restore();

    }

    let guard = maze.get_guard();

    println!("####### Part 1 #######");
    println!("Guard position:       {:?}", guard.get_position());
    println!("Guard direction:      {:?}", guard.get_direction());
    println!("Visited Cell Count:   {:?}", guard.get_unique_history_count());


    println!("####### Part 2 #######");

    let history = guard.get_history();

    let change_in_directions = history.windows(2).filter(|w| {
        w[0].get_direction() != w[1].get_direction()
    }).map(|pair| &pair[1]).collect::<Vec<_>>();

    debug!("Change in directions: {:#?}", change_in_directions);

    // so we take a window of size 3 to see if it makes up the 3 sides of the rectangle, with the potential added obstacle making the 4th point
    let new_obstacles = change_in_directions.windows(3).filter(|w| {
        // If the turnst are not in sequance then we can ignore this window
        match w[0].get_direction() {
            Direction::Up => {
                if ! matches!(w[1].get_direction(), Direction::Right) && ! matches!(w[1].get_direction(), Direction::Down) { return false; }
            }
            Direction::Down => {
                if ! matches!(w[1].get_direction(), Direction::Left) && ! matches!(w[1].get_direction(), Direction::Up) { return false; }
            }
            Direction::Left => {
                if ! matches!(w[1].get_direction(), Direction::Up) && ! matches!(w[1].get_direction(), Direction::Right) { return false; }
            }
            Direction::Right => {
                if ! matches!(w[1].get_direction(), Direction::Down) && ! matches!(w[1].get_direction(), Direction::Left) { return false; }
            }
        }
        true
    }).map(|w| {
        // If we get here then we know that we have the three sides of a loop
        // We know the co-ord that has to change for the added obstacle as we know the direction from the start point
        // (the direction from w0 is tha same direction as w1 -> w2)
        // We know that only one of row and column can change at the same time (due to being rectange)
        // We therefore can take the max of the absolute change in either dimension on the journey w1 -> w2

        let w1_pos = w[1].get_position();
        let w2_pos = w[2].get_position();

        // Get magnitude, we have to convert to isize as we may be going negative
        let magnitude = max((w2_pos.0 as isize - w1_pos.0 as isize).abs(),( w2_pos.1 as isize - w1_pos.1 as isize).abs()) as usize;

        match w[1].get_direction() {
            Direction::Up => { (w1_pos.0, w1_pos.1 - magnitude - 1) }
            Direction::Down => { (w1_pos.0, w1_pos.1 + magnitude + 1) }
            Direction::Left => { (w1_pos.0 - magnitude - 1, w1_pos.1) }
            Direction::Right => { (w1_pos.0 + magnitude + 1, w1_pos.1) }
        }
    }).collect::<Vec<(usize, usize)>>();

    println!("New obstacles: {:#?}", new_obstacles);
    println!("New obstacles count: {}", new_obstacles.len());


    Ok(())
}