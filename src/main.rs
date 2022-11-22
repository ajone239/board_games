use board_games::connect_four::{Board, Square};

fn main() {
    let mut board = Board::new();

    board.play_move(1, Square::Yellow);
    println!("Played yellow 1\n{}", board);

    board.play_move(1, Square::Yellow);
    println!("Played yellow 1\n{}", board);

    board.play_move(2, Square::Yellow);
    println!("Played yellow 2\n{}", board);

    board.play_move(2, Square::Red);
    println!("Played red 2\n{}", board);

    board.play_move(3, Square::Red);
    println!("Played red 3\n{}", board);
}
