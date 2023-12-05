mod gaming_board;
mod objects;
use std::io;use std::time::Duration;
use std::thread::sleep;
use stopwatch2;


fn main() {

    let mut board = gaming_board::gaming_board::Board::new();
    let mut action_code = 0;
    board.create_board(20, 20, '*', '#', 'o', ' ');
    board.apple_spawn();
    loop {
        if action_code == 100 {
           board.apple_spawn()
        }
        board.snake_cleanup();
        board.snake_move_on();
        action_code = board.asses_action();
        board.reset();
        if action_code == 101{
            break;
        }
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.to_lowercase().trim() {
                    "w" => {
                        board.snake_direction(2);
                    }
                    "a" => {
                        board.snake_direction(1);
                    }
                    "s" => {
                        board.snake_direction(0);
                    }
                    "d" => {
                        board.snake_direction(3);
                    }
                    _ => {

                    }
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
        println!();
    }

}
