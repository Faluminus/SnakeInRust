mod clear_screen;
mod gaming_board;
mod objects;

use clear_screen::clear_screen;
use gaming_board::Board;
use k_board::{keyboard::Keyboard, keys::Keys};
use std::{future::IntoFuture, time::Duration};

#[tokio::main]
async fn main() {
    let mut board: Board = Board::new();
    let mut action_code: i32 = 0;
    board.create_board(20, 20, '*', '#', 'o', ' ');
    board.apple_spawn();
    loop {
        if action_code == 100 {
            board.apple_spawn()
        }
        board.snake_cleanup();
        clear_screen();
        board.snake_move_on();
        action_code = board.asses_action();
        board.reset();
        if action_code == 101 {
            break;
        }
        board = keystroke(board).await;
        println!();
        tokio::time::sleep(Duration::new(1, 0)).await;
    }
}

async fn keystroke(mut board: Board) -> Board {
    let x = async { Keyboard::new().read_key() };
    let a = x.into_future();
    match a.await {
        Keys::Up => board.snake_direction(2),
        Keys::Down => board.snake_direction(0),
        Keys::Left => board.snake_direction(1),
        Keys::Right => board.snake_direction(3),
        _ => {}
    };
    board
}
