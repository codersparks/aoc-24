use std::fs::read_to_string;
use std::io;
use std::io::BufRead;
use ratatui::{DefaultTerminal, Frame};
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use aoc_24_day_6::maze::Maze;


fn handle_events() -> bool {

    if let Ok(e) = event::read() {
        match e {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => return true,
                    _ => {}
                }
            }
            _ => {}
        };
    }
    false
}

fn run(maze: &mut Maze, terminal: &mut DefaultTerminal) {
    let mut exit = false;
    loop {
        terminal.draw(|frame| maze.draw(frame));
        exit = handle_events();

    }

}

fn main() {

    let mut terminal = ratatui::init();



    let input = read_to_string("aoc-24-day-6/test_input/day6.txt").unwrap();
    let maze = &mut Maze::from_str(&input);

    run(maze, &mut terminal);

    let stdin = io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();
    ratatui::restore();



}
