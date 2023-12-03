mod objects;
mod snake;
mod gaming_board;

fn main(){
    let objects = objects::objects::Objects::new(1);
    let mut gaming_board = gaming_board::gaming_board::Board::new();
    gaming_board.create(20,20,'#',' ');


}



