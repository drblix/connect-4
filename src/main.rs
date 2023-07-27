use std::{io::{self, Write}, char};
use colored::{Colorize, CustomColor};

mod board;
mod ai_opponent;

fn main() {
    // Intro card section
    
    let mut user_y_n = String::new();

    print!("Play title card (y/n): ");
    io::stdout().flush().expect("flush failed!");

    io::stdin().read_line(&mut user_y_n).expect("failed to read line!");

    if user_y_n.to_lowercase().trim() == "y"{
        intro_card();
    }

    
    
    // Game starts
    
    // Creating board variable
    let mut playing_board: [char; board::BOARD_SIZE] = [board::EMPTY; board::BOARD_SIZE];

    let mut user_response: String;
    
    loop {

        // Player 1's turn starts
        // input loop for choosing where to drop a piece
        loop {
            clear_console();
            user_response = String::new();
            board::display_board(&playing_board);
            
            type_writer("Enter a column to drop a piece: ", 0.75, false, CustomColor::new(0, 0, 0));
            io::stdout().flush().expect("flush failed!");
            
            io::stdin().read_line(&mut user_response).expect("failed to read line!");
            
            if user_response_valid(&(user_response.trim().to_uppercase()), &playing_board) { break; }
            else {
                clear_console();
                
                type_writer("Invalid input!\nColumn not recognized or is already filled", 0.35, true, CustomColor::new(196,88,76));
                wait_for_seconds(2.5);
                
                clear_console();
            }
        }
        
        board::drop_at_column(&mut playing_board, user_response.to_uppercase().chars().next().unwrap(), board::RED_PIECE);
        clear_console();
        board::display_board(&playing_board);

        // Player 1's move is over

        // checking if player 1's move was a winning one
        if check_if_winner(&playing_board) {
            break;
        }

        // clear_console();
        // Player 2's turn starts
        let best_col: usize = ai_opponent::get_best_move(&mut playing_board);

        // type_writer("Thinking...", 1.5, true, CustomColor::new(208,208,23));
        // wait_for_seconds(2.0);

        board::drop_at_column_num(&mut playing_board, best_col, board::YELLOW_PIECE);
        board::display_board(&playing_board);
        
        if check_if_winner(&playing_board) {
            break;
        }
    }
}

fn intro_card() {
    clear_console();

    let mut freq: u32 = 300;
    let title_card: [&str; 6] = [
        "   _____                                  _            _  _   ",
        "  / ____|                                | |          | || |  ",
        " | |      ___   _ __   _ __    ___   ___ | |_  ______ | || |_ ",
        " | |     / _ \\ | '_ \\ | '_ \\  / _ \\ / __|| __||______||__   _|",
        " | |____| (_) || | | || | | ||  __/| (__ | |_            | |  ",
        "  \\_____|\\___/ |_| |_||_| |_| \\___| \\___| \\__|           |_|  "
    ];

    for card in title_card {
        type_writer(card, 0.35, true, CustomColor::new(0, 0, 0));
        
        // creating a thread to stop the pausing of playing the beep
        beep_no_pause(freq, 250);

        freq += 100;
    }

    println!();

    wait_for_seconds(1.5);
    clear_console();
}

/**
 Characters of a string gradually appear on the screen
 * `message` - the string to display
 * `duration` - how long until all characters are displayed
 * `new_line` - should a new line be printed after all characters are finished printing?
 */
fn type_writer(message: &str, duration: f32, new_line: bool, text_color: CustomColor) {
    let is_colored: bool = !(text_color.r == 0 && text_color.g == 0 && text_color.b == 0);
    let char_amnt: usize = message.chars().count();
    let wait_time: f32 = duration / char_amnt as f32;
    
    for character in message.chars() {
        if !is_colored {
            print!("{}", character);
        }
        else {
            print!("{}", character.to_string().custom_color(text_color));
        }

        io::stdout().flush().expect("flush failed!");
        wait_for_seconds(wait_time);
    }

    if new_line {
        println!();
    }
}

fn check_if_winner(playing_board: &[char; board::BOARD_SIZE]) -> bool {
    match board::evaluate_board(playing_board) {
        board::PLAYER_1_WIN => {
            type_writer("Player 1 wins!", 1.0, true, CustomColor::new(196,88,76));
            return true;
        },

        board::PLAYER_2_WIN => {
            type_writer("Player 2 wins!", 1.0, true, CustomColor::new(208,208,23));
            return true;
        }

        _ => { return false; }
    }
}

fn clear_console() { winconsole::console::clear().unwrap(); }

fn beep_no_pause(freq: u32, dur: u32) { std::thread::spawn(move || winconsole::console::beep(freq, dur)); }

fn wait_for_seconds(secs: f32) { std::thread::sleep(std::time::Duration::from_secs_f32(secs)); }

fn user_response_valid(resp: &String, playing_board: &[char; 42]) -> bool { (resp == "A" || resp == "B" || resp == "C" || resp == "D" || resp == "E" || resp == "F" || resp == "G") && board::is_column_open(playing_board, resp.chars().next().unwrap()) } 