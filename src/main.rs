use dotclimber::{board::{Board, Border, point::Point}, display, ai::{ArtificialChooser, naive_chooser::NaiveChooser}};

fn main() {
    let mut board = Board::new(Border { top: 10, right: 20, bottom: 0, left: 0 });
    let enemy = NaiveChooser::new(Point {x: 10, y: 0});
    println!("{}", board.to_string());

    loop {

        loop {
            let available_positions = board.get_move_options();
            let user_choice = display::get_user_move(&board.cur_position, available_positions);
            board.make_move(user_choice);
            println!("{}", board.to_string());
            if !board.current_has_multiple_connections() {
                break;
            }
        }

        loop {
            let enemy_choice = enemy.choose(&board);
            match enemy_choice {
                Some(point) => board.make_move(point),
                None => {break},
            }
            if !board.current_has_multiple_connections() {
                break;
            }
        }

        println!("{}", board.to_string());
    }
}
