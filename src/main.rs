use std::{
    thread,
    time::{Duration, SystemTime},
};

mod board;

fn main() {
    let mut board = board::new_board(80, 40);

    let mut time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut iterations = 0;

    print!("{}[2J", 27 as char);

    loop {
        print!("{esc}[1;1H", esc = 27 as char);
        println!("Population: {}\n", board.get_population());
        board.draw();
        board.compute_next_generation();
        if SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            == time
        {
            iterations += 1;
        } else {
            println!("FPS: {}", iterations);
            iterations = 0;
        }

        time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        thread::sleep(Duration::from_millis(100));
    }
}
