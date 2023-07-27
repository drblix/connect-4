use colored::{Colorize, CustomColor};

pub const RED_PIECE: char = 'R';
pub const YELLOW_PIECE: char = 'Y';
pub const EMPTY: char = '_';
pub const PIECE_ICON: char = 'O';

pub const PLAYER_1_WIN: i8 = 4;
pub const PLAYER_2_WIN: i8 = -4;
pub const STALEMATE: i8 = 0;

pub const BOARD_SIZE: usize = BOARD_HEIGHT * BOARD_WIDTH;
pub const BOARD_HEIGHT: usize = 6;
pub const BOARD_WIDTH: usize = 7;



pub fn display_board(board: &[char; BOARD_SIZE]) {
    let board_color: CustomColor = CustomColor::new(36, 101, 181);

    println!("  A   B   C   D   E   F   G");
    println!("  ↓   ↓   ↓   ↓   ↓   ↓   ↓");
    println!("{}", "_____________________________".custom_color(board_color));
    
    for x in 0..BOARD_HEIGHT {
        for y in 0..BOARD_WIDTH {
            let piece: char = get_piece_at(board, y, x);

            if y != 6 {
                print!("{}", "| ".custom_color(board_color));
                
                if piece == EMPTY {
                    print!("{} ", piece.to_string());
                }
                else if piece == RED_PIECE {
                    print!("{} ", PIECE_ICON.to_string().red());
                }
                else {
                    print!("{} ", PIECE_ICON.to_string().yellow());
                }
            }
            else {
                print!("{}", "| ".custom_color(board_color));
                
                if piece == EMPTY {
                    print!("{}", piece.to_string());
                }
                else if piece == RED_PIECE {
                    print!("{}", PIECE_ICON.to_string().red());
                }
                else {
                    print!("{}", PIECE_ICON.to_string().yellow());
                }

                print!("{}", " |".custom_color(board_color));
            }
        }
        
        if x != BOARD_HEIGHT - 1 {
            println!("{}", "\n|---|---|---|---|---|---|---|".custom_color(board_color));
        }
    }

    println!("{}", "\n‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾".custom_color(board_color));
}

pub fn evaluate_board(board: &[char; BOARD_SIZE]) -> i8 {
    let owned_board: [char; 42] = board.to_owned();

    let col_thread: std::thread::JoinHandle<i8> = std::thread::spawn(move || {
        let mut last_piece: char = ' ';

        // iterates top-bottom thru each column
        for x in 0..BOARD_WIDTH {
            let mut col_sum: i8 = 0;

            for y in 0..BOARD_HEIGHT {
                if last_piece != get_piece_at(&owned_board, x, y) {
                    col_sum = 0;
                }

                col_sum += get_piece_value(&get_piece_at(&owned_board, x, y));
                last_piece = get_piece_at(&owned_board, x, y);
                
                if col_sum == PLAYER_1_WIN {
                    return PLAYER_1_WIN;
                }
                else if col_sum == PLAYER_2_WIN {
                    return PLAYER_2_WIN;
                }
            }
        }

        return STALEMATE;
    });

    let row_thread: std::thread::JoinHandle<i8> = std::thread::spawn(move || {
        let mut last_piece: char = ' ';
    
        // iterates left-right thru each row
        for y in 0..BOARD_HEIGHT {
            let mut row_sum: i8 = 0;

            for x in 0..BOARD_WIDTH {
                if last_piece != get_piece_at(&owned_board, x, y) {
                    row_sum = 0;
                }

                row_sum += get_piece_value(&get_piece_at(&owned_board, x, y));
                last_piece = get_piece_at(&owned_board, x, y);

                if row_sum == PLAYER_1_WIN {
                    return PLAYER_1_WIN;
                }
                else if row_sum == PLAYER_2_WIN {
                    return PLAYER_2_WIN;
                }
            }
        }

        return STALEMATE;
    });
    
    let dec_diag_thread: std::thread::JoinHandle<i8> = std::thread::spawn(move || {
        let mut last_piece: char = ' ';

        // starting square is (0, 2)
        // iterates descending diagonally thru entire board
        // checks for all '\' diagonals
        for x in 0..BOARD_WIDTH {
            let mut diag_sum: i8 = 0;
            let mut temp_x: usize = x;
            let mut temp_y: usize = 2; // our starting y-value is 2

            // moving up and left until out of range
            while temp_x > 0 && temp_y > 0 {
                temp_x -= 1;
                temp_y -= 1;
            }

            // moving down and right
            // moving back through entire diagonal and summing up all pieces
            while temp_x < BOARD_WIDTH && temp_y < BOARD_HEIGHT {
                if last_piece != get_piece_at(&owned_board, temp_x, temp_y) {
                    diag_sum = 0;
                }

                diag_sum += get_piece_value(&get_piece_at(&owned_board, temp_x, temp_y));
                
                last_piece = get_piece_at(&owned_board, temp_x, temp_y);

                if diag_sum == PLAYER_1_WIN {
                    return PLAYER_1_WIN;
                }
                else if diag_sum == PLAYER_2_WIN {
                    return PLAYER_2_WIN;
                }

                temp_x += 1;
                temp_y += 1;
            }
        }

        return STALEMATE;
    });
    
    let asc_diag_thread: std::thread::JoinHandle<i8> = std::thread::spawn(move || {
        let mut last_piece: char = ' ';

        // starting square is (3, 5)
        // iterates ascending diagonally thru entire board
        // checks for all '/' diagonals
        for y in (0..BOARD_HEIGHT).rev() {
            let mut diag_sum: i8 = 0;
            let mut temp_x: usize = 3; // our starting x-value is 3
            let mut temp_y: usize = y;

            // moving down and left until out of range
            while temp_x > 0 && temp_y < BOARD_HEIGHT - 1 {
                temp_x -= 1;
                temp_y += 1;
            }

            // moving up and right
            while temp_x < BOARD_WIDTH {
                if last_piece != get_piece_at(&owned_board, temp_x, temp_y) {
                    diag_sum = 0;
                }
                
                diag_sum += get_piece_value(&get_piece_at(&owned_board, temp_x, temp_y));

                last_piece = get_piece_at(&owned_board, temp_x, temp_y);

                if diag_sum == PLAYER_1_WIN {
                    return PLAYER_1_WIN;
                }
                else if diag_sum == PLAYER_2_WIN {
                    return PLAYER_2_WIN;
                }

                temp_x += 1;

                if temp_y != 0 {
                    temp_y -= 1;
                }
                else {
                    break;
                }
            }
        }

        return STALEMATE;
    });
    
    let col_eval: i8 = col_thread.join().unwrap();
    let row_eval: i8 = row_thread.join().unwrap();
    let dec_diag_eval: i8 = dec_diag_thread.join().unwrap();
    let asc_diag_eval: i8 = asc_diag_thread.join().unwrap();

    if col_eval == PLAYER_1_WIN || row_eval == PLAYER_1_WIN || dec_diag_eval == PLAYER_1_WIN || asc_diag_eval == PLAYER_1_WIN {
        return PLAYER_1_WIN;
    }
    else if col_eval == PLAYER_2_WIN || row_eval == PLAYER_2_WIN || dec_diag_eval == PLAYER_2_WIN || asc_diag_eval == PLAYER_2_WIN {
        return PLAYER_2_WIN;
    }
    else {
        return STALEMATE;
    }
}

