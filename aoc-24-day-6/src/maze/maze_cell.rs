use crate::guard::Direction;

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
