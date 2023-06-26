use super::{maze, maze::{MAZE_SIZE}};
use std::str;


const BUFFER_SIZE: usize = 10 * MAZE_SIZE * MAZE_SIZE;

pub enum DirectionOfTravel{
    Forward,
    Right,
    Left,
    Backward,
}

pub type StepMap = maze::MazeInfo<u16>;

impl maze::MazeInfo<u16> { // StepMap
    pub fn new() -> Self {
        let map = StepMap{grid: [[0; MAZE_SIZE]; MAZE_SIZE]};

        map
    }
    pub fn calc_step_map(&mut self, maze: &maze::Maze, goal_x: usize, goal_y: usize) {
        let mut no_cell_updated: bool;
        no_cell_updated = false;


        for i in 0..MAZE_SIZE {
            for j in 0.. MAZE_SIZE {
                *self.get_mut(i, j) = 0xFFFE; // Not set to 0xFFFF because it may be +1
            }
        }

        *self.get_mut(goal_x, goal_y) = 0;

        while !no_cell_updated {
            self.display();
            println!("");
            no_cell_updated = true;
            for i in 0..MAZE_SIZE {
                for j in 0..MAZE_SIZE {
                    for direction in maze::TOZAINANBOKU {
                        if maze.get_cell(i,j)[direction] == maze::Wall::Absent {
                            match maze.get_neighbor(i, j, direction){
                                Some(_) => {
                                    let neighbor;
                                    {
                                        neighbor = *self.get_neighbor(i, j, direction).unwrap();
                                    }
                                    let current = self.get_mut(i, j);
                                    if *current > (neighbor+1) {
                                        *current = neighbor + 1;
                                        no_cell_updated = false;
                                    }
                                }
                                None => (),
                            }
                        }
                    }
                }
            } 
        }
    }

    pub fn display(&self) {
        for i in 0..MAZE_SIZE {
            for j in 0..MAZE_SIZE {
                print!("{} ", self.get(i, j));
            }
            println!("");
        }
    }   
}

pub fn decide_direction(maze: &maze::Maze) -> DirectionOfTravel {
    DirectionOfTravel::Forward
}
