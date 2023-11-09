use std::num::{ParseFloatError, ParseIntError};
use std::time::Duration;
use std::thread::{self};
use game_of_life::game::Game;

fn main() {

    let mut line = String::new();
    println!("Enter desired size (Windows cmd line max: 30):");
    std::io::stdin().read_line(&mut line).unwrap();
    let size = parse_for_size(line.trim().to_string()).expect("Invalid size!");

    let mut game = Game::new(size as usize);

    println!("Enter desired threshold (in % as 0.xxx):");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let threshold = parse_for_threshold(line.trim().to_string()).expect("Invalid threshold!");

    //initialize board with random seed
    game.init_board(threshold);

    let term = console::Term::stdout();

    let time_step = Duration::from_millis(50);

    loop {
        //clear terminal
        let res = game.update();
        let res_str = res.as_str();                
        term.write_line(res_str).expect("Failed to write to terminal.");
        thread::sleep(time_step);
        term.clear_screen().expect("failed clearing screen");
    }
}






fn parse_for_threshold(line: String) -> Result<f32, ParseFloatError> {
    let num = match line.parse::<f32>() {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    Ok(num)
}

fn parse_for_size(line: String) -> Result<u32, ParseIntError> {
    let num = match line.trim().parse::<u32>() {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    Ok(num)
}

