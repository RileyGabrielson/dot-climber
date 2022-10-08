use dotclimber::{board::{Board, Border}, display};

fn main() {
    let mut board = Board::new(Border { top: 10, right: 20, bottom: 0, left: 0 });
    println!("{}", board.to_string());

    let available_positions = board.get_move_options();
    let choice = display::get_user_move(&board.cur_position, available_positions);

    board.make_move(choice);
    println!("{}", board.to_string());
}
