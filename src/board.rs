use colored::{Colorize, CustomColor};

pub const RED_PIECE: char = 'R';
pub const YELLOW_PIECE: char = 'Y';
pub const EMPTY: char = '_';
pub const PIECE_ICON: char = 'O';

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

fn evaluate_section(section: &[char], piece: char) -> i16 {
    let mut score: i16 = 0;
    let opp_piece: char = if piece == RED_PIECE { YELLOW_PIECE } else { RED_PIECE };

    let piece_count: usize = section.iter().filter(|&x| x == &piece).count(); // count_of(section, piece);
    let empty_count: usize = section.iter().filter(|&x| x == &EMPTY).count();
    let opp_count: usize = section.iter().filter(|&x| x == &opp_piece).count();

    // 4 in a row (max priority due to possible win)
    if piece_count == 4 {
        score += 1000;
    }
    // 3 in a row
    else if piece_count == 3 && empty_count == 1 {
        score += 10;
    }
    // 2 in a row
    else if piece_count == 2 && empty_count == 2 {
        score += 3;
    }

    // enemy has a chance to get 4 in a row
    if opp_count == 3 && empty_count == 1 {
        score -= 10;
    }

    return score;
}

pub fn evaluate_board(board: &[char; BOARD_SIZE], piece: char) -> i16 {
    let window_length: usize = 4;
    let mut score: i16 = 0;

    //for x in 0..BOARD_WIDTH {
    //    for y in 0..BOARD_HEIGHT {
    //        println!("{} ({}, {})", get_piece_at(board, x, y), x, y);
    //    }
    //}

    // Sectioning and evaluating each row
    for r in 0..BOARD_HEIGHT {
        // Splitting array to only this row
        let this_row: &[char] = &board[BOARD_WIDTH * r..BOARD_WIDTH * (r + 1)];
        
        for c in 0..(BOARD_WIDTH - 3) {
            let section: &[char] = &this_row[c..(c + window_length)];
            score += evaluate_section(section, piece);
        }
    }

    // Sectioning and evaluating each column
    for c in 0..BOARD_WIDTH {
        // Create array to only this column
        let mut this_col: [char; BOARD_HEIGHT] = [EMPTY; BOARD_HEIGHT];
        for r in 0..BOARD_HEIGHT { this_col[r] = get_piece_at(board, c, r); }

        for r in 0..(BOARD_HEIGHT - 3) {
            let section: &[char] = &this_col[r..(r + window_length)];
            score += evaluate_section(section, piece);
        }
    }

    // Starting position: (3, 5) / middle and bottom of board
    // Sectioning and evaluating each ascending diagonal
    for row in (0..BOARD_HEIGHT).rev() {
        let mut x: usize = 3;
        let mut y: usize = row;

        // Moving down and left in order to encompass all of the diagonal in the sectioning
        while x > 0 && y < (BOARD_HEIGHT - 1) {
            x -= 1;
            y += 1;
        }

        let mut asc_diag_section: Vec<char> = Vec::new();

        // Moving up and right, adding each piece to the section
        while x < (BOARD_WIDTH - 1) && y > 0 {
            asc_diag_section.push(get_piece_at(board, x, y));

            x += 1;
            y -= 1;
        }
        asc_diag_section.push(get_piece_at(board, x, y));

        score += evaluate_section(&asc_diag_section, piece);
        // println!("Ascending: {}", asc_diag_section.iter().count());
    }

    // Starting position: (3, 5) / middle and bottom of board
    // Sectioning and evaluating each descending diagonal
    for row in (0..BOARD_HEIGHT).rev() {
        let mut x: usize = 3;
        let mut y: usize = row;

        // Moving down and right in order to encompass all of the diagonal in the sectioning
        while x < (BOARD_WIDTH - 1) && y < (BOARD_HEIGHT - 1) {
            x += 1;
            y += 1;
        }

        let mut asc_diag_section: Vec<char> = Vec::new();

        // Moving up and left, adding each piece to the section
        while x > 0 && y > 0 {
            asc_diag_section.push(get_piece_at(board, x, y));

            x -= 1;
            y -= 1;
        }
        asc_diag_section.push(get_piece_at(board, x, y));

        score += evaluate_section(&asc_diag_section, piece);
        // println!("Descending: {}", asc_diag_section.iter().count());
    }

    return score;
}

pub fn drop_at_column(board: &mut [char; BOARD_SIZE], col: usize, piece: char) -> (usize, usize) {

    for row in (0..BOARD_HEIGHT).rev() {
        if get_piece_at(board, col, row) == EMPTY {
            set_square_at(board, col, row, piece);
            return (col, row);
        }
    }

    panic!("Column is filled!");
}

pub fn is_column_open(board: &[char; BOARD_SIZE], col: usize) -> bool {
    for row in (0..BOARD_HEIGHT).rev() {
        if get_piece_at(board, col, row) == EMPTY {
            return true;
        }
    }

    return false;
}

pub fn get_open_columns(board: &[char; BOARD_SIZE]) -> Vec<usize> {
    let mut open_columns: Vec<usize> = Vec::new();

    for col in 0..BOARD_WIDTH {
        if is_column_open(board, col) {
            open_columns.push(col);
        }
    }

    return open_columns;
}

pub fn clear_board(playing_board: &mut [char; BOARD_SIZE]) {
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            set_square_at(playing_board, x, y, EMPTY);
        }
    }
}

pub fn get_piece_at(board: &[char; BOARD_SIZE], x: usize, y: usize) -> char { board[x + y * BOARD_WIDTH] }

pub fn set_square_at(board: &mut [char; BOARD_SIZE], x: usize, y: usize, piece: char) { board[x + y * BOARD_WIDTH] = piece }