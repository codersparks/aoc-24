use std::fs::read_to_string;
use aoc_visualisation::grid::GridVisualiser;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::widgets::Clear;
use aoc_24_day_6::maze::Maze;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let input = read_to_string("aoc-24-day-6/test_input/day6.txt").unwrap();
    let input = read_to_string("aoc-24-day-6/input/day6.txt").unwrap();
    let maze = &Maze::from_str(&input);

    println!("Maze: {:#?}", maze);

    let mut terminal = ratatui::init();

    let area = terminal.get_frame().area();
    let visualiser = GridVisualiser::new(3, 3);



    loop {
        terminal.draw(|f| {

            let area = f.area();
            let no_cols = area.width / 3;
            let no_rows = area.height / 3;

            let view = maze.to_view_sized(no_rows as usize, no_cols as usize);
            f.render_widget(Clear, area);
            visualiser.draw(view, area, f.buffer_mut());
        })?;

        if let Event::Key(key_event) = event::read()? {
            if key_event.code == event::KeyCode::Char('q') {
                break;
            }
        }
    }
    ratatui::restore();
    Ok(())
}