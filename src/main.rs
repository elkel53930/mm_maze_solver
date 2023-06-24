use mm_maze_solver::maze::Maze;
use std::str;



fn main() {
    let maze = Maze::new(7,7);

    let maze_string = maze.to_string();
    let maze_string = str::from_utf8(&maze_string).expect("Found invalid UTF-8");

    println!("{}", maze_string.trim_end_matches(char::from(0)));
}
