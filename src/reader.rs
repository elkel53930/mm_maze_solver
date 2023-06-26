use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::maze::{Maze, Direction, Wall, MAZE_SIZE};

enum State{
    Pillar,
    Cell,
    HorizontalWall,
    VerticalWall,
}

struct ReaderStateMachine {
    pub state: State,
    pub row: usize,
    pub col: usize,
}

impl ReaderStateMachine {
    pub const fn new() -> Self {
        ReaderStateMachine {
            state: State::Pillar,
            row: 0,
            col: 0,
        }
    }
}

pub fn read(maze: &mut Maze, goal_x: &mut usize, goal_y: &mut usize, filename: String) {
    let path = Path::new(&filename);
    let mut file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", path.display()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => panic!("couldn't read {}", path.display()),
        Ok(_) => (),
    }

    let mut machine = ReaderStateMachine::new();

    for c in s.chars() {
        match machine.state {
            State::Pillar => {
                match c {
                    '+' => {
                        machine.state = State::HorizontalWall;
                    },
                    _ => panic!("Expected +, found '{}'", c),
                }
            },
            State::HorizontalWall => { // '-' or ' '
                match c {
                    '-' => {
                        if machine.row == MAZE_SIZE {
                            maze.set_wall(MAZE_SIZE-1, machine.col, Direction::South, Wall::Present);
                        } else {
                            maze.set_wall(machine.row, machine.col, Direction::North, Wall::Present);
                        }
                        machine.col += 1;
                        machine.state = State::Pillar;
                        if machine.col > MAZE_SIZE + 1 {
                            panic!("Too many columns in maze");
                        }
                    },
                    ' ' => {
                        machine.col += 1;
                        machine.state = State::Pillar;
                        if machine.col > MAZE_SIZE + 1 {
                            panic!("Too many columns in maze");
                        }
                    },
                    '\n' => {
                        machine.col = 0;
                        machine.state = State::VerticalWall;
                    },
                    _ => panic!("Expected - or Space, found '{}'", c),
                }
            },
            State::VerticalWall => { // '|' or ' '
                match c {
                    '|' => {
                        if machine.col == MAZE_SIZE {
                            maze.set_wall(machine.row, MAZE_SIZE-1, Direction::East, Wall::Present);
                        } else {
                            maze.set_wall(machine.row, machine.col, Direction::West, Wall::Present);
                        }
                        machine.col += 1;
                        machine.state = State::Cell;
                    },
                    ' ' => {
                        machine.col += 1;
                        machine.state = State::Cell;
                    },
                    _ => panic!("Expected |, found '{}'", c), 
                }
            },
            State::Cell => {
                match c {
                    ' ' => {
                        machine.state = State::VerticalWall;
                    },
                    'G' => {
                        machine.state = State::VerticalWall;
                        *goal_x = machine.col-1;
                        *goal_y = machine.row;
                    },
                    'S' => {
                        machine.state = State::VerticalWall;
                    },
                    '\n' => {
                        machine.row += 1;
                        machine.col = 0;
                        machine.state = State::Pillar;
                    },
                    _ => panic!("Expected ' ', found '{}'", c),
                }
            }
        }
    }
}

