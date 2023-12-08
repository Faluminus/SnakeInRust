mod gaming_board;
mod objects;

use std::future::IntoFuture;
use k_board::{Keys,Keyboard};
use std::time::Duration;



#[tokio::main]
async fn main(){
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
        board = keystroke( board).await;
        println!();
        tokio::time::sleep(Duration::new(1,0)).await;

    }
}
async fn keystroke(mut board:gaming_board::gaming_board::Board) -> gaming_board::gaming_board::Board{
    let x = async {Keyboard::new().read_key()};
    let a = x.into_future();
    match a.await{
         Keys::Up=> board.snake_direction(2),
        Keys::Down => board.snake_direction(0),
        Keys::Left => board.snake_direction(1),
        Keys::Right => board.snake_direction(3),
        _ => {}
    };
    board
}

