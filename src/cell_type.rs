use std::fmt::{self, Display, Formatter};

#[derive(Clone, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Base,
    Tree,
    Worker,
}

impl CellType {
    fn to_char(&self) -> char {
        match self {
            CellType::Empty => ' ',
            CellType::Base => 'B',
            CellType::Tree => 'T',
            CellType::Worker => 'W',
        }
    }
}

impl Display for CellType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