pub fn drop_at_column(board: &mut [char; BOARD_SIZE], letter: char, piece: char) {
    // (A, G) [65, 71]
    // [0, 6]
    let col_index: usize = (letter as usize) - 65;
    
    for ind in (0..BOARD_HEIGHT).rev() {
        if get_piece_at(board, col_index, ind) == EMPTY {
            set_square_at(board, col_index, ind, piece);
            return;
        }
    }
}

pub fn drop_at_column_num(board: &mut [char; BOARD_SIZE], col: usize, piece: char) -> (usize, usize) {
    for row in (0..BOARD_HEIGHT).rev() {
        if get_piece_at(board, col, row) == EMPTY {
            set_square_at(board, col, row, piece);
            return (col, row);
        }
    }

    panic!("Column is filled!");
}

pub fn is_column_open(board: &[char; BOARD_SIZE], letter: char) -> bool {
    let col_index: usize = (letter as usize) - 65;

    for ind in (0..BOARD_HEIGHT).rev() {
        if get_piece_at(board, col_index, ind) == EMPTY {
            return true;
        }
    }

    return false;
}

pub fn is_column_open_num(board: &[char; BOARD_SIZE], col: usize) -> bool {
    for row in (0..BOARD_HEIGHT).rev() {
        if get_piece_at(board, col, row) == EMPTY {
            return true;
        }
    }

    return false;
}

pub fn are_moves_left(board: &[char; BOARD_SIZE]) -> bool {
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            if get_piece_at(board, x, y) == EMPTY {
                return true;
            }
        }
    }

    return false;
}

pub fn is_piece_below_at(board: &[char; BOARD_SIZE], x: usize, y: usize) -> bool {
    // piece is on bottom
    if y == BOARD_HEIGHT - 1 { return true; }
    else {
        return get_piece_at(board, x, y + 1) != EMPTY;
    }
}

pub fn clear_board(playing_board: &mut [char; BOARD_SIZE]) {
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            set_square_at(playing_board, x, y, EMPTY);
        }
    }
}

// #[track_caller]
pub fn get_piece_at(board: &[char; BOARD_SIZE], x: usize, y: usize) -> char { /* println!("{}", std::panic::Location::caller().line()); */ board[x + y * BOARD_WIDTH] }

pub fn set_square_at(board: &mut [char; BOARD_SIZE], x: usize, y: usize, piece: char) { board[x + y * BOARD_WIDTH] = piece }

pub fn get_piece_value(piece: &char) -> i8 {
    match piece {
        &EMPTY => { return 0; },
        &RED_PIECE => { return 1; },
        &YELLOW_PIECE => { return -1; },
        _ => { return 0; }
    }
}