use std::fs::read_to_string;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::widgets::Clear;
use aoc_24_day_6::maze::Maze;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("aoc-24-day-6/test_input/day6.txt").unwrap();
    //let input = read_to_string("aoc-24-day-6/input/day6.txt").unwrap();
    let maze = &Maze::from_str(&input);

    println!("Maze: {:#?}", maze);

    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|f| {
            let area = f.area();

            f.render_widget(Clear, area);
            f.render_widget(maze, area);
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