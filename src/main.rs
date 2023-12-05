mod gaming_board;
mod objects;

fn main() {
    let mut board = gaming_board::gaming_board::Board::new();
    board.create_board(20, 20, '*', '#', 'o', ' ');
    board.apple_spawn();

}
