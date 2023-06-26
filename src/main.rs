mod reader;
mod maze;
mod solver;
use std::str;

use maze::{Maze, TOZAINANBOKU};
use solver::{StepMap, StepMapMode};

use crate::solver::decide_direction;

fn main() {
    let mut actual_maze = Maze::new();
    let mut goal_x: usize = 0;  
    let mut goal_y: usize = 0;

    reader::read(&mut actual_maze, &mut goal_x, &mut goal_y, String::from("assets/maze_sample.txt"));

    let maze_string = actual_maze.to_string(goal_x, goal_y);
    let maze_string = str::from_utf8(&maze_string).expect("Found invalid UTF-8");
    println!("{}", maze_string.trim_end_matches(char::from(0)));

    simulate(&actual_maze, goal_x, goal_y);    
}

fn update_wall(actual_maze: &Maze, row: usize, col: usize, local_maze :&mut Maze)
{
    for d in TOZAINANBOKU {
        local_maze.set_wall(row, col, d, actual_maze.get(row, col)[d]);
    }
}

fn simulate(actual_maze: &Maze, goal_x: usize, goal_y: usize)
{
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut stepmap = StepMap::new();
    let mut local_maze = Maze::new();

    while x != goal_x || y != goal_y {
        let dir_to_go = 
            decide_direction(&local_maze, goal_x, goal_y, 
                y, x, &mut stepmap);

        let update_x:isize;
        let update_y:isize;
        (update_x, update_y) = maze::nsew_to_index(dir_to_go);
        x = ((x as isize) + update_x) as usize;
        y = ((y as isize) + update_y) as usize;

        update_wall(&actual_maze, y, x, &mut local_maze);

        println!("d: {:?}, x: {}, y: {}",dir_to_go, x, y);
    }
}