use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn get_inverse(&self) -> Direction {
        match self {
            Direction::Up => { Direction::Down }
            Direction::Down => { Direction::Up }
            Direction::Left => { Direction::Right }
            Direction::Right => { Direction::Left }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GuardLocation {
    pub(crate) direction: Direction,
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl GuardLocation {
    pub fn new(direction: Direction, row: usize, col: usize) -> GuardLocation {
        Self { direction, row, col }
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.row, self.col)
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

    pub fn get_unique_history_count(&self) -> usize {
        self.history.iter().map(|l| (l.row, l.col)).collect::<HashSet<(usize,usize)>>().len()
    }

    pub fn update_history_with_current_position(&mut self) {
        self.history.push(self.position.clone());
    }

    pub fn change_direction(&mut self) {

        let new_direction = match self.position.direction {
            Direction::Up => { Direction::Right }
            Direction::Down => { Direction::Left }
            Direction::Left => { Direction::Up }
            Direction::Right => { Direction::Down }
        };

        self.position.direction = new_direction;
    }

    pub fn update_position(&mut self, row: usize, col: usize) {
        self.position.row = row;
        self.position.col = col;
    }
}



