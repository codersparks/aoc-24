#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


impl Direction {
    pub fn from_char(c: char) -> Direction {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Invalid direction: {}", c)
        }
    }

    pub fn representation(&self) -> char {
        match self {
            Direction::Up => { '^'}
            Direction::Down => { 'v' }
                Direction::Left => { '<' }
                Direction::Right => { '>' }
            }
        }
}

#[derive(Debug)]
pub struct GuardLocation {
    direction: Direction,
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl GuardLocation {
    pub fn new(direction: Direction, row: usize, col: usize) -> GuardLocation {
        Self { direction, row, col }
    }
}

#[derive(Debug)]
pub struct Guard {
    position: GuardLocation,
    history: Vec<GuardLocation>,
}

impl Guard {

    pub fn new(direction: Direction, row: usize, col: usize) -> Guard {
        Self {
            position: GuardLocation::new(direction, row, col),
            history: vec![]
        }
    }

    pub fn get_direction(&self) -> &Direction {
        &self.position.direction
    }

    pub fn get_position(&self) -> &GuardLocation {
        &self.position
    }

    pub fn get_history(&self) -> &Vec<GuardLocation> { &self.history }
}



