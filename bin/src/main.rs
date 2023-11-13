mod reader;
use glob::glob;
use mm_maze_solver::maze;
use mm_maze_solver::solver;
use std::str;

use maze::{Maze, MAZE_SIZE, TOZAINANBOKU};
use solver::{StepMap, StepMapMode};

use crate::solver::decide_direction;

fn main() {
    let mut actual_maze = Maze::new();
    let mut goal_x: usize = 0;
    let mut goal_y: usize = 0;

    let files = glob("assets/*.txt")
        .unwrap()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    println!("{:?}", files);
    for file in files {
        reader::read(
            &mut actual_maze,
            &mut goal_x,
            &mut goal_y,
            String::from(file.to_str().unwrap()),
        );

        for line in actual_maze.lines_iter(goal_x, goal_y) {
            println!("{}", line);
        }

        println!("{}", file.to_str().unwrap());
        if !simulate(&actual_maze, goal_x, goal_y) {
            println!("Cannot reach the goal!");
            break;
        }
    }
}

fn display(stepmap: &StepMap) {
    for i in 0..MAZE_SIZE {
        for j in 0..MAZE_SIZE {
            print!("{:^4X} ", stepmap.get(i, j));
        }
        println!("");
    }
}

fn update_wall(actual_maze: &Maze, row: usize, col: usize, local_maze: &mut Maze) {
    for d in TOZAINANBOKU {
        local_maze.set_wall(row, col, d, actual_maze.get(row, col).get(d));
    }
}

fn simulate(actual_maze: &Maze, goal_x: usize, goal_y: usize) -> bool {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut stepmap = StepMap::new();
    let mut local_maze = Maze::new();

    while x != goal_x || y != goal_y {
        match decide_direction(&local_maze, goal_x, goal_y, y, x, &mut stepmap) {
            Some(dir_to_go) => {
                let update_x: isize;
                let update_y: isize;
                (update_x, update_y) = maze::nsew_to_index(dir_to_go);
                x = ((x as isize) + update_x) as usize;
                y = ((y as isize) + update_y) as usize;

                update_wall(&actual_maze, y, x, &mut local_maze);

                println!("d: {:?}, x: {}, y: {}", dir_to_go, x, y);
            }
            None => {
                println!("Cannot reach the goal!");
                return false;
            }
        }
    }
    true
}
