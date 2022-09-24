mod board;
use board::Board;
mod play;
use play::play_game;

fn main() {
    let board = Board::new(10, 10, 8);

    // board.display();
    // board.trigger(3, 3);
    // board.display();

    let _game = play_game(board);
}
