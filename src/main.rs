fn main() {
    let mut game = reversi::game::Game::init(reversi::disk::Disk::B);
    println!("{}", game.get_board());
    println!("next: {}\n", game.get_next_disk());
    loop {
        progress(&mut game);
    }
}

fn progress(game: &mut reversi::game::Game) {
    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut input) {
        eprintln!("{}", e);
    }
    let first_digit: usize = match input.chars().nth(0) {
        Some(char) => match char.to_digit(10) {
            Some(i) => i as usize,
            None => {
                eprintln!("input is invalid!");
                return;
            },
        },
        None => {
            eprintln!("input is invalid!");
            return;
        },
    };
    let second_digit: usize = match input.chars().nth(1) {
        Some(char) => match char.to_digit(10) {
            Some(i) => i as usize,
            None => {
                eprintln!("input is invalid!");
                return;
            },
        },
        None => {
            eprintln!("input is invalid!");
            return;
        },
    };

    if let Err(e) = game.set_disk_and_progress(first_digit, second_digit) {
        eprintln!("invalid position! {}", e);
        return;
    }

    println!("{}", game.get_board());
    println!("next: {}\n", game.get_next_disk());
}