mod reader;
mod maze;
mod solver;
use std::str;

fn main() {
    let mut maze = maze::Maze::new();
    let mut stepmap = solver::StepMap::new();
    let mut goal_x: usize = 0;  
    let mut goal_y: usize = 0;

    reader::read(&mut maze, &mut goal_x, &mut goal_y, String::from("assets/maze_sample.txt"));
    stepmap.calc_step_map(&maze, solver::StepMapMode::UnexploredAsPresent, goal_x, goal_y);

    let maze_string = maze.to_string(goal_x, goal_y);
    let maze_string = str::from_utf8(&maze_string).expect("Found invalid UTF-8");
    println!("{}", maze_string.trim_end_matches(char::from(0)));
    stepmap.display();
}
