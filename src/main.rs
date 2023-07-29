use std::{io::{self, Write}, char};
use colored::{Colorize, CustomColor};

mod board;
mod ai_opponent;

// TODO:
// Highlight latest placed piece
// Highlight 4-in-a-row when game is over
// AI prolongs the game when it doesn't need to; this is usually when there's multiple ways it can win. Almost as if it's "teasing" the player (i guess this is a bug?)
// ^Important to note that it DOESN'T lose when it does this normally
// Odd behaviour if you hold down the enter key when prompted to enter a column to drop a piece

fn main() {
    // Intro card section
    {
        let mut user_y_n = String::new();
    
        print!("Play title card (y/n): ");
        io::stdout().flush().expect("flush failed!");
    
        io::stdin().read_line(&mut user_y_n).expect("failed to read line!");
    
        if user_y_n.to_lowercase().trim() == "y"{
            intro_card();
        }
    }
    
    // Game starts

    // Creating board variable
    let mut playing_board: [char; board::BOARD_SIZE] = [board::EMPTY; board::BOARD_SIZE];

    loop {
        board::clear_board(&mut playing_board);
        
        // container for AI's depth setting
        let mut depth_setting: u16 = 2;

        // bool to track if player wants to play vs AI
        let mut vs_ai: bool = true;

        // Prompt user if they want to play vs an AI
        loop {
            clear_console();

            let mut user_y_n = String::new();

            print!("Play vs AI (y/n): ");
            
            io::stdout().flush().expect("flush failed!");
            io::stdin().read_line(&mut user_y_n).expect("failed to read line!");

            if user_y_n.trim().to_lowercase() == "y" {
                vs_ai = true;
                break;
            }
            else if user_y_n.trim().to_lowercase() == "n" {
                vs_ai = false;
                break;
            }
            else {
                clear_console();
                println!("{}", "invalid choice".red());
                wait_for_seconds(1.5);
            }
        }

        // Prompt user for depth setting (basically difficulty of AI if they aren't doing PvP)
        if vs_ai {
            loop {
                clear_console();
                
                println!("{}", "Search depths:".underline());
                type_writer("1\n2 - easy\n3\n4 - moderate\n5\n6 - difficult\n7\n8\n9 - virtually unbeatable", 0.2, true, CustomColor::new(0, 0, 0));
                
                type_writer("\nHigher depth selections means higher calculation time; I am NOT responsible for frying a school laptop!", 0.2, true, CustomColor::new(0, 0, 0));
                type_writer("Enter the search depth for the AI [1 - 9]: ", 0.2, false, CustomColor::new(0, 0, 0));    
    
                let mut temp_str: String = String::new();
    
                io::stdin().read_line(&mut temp_str).expect("Err reading line!");
    
                let temp_depth: u16 = match temp_str.trim().parse() {
                    Ok(num) => {
                        if num <= 9 && num >= 1 { num }
                        else {
                            clear_console();
                            type_writer("Number is out of range", 1.0, true, CustomColor::new(196,88,76));
                            wait_for_seconds(0.75);
                            continue;
                        }
                    },
                    Err(_) => { 
                        clear_console();
                        type_writer("Err parsing! Enter an integer", 1.5, true, CustomColor::new(196,88,76));
                        wait_for_seconds(0.75);
                        continue;
                    }
                };
    
                depth_setting = temp_depth;
                break;
            }
        }
        else {

        }


        loop {
            let mut player_1_response: String = String::new();

            // Player 1's turn starts
            get_player_col_input(&mut player_1_response, playing_board, &mut String::from("[Player 1]"));
            
            clear_console();
            board::drop_at_column(&mut playing_board, letter_to_col(player_1_response.to_uppercase().chars().next().unwrap()), board::RED_PIECE);
            board::display_board(&playing_board);

            // checking if player 1's move was a winning one
            if check_if_winner(&playing_board, board::RED_PIECE) {
                enter_to_continue();
                break;
            }
            else if board::get_open_columns(&playing_board).len() == 0 {
                type_writer("Tie! No one wins!", 2.0, true, CustomColor::new(19, 194, 22));
                enter_to_continue();
                break;
            }
            // Player 1's move is over



            // Player 2's turn starts (AI branch)
            if vs_ai {
                clear_console();
    
                // Displays board while AI selects move
                board::display_board(&playing_board);
        
                println!("{}", "Thinking...".yellow());
        
                // 2nd value is unused
                let best_col: (usize, i16) = ai_opponent::minimax(&mut playing_board, depth_setting, true, i16::MIN, i16::MAX);
        
                clear_console();
                board::drop_at_column(&mut playing_board, best_col.0, board::YELLOW_PIECE);
                board::display_board(&playing_board);
            }
            // Player 2's turn is over (AI branch)
            // Player 2's turn starts (non-AI branch)
            else {
                let mut player_2_response: String = String::new();
                
                get_player_col_input(&mut player_2_response, playing_board, &mut String::from("[Player 2]"));
                
                clear_console();
                board::drop_at_column(&mut playing_board, letter_to_col(player_2_response.to_uppercase().chars().next().unwrap()), board::YELLOW_PIECE);
                board::display_board(&playing_board);
            }
            // Player 2's turn ends (non-AI branch)

            // check if 2nd player won or if game is tied
            if check_if_winner(&playing_board, board::YELLOW_PIECE) {
                enter_to_continue();
                break;
            }
            else if board::get_open_columns(&playing_board).len() == 0 {
                type_writer("Tie! No one wins!", 2.0, true, CustomColor::new(19, 194, 22));
    
                enter_to_continue();
                break;
            }
        }
    }
}

