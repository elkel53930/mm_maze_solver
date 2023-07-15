use core::clone::Clone;
use core::cmp::PartialEq;
use core::fmt::Debug;
use core::marker::Copy;
use core::option::{
    Option,
    Option::{None, Some},
};
use core::prelude::v1::derive;

/*
    - The start is north-west of the maze. The position is (X, Y) = (0, 0)
    - MAZE consists of CELLs, CELLs have WALLs.
*/

pub const MAZE_SIZE: usize = 16;

// The start cell is fixed. Those values are basically used to initialize the wall to the right of the start cell.
const MAZE_START_Y: usize = 0;
const MAZE_START_X: usize = 0;

// Based on the size of the maze, calculate the buffer size required for conversion to a string.
const BUFFER_SIZE: usize = (MAZE_SIZE * 4 + 2) * (MAZE_SIZE * 2 + 1);

pub struct MazeInfo<T> {
    pub grid: [[T; MAZE_SIZE]; MAZE_SIZE],
}

impl<T> MazeInfo<T> {
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[row][col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.grid[row][col]
    }

    pub fn get_neighbor(&self, row: usize, col: usize, direction: Direction) -> Option<&T> {
        match direction {
            Direction::North => {
                if row == 0 {
                    None
                } else {
                    Some(self.get(row - 1, col))
                }
            }
            Direction::East => {
                if col == MAZE_SIZE {
                    None
                } else {
                    Some(self.get(row, col + 1))
                }
            }
            Direction::South => {
                if row == MAZE_SIZE {
                    None
                } else {
                    Some(self.get(row + 1, col))
                }
            }
            Direction::West => {
                if col == 0 {
                    None
                } else {
                    Some(self.get(row, col - 1))
                }
            }
        }
    }

    pub fn get_neighbor_mut(
        &mut self,
        row: usize,
        col: usize,
        direction: Direction,
    ) -> Option<&mut T> {
        match direction {
            Direction::North => {
                if row == 0 {
                    None
                } else {
                    Some(self.get_mut(row - 1, col))
                }
            }
            Direction::East => {
                if col == MAZE_SIZE {
                    None
                } else {
                    Some(self.get_mut(row, col + 1))
                }
            }
            Direction::South => {
                if row == MAZE_SIZE {
                    None
                } else {
                    Some(self.get_mut(row + 1, col))
                }
            }
            Direction::West => {
                if col == 0 {
                    None
                } else {
                    Some(self.get_mut(row, col - 1))
                }
            }
        }
    }
}

pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub const fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    pub fn neighbor_position(self, direction: Direction) -> Position {
        match direction {
            Direction::North => Position {
                row: self.row - 1,
                col: self.col,
            },
            Direction::East => Position {
                row: self.row,
                col: self.col + 1,
            },
            Direction::West => Position {
                row: self.row,
                col: self.col - 1,
            },
            Direction::South => Position {
                row: self.row + 1,
                col: self.col,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Wall {
    Present,
    Absent,
    Unexplored,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn nsew_to_fblr(facing: Direction, direction: Direction) -> DirectionOfTravel {
    match facing {
        Direction::North => match direction {
            Direction::North => DirectionOfTravel::Forward,
            Direction::East => DirectionOfTravel::Right,
            Direction::South => DirectionOfTravel::Backward,
            Direction::West => DirectionOfTravel::Left,
        },
        Direction::East => match direction {
            Direction::North => DirectionOfTravel::Left,
            Direction::East => DirectionOfTravel::Forward,
            Direction::South => DirectionOfTravel::Right,
            Direction::West => DirectionOfTravel::Backward,
        },
        Direction::South => match direction {
            Direction::North => DirectionOfTravel::Backward,
            Direction::East => DirectionOfTravel::Left,
            Direction::South => DirectionOfTravel::Forward,
            Direction::West => DirectionOfTravel::Right,
        },
        Direction::West => match direction {
            Direction::North => DirectionOfTravel::Right,
            Direction::East => DirectionOfTravel::Backward,
            Direction::South => DirectionOfTravel::Left,
            Direction::West => DirectionOfTravel::Forward,
        },
    }
}

pub fn nsew_to_index(direction: Direction) -> (isize, isize) {
    match direction {
        Direction::North => (0, -1),
        Direction::East => (1, 0),
        Direction::South => (0, 1),
        Direction::West => (-1, 0),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DirectionOfTravel {
    Forward,
    Right,
    Left,
    Backward,
}

pub const TOZAINANBOKU: [Direction; 4] = [
    Direction::East,
    Direction::West,
    Direction::South,
    Direction::North,
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Facing {
    Forward,
    Right,
    Left,
    Backward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub north: Wall,
    pub east: Wall,
    pub south: Wall,
    pub west: Wall,
}

impl Cell {
    pub const fn new() -> Self {
        Cell {
            north: Wall::Absent,
            east: Wall::Absent,
            south: Wall::Absent,
            west: Wall::Absent,
        }
    }

    pub const fn get(&self, direction: Direction) -> Wall {
        match direction {
            Direction::North => self.north,
            Direction::East => self.east,
            Direction::South => self.south,
            Direction::West => self.west,
        }
    }
}

impl MazeInfo<Cell> {
    // Maze
    pub fn new() -> Self {
        let mut grid = [[Cell::new(); MAZE_SIZE]; MAZE_SIZE];

        // Set the walls around the maze
        for i in 0..MAZE_SIZE {
            grid[0][i].north = Wall::Present;
            grid[MAZE_SIZE - 1][i].south = Wall::Present;
            grid[i][0].west = Wall::Present;
            grid[i][MAZE_SIZE - 1].east = Wall::Present;
        }

        let mut maze = MazeInfo { grid: grid };

        // The starting cell is walled off except for the front.
        maze.set_wall2(
            MAZE_START_Y,
            MAZE_START_X,
            Direction::East,
            Facing::Right,
            Wall::Present,
        );

        maze
    }

    pub fn set_wall2(
        &mut self,
        row: usize,
        col: usize,
        direction: Direction,
        facing: Facing,
        wall: Wall,
    ) {
        let actual_direction: Direction = match direction {
            Direction::North => match facing {
                Facing::Forward => Direction::North,
                Facing::Right => Direction::East,
                Facing::Backward => Direction::South,
                Facing::Left => Direction::West,
            },
            Direction::East => match facing {
                Facing::Forward => Direction::East,
                Facing::Right => Direction::South,
                Facing::Backward => Direction::West,
                Facing::Left => Direction::North,
            },
            Direction::South => match facing {
                Facing::Forward => Direction::South,
                Facing::Right => Direction::West,
                Facing::Backward => Direction::North,
                Facing::Left => Direction::East,
            },
            Direction::West => match facing {
                Facing::Forward => Direction::West,
                Facing::Right => Direction::North,
                Facing::Backward => Direction::East,
                Facing::Left => Direction::South,
            },
        };
        self.set_wall(row, col, actual_direction, wall)
    }

    pub fn set_wall(&mut self, row: usize, col: usize, direction: Direction, wall: Wall) {
        match direction {
            Direction::North => {
                if row > 0 {
                    self.grid[row][col].north = wall;
                    self.grid[row - 1][col].south = wall;
                }
            }
            Direction::East => {
                if col < MAZE_SIZE - 1 {
                    self.grid[row][col].east = wall;
                    self.grid[row][col + 1].west = wall;
                }
            }
            Direction::South => {
                if row < MAZE_SIZE - 1 {
                    self.grid[row][col].south = wall;
                    self.grid[row + 1][col].north = wall;
                }
            }
            Direction::West => {
                if col > 0 {
                    self.grid[row][col].west = wall;
                    self.grid[row][col - 1].east = wall;
                }
            }
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> &Cell {
        &self.grid[row][col]
    }

    pub fn to_string(&self, goal_x: usize, goal_y: usize) -> [u8; BUFFER_SIZE] {
        let mut s = [b' '; BUFFER_SIZE];

        let mut idx = 0;

        for y in 0..MAZE_SIZE {
            // Write the "top" wall of the cells
            for x in 0..MAZE_SIZE {
                s[idx] = b'+';
                idx += 1;
                s[idx..idx + 3].copy_from_slice(match self.grid[y][x].north {
                    Wall::Present => b"---",
                    Wall::Absent => b"   ",
                    Wall::Unexplored => b"...",
                });
                idx += 3;
            }
            s[idx] = b'+';
            idx += 1;
            s[idx] = b'\n';
            idx += 1;

            // Write the "sides" of the cells
            for x in 0..MAZE_SIZE {
                s[idx..idx + 1].copy_from_slice(match self.grid[y][x].west {
                    Wall::Present => b"|",
                    Wall::Absent => b" ",
                    Wall::Unexplored => b":",
                });
                idx += 1;
                if goal_x == x && goal_y == y {
                    s[idx..idx + 3].copy_from_slice(b" G "); // The cell's space (the goal)
                } else if MAZE_START_X == x && MAZE_START_Y == y {
                    s[idx..idx + 3].copy_from_slice(b" S "); // The cell's space (the start)
                } else {
                    s[idx..idx + 3].copy_from_slice(b"   "); // The cell's space
                }
                idx += 3;
            }
            s[idx..idx + 1].copy_from_slice(match self.grid[y][MAZE_SIZE - 1].east {
                Wall::Present => b"|",
                Wall::Absent => b" ",
                Wall::Unexplored => b":",
            });
            idx += 1;
            s[idx] = b'\n';
            idx += 1;
        }

        // Write the bottom wall of the maze
        for x in 0..MAZE_SIZE {
            s[idx] = b'+';
            idx += 1;
            s[idx..idx + 3].copy_from_slice(match self.grid[MAZE_SIZE - 1][x].south {
                Wall::Present => b"---",
                Wall::Absent => b"   ",
                Wall::Unexplored => b"...",
            });
            idx += 3;
        }
        s[idx] = b'+';
        idx += 1;
        s[idx] = b'\n';

        s
    }
}

pub type Maze = MazeInfo<Cell>;
