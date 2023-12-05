mod gaming_board;
mod objects;
use std::io;
fn main() {
    let mut board = gaming_board::gaming_board::Board::new();
    let mut action_code = 0;
    board.create_board(20, 20, '*', '#', 'o', ' ');
    loop {
        if action_code == 100 {
           board.apple_spawn()
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
        board.snake_move_on();
        board.snake_cleanup();
        action_code = board.asses_action();
        board.reset();
    }

}
