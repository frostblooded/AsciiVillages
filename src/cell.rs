use std::fmt::{self, Display, Formatter};

#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Base,
    Tree,
    Worker,
}

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::Base => 'B',
            Cell::Tree => 'T',
            Cell::Worker => 'W',
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
