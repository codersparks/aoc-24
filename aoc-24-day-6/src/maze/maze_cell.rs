use std::fmt::{Display, Formatter};
use aoc_visualisation::traits::ratatui::RatatuiStylised;
use ratatui::style::{Color, Style};
use crate::guard::Direction;

#[derive(Debug, Default)]
pub enum MazeCell {
    #[default]
    Empty,
    Obstacle,
    Guard(Direction),
    Visited,
}

impl Display for MazeCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.representation())
    }
}

impl RatatuiStylised for MazeCell {
    fn get_style(&self) -> Option<Style> {

        let s = match self {
            MazeCell::Empty => { Style::default() }
            MazeCell::Obstacle => { Style::default().bg(Color::Red) }
            MazeCell::Guard(_) => { Style::default().bg(Color::White).fg(Color::Black)}
            MazeCell::Visited => { Style::default().bg(Color::Blue) }
        };
        Some(s)
    }
}

impl MazeCell {

    pub fn representation(&self) -> char {
        match self {
            MazeCell::Empty => { '.' }
            MazeCell::Obstacle => { '#' }
            MazeCell::Visited => { 'X' }
            MazeCell::Guard(d) => {
                match d {
                    Direction::Up => {'^'}
                    Direction::Down => {'v'}
                    Direction::Left => {'<'}
                    Direction::Right => {'>'}
                }
            }
        }
    }
}
