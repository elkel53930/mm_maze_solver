use super::{maze, maze::{MAZE_SIZE, MazeInfo, Maze, Wall}};

const BUFFER_SIZE: usize = 10 * MAZE_SIZE * MAZE_SIZE;

pub enum DirectionOfTravel{
    Forward,
    Right,
    Left,
    Backward,
}

pub type StepMap = MazeInfo<u16>;

#[derive(Clone, Copy, PartialEq)]
pub enum StepMapMode{
    UnexploredAsAbsent,
    UnexploredAsPresent,
}

impl MazeInfo<u16> { // StepMap
    pub fn new() -> Self {
        let map = StepMap{grid: [[0; MAZE_SIZE]; MAZE_SIZE]};

        map
    }
    pub fn calc_step_map(&mut self, maze: &Maze, mode: StepMapMode, goal_x: usize, goal_y: usize) {
        let mut no_cell_updated: bool;
        no_cell_updated = false;

        fn no_wall_present(mode: StepMapMode, wall: Wall) -> bool {
            match mode {
                StepMapMode::UnexploredAsAbsent => {
                    wall == Wall::Absent || wall == Wall::Unexplored
                },
                StepMapMode::UnexploredAsPresent => {
                    wall == Wall::Absent
                },
            }
        }


        for i in 0..MAZE_SIZE {
            for j in 0.. MAZE_SIZE {
                *self.get_mut(i, j) = 0xFFFE; // Not set to 0xFFFF because it may be +1
            }
        }

        *self.get_mut(goal_x, goal_y) = 0;

        while !no_cell_updated {
            no_cell_updated = true;
            for i in 0..MAZE_SIZE {
                for j in 0..MAZE_SIZE {
                    for direction in maze::TOZAINANBOKU {
                        if no_wall_present(mode,maze.get_cell(i,j)[direction]) {
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
                print!("{:^4X} ", self.get(i, j));
            }
            println!("");
        }
    }   
}

pub fn decide_direction(maze: &Maze) -> DirectionOfTravel {
    DirectionOfTravel::Forward
}
