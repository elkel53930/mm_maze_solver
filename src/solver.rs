use super::maze;

pub enum DirectionOfTravel{
    Forward,
    Right,
    Left,
    Backward,
}

pub fn decide_direction(maze: &maze::Maze) -> DirectionOfTravel {
    DirectionOfTravel::Forward
}
