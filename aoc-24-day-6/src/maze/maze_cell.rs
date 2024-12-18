use crate::guard::Direction;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph, Widget};

#[derive(Debug, Default)]
pub enum MazeCell {
    #[default]
    Empty,
    Obstacle,
    Guard(Direction),
    Visited,
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
impl Widget for &MazeCell {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let block = Block::default().borders(ratatui::widgets::Borders::ALL);

        let representation = format!("{}", self.representation());
        let paragraph = Paragraph::new(representation)
            .centered().block(block);
        paragraph.render(area, buf);
    }
}
