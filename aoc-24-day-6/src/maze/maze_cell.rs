
#[derive(Debug, Default)]
pub enum MazeCell {
    #[default]
    Empty,
    Obstacle,
    Visited,
}

impl MazeCell {

    pub fn representation(&self) -> char {
        match self {
            MazeCell::Empty => { '.' }
            MazeCell::Obstacle => { '#' }
            MazeCell::Visited => { 'X' }
        }
    }
}