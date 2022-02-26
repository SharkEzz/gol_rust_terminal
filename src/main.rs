use std::{thread, time::Duration};

mod board;

fn main() {
    let mut board = board::new_board(30, 10);

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}\n", board.get_population());
        board.draw();
        board.compute_next_generation();
        thread::sleep(Duration::from_millis(100));
    }
}
