use dotclimber::{board::{Board, Border}, display, ai::{ArtificialChooser, random_chooser::RandomChooser}};

fn main() {
    let mut board = Board::new(Border { top: 10, right: 20, bottom: 0, left: 0 });
    let enemy = RandomChooser {};
    println!("{}", board.to_string());

    loop {
        let available_positions = board.get_move_options();
        let user_choice = display::get_user_move(&board.cur_position, available_positions);
        board.make_move(user_choice);

        let enemy_choice = enemy.choose(&board);
        match enemy_choice {
            Some(point) => board.make_move(point),
            None => {break},
        }

        println!("{}", board.to_string());
    }

}
