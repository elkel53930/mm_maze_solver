#![no_std]

/*
    - The start is south-west of the maze. The position is (X, Y) = (0, 15)
    - MAZE consists of CELLs, CELLs have WALLs.
*/

const MAZE_SIZE: usize = 16;

// The start cell is fixed. Those values are basically used to initialize the wall to the right of the start cell.
const MAZE_START_Y: usize = 15;
const MAZE_START_X: usize = 0;

// Based on the size of the maze, calculate the buffer size required for conversion to a string.
const BUFFER_SIZE: usize = (MAZE_SIZE * 4 + 2) * (MAZE_SIZE *2 + 1);

type Position = (usize, usize);


#[derive(Clone, Copy)]
pub enum Wall {
    Present,
    Absent,
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

pub enum Facing {
    Forward,
    Right,
    Left,
    Backward,
}

#[derive(Clone, Copy)]
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
}

pub struct Maze {
    grid: [[Cell; MAZE_SIZE]; MAZE_SIZE],

    // The goal cell varies depending on the situation.
    //  e.g. When returning to the start cell, the goal will be set to the start cell.
    pub goal: Position,
}

impl Maze {
    pub fn new(goal_x:usize, goal_y:usize) -> Self {
        let mut grid = [[Cell::new(); MAZE_SIZE]; MAZE_SIZE];

        // Set the walls around the maze
        for i in 0..MAZE_SIZE {
            grid[0][i].north = Wall::Present;
            grid[MAZE_SIZE-1][i].south = Wall::Present;
            grid[i][0].west = Wall::Present;
            grid[i][MAZE_SIZE-1].east = Wall::Present;
        }

        let mut maze = Maze { grid, goal: (goal_y, goal_x) };

        // The starting cell is walled off except for the front.
        maze.set_wall2(MAZE_START_Y, MAZE_START_X, Direction::North, Facing::Right, Wall::Present);

        maze
    }

    pub fn set_wall2(&mut self, row: usize, col: usize, direction: Direction, facing: Facing, wall: Wall) {
        let actual_direction: Direction = 
            match direction {
                Direction::North => {
                    match facing {
                        Facing::Forward => Direction::North,
                        Facing::Right => Direction::East,
                        Facing::Backward => Direction::South,
                        Facing::Left => Direction::West,

                    }
                },
                Direction::East => {
                    match facing {
                        Facing::Forward => Direction::East,
                        Facing::Right => Direction::South,
                        Facing::Backward => Direction::West,
                        Facing::Left => Direction::North,
                    }
                },
                Direction::South => {
                    match facing {
                        Facing::Forward => Direction::South,
                        Facing::Right => Direction::West,
                        Facing::Backward => Direction::North,
                        Facing::Left => Direction::East,
                    }
                },
                Direction::West => {
                    match facing {
                        Facing::Forward => Direction::West,
                        Facing::Right => Direction::North,
                        Facing::Backward => Direction::East,
                        Facing::Left => Direction::South,
                    }
                }
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
                if col < MAZE_SIZE-1 {
                    self.grid[row][col].east = wall;
                    self.grid[row][col + 1].west = wall;
                }
            }
            Direction::South => {
                if row < MAZE_SIZE-1 {
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

    pub fn to_string(&self) -> [u8; BUFFER_SIZE] {
        let mut s = [b' '; BUFFER_SIZE];

        let mut idx = 0;

        for y in 0..MAZE_SIZE {
            // Write the "top" wall of the cells
            for x in 0..MAZE_SIZE {
                s[idx] = b'+';
                idx += 1;
                s[idx..idx+3].copy_from_slice(
                    match self.grid[y][x].north {
                        Wall::Present => b"---",
                        Wall::Absent => b"   ",
                    }
                );
                idx += 3;
            }
            s[idx] = b'+';
            idx += 1;
            s[idx] = b'\n';
            idx += 1;

            // Write the "sides" of the cells
            for x in 0..MAZE_SIZE {
                s[idx..idx+1].copy_from_slice(
                    match self.grid[y][x].west {
                        Wall::Present => b"|",
                        Wall::Absent => b" ",
                    }
                );
                idx += 1;
                if self.goal.0 == x && self.goal.1 == y {
                    s[idx..idx+3].copy_from_slice(b" G "); // The cell's space (the goal)
                } else if MAZE_START_X == x && MAZE_START_Y == y {
                    s[idx..idx+3].copy_from_slice(b" S "); // The cell's space (the start)
                } else {
                    s[idx..idx+3].copy_from_slice(b"   "); // The cell's space
                }
                idx += 3;
            }
            s[idx..idx+1].copy_from_slice(
                match self.grid[y][MAZE_SIZE - 1].east {
                    Wall::Present => b"|",
                    Wall::Absent => b" ",
                }
            );
            idx += 1;
            s[idx] = b'\n';
            idx += 1;
        }

        // Write the bottom wall of the maze
        for x in 0..MAZE_SIZE {
            s[idx] = b'+';
            idx += 1;
            s[idx..idx+3].copy_from_slice(
                match self.grid[MAZE_SIZE - 1][x].south {
                    Wall::Present => b"---",
                    Wall::Absent => b"   ",
                }
            );
            idx += 3;
        }
        s[idx] = b'+'; 
        idx += 1;
        s[idx] = b'\n';

        s
    }
}
