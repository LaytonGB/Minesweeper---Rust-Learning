mod board;
use board::Board;
mod play;
use play::play_game;

fn main() {
    let mut board = Board::new(10, 10, 5);

    // board.display();
    // board.trigger(3, 3);
    // board.display();

    let game = play_game(board);
}
