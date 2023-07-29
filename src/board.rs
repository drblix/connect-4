use colored::{Colorize, CustomColor};

pub const RED_PIECE: char = 'R';
pub const YELLOW_PIECE: char = 'Y';
pub const EMPTY: char = '_';
pub const PIECE_ICON: char = 'O';

pub const BOARD_SIZE: usize = BOARD_HEIGHT * BOARD_WIDTH;
pub const BOARD_HEIGHT: usize = 6;
pub const BOARD_WIDTH: usize = 7;


/**
 Function for displaying the board with appropriate colors for all pieces
 * `board` - the board currently being used in the game
 */
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

/**
 Evaluates a sub-section of size 4, returning the score for that sub-section
 * `section` - sub-section (should always be 4)
 * `piece` - the piece to evaluate for
 */
fn evaluate_section(section: &[char], piece: char) -> i16 {
    if section.iter().count() != 4 { panic!("Section must be 4!"); }

    let mut score: i16 = 0;
    let opp_piece: char = if piece == RED_PIECE { YELLOW_PIECE } else { RED_PIECE };

    let piece_count: usize = section.iter().filter(|&x| x == &piece).count(); // count_of(section, piece);
    let empty_count: usize = section.iter().filter(|&x| x == &EMPTY).count();
    let opp_count: usize = section.iter().filter(|&x| x == &opp_piece).count();

    // 4 in a row (max priority due to possible win)
    if piece_count == 4 {
        score += 100;
    }
    // 3 in a row
    else if piece_count == 3 && empty_count == 1 {
        score += 5;
    }
    // 2 in a row
    else if piece_count == 2 && empty_count == 2 {
        score += 2;
    }

    // enemy has a chance to get 4 in a row
    if opp_count == 3 && empty_count == 1 {
        score -= 4;
    }

    return score;
}

/**
 Evaluates the entire board by dividing rows, columns, and diagonals into smaller sub-sections that are each individually evaluated. Then, summing those evaluations up into an encompassing score
 * `board` - the board currently in the game
 * `piece` - the piece to evaluate for
 */
pub fn evaluate_board(board: &[char; BOARD_SIZE], piece: char) -> i16 {
    let window_length: usize = 4;
    let mut score: i16 = 0;

    // Scoring the center column (makes the AI prefer putting pieces in this column)
    let mut center_column: [char; BOARD_HEIGHT] = [EMPTY; BOARD_HEIGHT];
    for r in 0..BOARD_HEIGHT { center_column[r] = get_piece_at(board, 3, r); }
    let center_count: i16 = center_column.iter().filter(|&x| x == &piece).count() as i16;

    score += center_count * 4;

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


        let section_size: usize = asc_diag_section.iter().count();
        for i in 0..section_size {
            if i + window_length < section_size {
                let sub_section: &[char] = &asc_diag_section[i..(i + window_length)];
                score += evaluate_section(sub_section, piece)
            }
        }
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

        let mut dsc_diag_section: Vec<char> = Vec::new();

        // Moving up and left, adding each piece to the section
        while x > 0 && y > 0 {
            dsc_diag_section.push(get_piece_at(board, x, y));

            x -= 1;
            y -= 1;
        }
        dsc_diag_section.push(get_piece_at(board, x, y));


        let section_size: usize = dsc_diag_section.iter().count();
        for i in 0..section_size {
            if i + window_length < section_size {
                let sub_section: &[char] = &&dsc_diag_section[i..(i + window_length)];
                score += evaluate_section(sub_section, piece)
            }
        }
    }

    return score;
}

/**
 Drops a piece down the specified column, being placed at the first empty spot from the bottom
 * `board` - the board currently in the game
 * `col` - the column to drop the piece
 * `piece` - the piece to drop
 */
pub fn drop_at_column(board: &mut [char; BOARD_SIZE], col: usize, piece: char) -> (usize, usize) {

    for row in (0..BOARD_HEIGHT).rev() {
        if get_piece_at(board, col, row) == EMPTY {
            set_square_at(board, col, row, piece);
            return (col, row);
        }
    }

    panic!("Column is filled!");
}

/**
 Checks if a specified column is open (not filled), returning true if so
 * `board` - the board currently in the game
 * `col` - the column to check
 */
