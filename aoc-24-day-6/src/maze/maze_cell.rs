use std::fmt::{Debug, Display, Formatter};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use crate::guard::Direction;

#[derive(Default, Clone)]
pub enum MazeCell {
    #[default]
    Empty,
    Obstacle,
    Visited,
    Guard(Direction),
}

impl MazeCell {

    pub fn representation(&self) -> char {
        match self {
            MazeCell::Empty => { '.' }
            MazeCell::Obstacle => { '#' }
            MazeCell::Visited => { 'X' }
            MazeCell::Guard(direction) => {
                match direction {
                    Direction::Up => {'^'}
                    Direction::Down => {'v'}
                    Direction::Left => {'<'}
                    Direction::Right => {'>'}
                }
            }
        }
    }
}

impl Display for MazeCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.representation().to_string())
    }
}

impl Debug for MazeCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.representation().to_string())
    }
}


impl Widget for MazeCell {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let block = Block::bordered()
            .border_set(border::THICK);


        Paragraph::new(self.representation().to_string())
            .centered()
            .block(block)
            .render(area, buf);
    }
}