/**
 Function that retrieves the player's input for a column and validates it
 * `user_response` - the string for the response to be assigned to
 * `playing_board` - the board being used for the game
 */
fn get_player_col_input(user_response: &mut String, playing_board: [char; 42], plr_name: &mut String) {

    // input loop for choosing where to drop a piece
    loop {
        clear_console();
        
        *user_response = String::new();
        board::display_board(&playing_board);
        
        // (0, 0, 0) custom color is just my default for "no color"
        plr_name.push_str(" Enter a column to drop a piece: ");

        type_writer(&plr_name, 0.75, false, CustomColor::new(0, 0, 0));
        // Prevents text afterwards from disappearing randomly
        io::stdout().flush().expect("flush failed!");
        
        // Read the player's input
        io::stdin().read_line(user_response).expect("failed to read line!");
        
        // If player's input is valid, return back to the main function
        // Otherwise, notify the player of invalid input and try again
        if user_response_valid(&(user_response.trim().to_uppercase()), &playing_board) { return; }
        else {
            clear_console();
        
            type_writer("Invalid input!\nColumn not recognized or is already filled", 0.35, true, CustomColor::new(196,88,76));
            wait_for_seconds(2.5);
        
            clear_console();
        }
    }
}

/** Plays the fancy intro card :) */
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
    type_writer("by benjamin n.", 1.5, true, CustomColor::new(0, 0, 0));
    println!();

    enter_to_continue();

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

/**
 Checks if the provided piece is a winner in the current board, then printing the necessary message
 * `playing_board` - the board being used for the game
 * `piece` - the piece to check for (either red piece or yellow piece)
 */
fn check_if_winner(playing_board: &[char; board::BOARD_SIZE], piece: char) -> bool {
    if board::is_winning_board(playing_board, piece) {
        if piece == board::RED_PIECE {
            type_writer("Player 1 wins!", 1.0, true, CustomColor::new(196,88,76));
            return true;
        }
        else {
            type_writer("Player 2 wins!", 1.0, true, CustomColor::new(208,208,23));
            return true;
        }
    }

    return false;
}

/** Helper function to clear console */
fn clear_console() { winconsole::console::clear().unwrap(); }

/** 
 Helper function that prevents the main thread from being paused in order to play a Windows console beep 
 * `freq` - frequency of the beep (hertz)
 * `dur` - duration of the beep (milliseconds)
 */
fn beep_no_pause(freq: u32, dur: u32) { std::thread::spawn(move || winconsole::console::beep(freq, dur)); }

/** 
 Helper function that provides a quick method to pause for a specified amount of seconds 
 * `secs` - duration in seconds to pause the thread
 */
fn wait_for_seconds(secs: f32) { std::thread::sleep(std::time::Duration::from_secs_f32(secs)); }

/**
 Helper function that checks if a users response is valid (utilized in the column input function)
 * `resp` - the input string to check
 * `playing_board` - the board currently being used in the game
 */
fn user_response_valid(resp: &String, playing_board: &[char; 42]) -> bool { (resp == "A" || resp == "B" || resp == "C" || resp == "D" || resp == "E" || resp == "F" || resp == "G") && board::is_column_open(playing_board, letter_to_col(resp.chars().next().unwrap())) } 

/**
 Helper function to convert a character into a usable index for the "playing_board" array
 * `col` - the character to convert (A - G)
 */
fn letter_to_col(col: char) -> usize { col as usize - 65 }

/** Helper function that pauses the thread until the user presses enter */
fn enter_to_continue() {
    println!("\nPress ENTER to continue");

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}