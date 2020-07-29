use crate::ds::account::position::Position;

#[derive(PartialEq, Debug)]
pub struct Portfolio {
    pub position_history: Vec<Position>,
    pub open_positions: Vec<Position>
}

impl std::fmt::Display for Portfolio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?}, {:#?})", self.position_history, self.open_positions)
    }
}