pub fn is_column_open(board: &[char; BOARD_SIZE], col: usize) -> bool {
    for row in (0..BOARD_HEIGHT).rev() {
        if get_piece_at(board, col, row) == EMPTY {
            return true;
        }
    }

    return false;
}

/**
 Gets a vector of column indicies that are not filled
 * `board` - the board currently in the game
 */
pub fn get_open_columns(board: &[char; BOARD_SIZE]) -> Vec<usize> {
    let mut open_columns: Vec<usize> = Vec::new();

    for col in 0..BOARD_WIDTH {
        if is_column_open(board, col) {
            open_columns.push(col);
        }
    }

    return open_columns;
}

/**
 Sets all squares in the board to empty
 * `playing_board` - the board currently in the game
 */
pub fn clear_board(playing_board: &mut [char; BOARD_SIZE]) {
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            set_square_at(playing_board, x, y, EMPTY);
        }
    }
}

/**
 Checks if the board at its current state has a 4-in-a-row for a specified piece
 * `board` - the board currently in the game
 * `piece` - the piece to check for 4-in-a-row
 */
pub fn is_winning_board(board: &[char; BOARD_SIZE], piece: char) -> bool {
    // Check each row for 4 in a row
    for r in 0..BOARD_HEIGHT {
        for c in 0..(BOARD_WIDTH - 3) {
            if get_piece_at(board, c, r) == piece && 
               get_piece_at(board, c + 1, r) == piece && 
               get_piece_at(board, c + 2, r) == piece && 
               get_piece_at(board, c + 3, r) == piece 
               { return true; }
        }
    }

    // Check each column for 4 in a row
    for c in 0..BOARD_WIDTH {
        for r in 0..(BOARD_HEIGHT - 3) {
            if get_piece_at(board, c, r) == piece && 
               get_piece_at(board, c, r + 1) == piece && 
               get_piece_at(board, c, r + 2) == piece && 
               get_piece_at(board, c, r + 3) == piece 
               { return true; }
        }
    }

    // Check each ascending diagonal for 4 in a row
    // Starting position is (3, 5)
    let mut last_piece: char = ' ';
    for row in (0..BOARD_HEIGHT).rev() {
        let mut diag_sum: u8 = 0;
        let mut x: usize = 3;
        let mut y: usize = row;

        // Moving down and left in order to encompass all of the diagonal
        while x > 0 && y < (BOARD_HEIGHT - 1) {
            x -= 1;
            y += 1;
        }

        while x < (BOARD_WIDTH - 1) && y > 0 {
            if get_piece_at(board, x, y) != last_piece {
                diag_sum = 0;
            }

            if get_piece_at(board, x, y) == piece {
                diag_sum += 1;
            }

            last_piece = get_piece_at(board, x, y);

            if diag_sum == 4 { return true; }

            x += 1;
            y -= 1;
        }
    }

    // Check each descending diagonal for 4 in a row
    // Starting position is (3, 5)
    for row in (0..BOARD_HEIGHT).rev() {
        let mut diag_sum: u8 = 0;
        let mut x: usize = 3;
        let mut y: usize = row;

        // Moving down and right in order to encompass all of the diagonal in the sectioning
        while x < (BOARD_WIDTH - 1) && y < (BOARD_HEIGHT - 1) {
            x += 1;
            y += 1;
        }

        // Moving up and left, adding each piece to the section
        while x > 0 && y > 0 {
            if get_piece_at(board, x, y) != last_piece {
                diag_sum = 0;
            }

            if get_piece_at(board, x, y) == piece {
                diag_sum += 1;
            }

            last_piece = get_piece_at(board, x, y);

            if diag_sum == 4 { return true; }

            x -= 1;
            y -= 1;
        }
    }
    
    return false;
}

/**
 Gets the piece at the specified (x, y) coordinates
 * `board` - the board currently in the game
 * `x` - column (usually)
 * `y` - row (usually)
 */
pub fn get_piece_at(board: &[char; BOARD_SIZE], x: usize, y: usize) -> char { board[x + y * BOARD_WIDTH] }

/**
 Sets the square at the specified (x, y) coordinates to a specified piece
 * `board` - the board currently in the game
 * `x` - column (usually)
 * `y` - row (usually)
 * `piece` - the piece to set
 */
pub fn set_square_at(board: &mut [char; BOARD_SIZE], x: usize, y: usize, piece: char) { board[x + y * BOARD_WIDTH] = piece }