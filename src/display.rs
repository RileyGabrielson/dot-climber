use crate::board::point::Point;
use std::cmp::Ordering;
use std::io;

pub fn get_user_move(cur_position: &Point, options: Vec<Point>) -> Point {
    for index in 0..options.len() {
        let label = get_point_label(&cur_position, &options[index]);
        println!("{}: {}, ({}, {})", index, label, options[index].x, options[index].y);
    };

    let mut choice: String;
    let mut index: i32 = -1;
    while index < 0 || index >= (options.len() as i32) {
        choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        println!("Input: {}", choice);
        index = choice.trim().parse().unwrap_or_else(|_| -1);
        println!("{}", index);
    };

    options[index as usize]
}

fn get_point_label(cur_position: &Point, option: &Point) -> String {
    match cur_position.x.cmp(&option.x) {
        Ordering::Less => match cur_position.y.cmp(&option.y) {
            Ordering::Less => String::from(""),
            Ordering::Equal => String::from(""),
            Ordering::Greater => String::from(""),
        },
        Ordering::Equal => match cur_position.y.cmp(&option.y) {
            Ordering::Less => String::from("Up"),
            Ordering::Equal => String::from("Invalid"),
            Ordering::Greater => String::from("Down"),
        }
        Ordering::Greater => match cur_position.y.cmp(&option.y) {
            Ordering::Less => String::from(""),
            Ordering::Equal => String::from(""),
            Ordering::Greater => String::from(""),
        }
    }
}


