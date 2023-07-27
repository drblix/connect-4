#[path ="board.rs"]
mod board;

/**
 Gets the best column for the AI to drop a piece into (depending on difficulty)
 * `playing_board` the board being used to play the game
 */
pub fn get_best_move(playing_board: &mut [char; board::BOARD_SIZE]) -> usize {
    let mut best_eval: i8 = i8::MIN;
    let mut best_move: usize = 100;

    for col in 0..board::BOARD_WIDTH {
        if board::is_column_open_num(playing_board, col) {
            // perform move
            let temp_move: (usize, usize) = board::drop_at_column_num(playing_board, col, board::YELLOW_PIECE);

            // evaluate move
            let move_eval: i8 = minimax(playing_board, 0, false, 4);

            // undo previous move
            board::set_square_at(playing_board, temp_move.0, temp_move.1, board::EMPTY);

            // check if this move is best
            if move_eval > best_eval {
                best_eval = move_eval;
                best_move = col;
            }
        }
    }

    return best_move;
}

fn minimax(playing_board: &mut [char; board::BOARD_SIZE], depth: u32, is_max: bool, max_depth: u32) -> i8 {
    let evaluation: i8 = board::evaluate_board(playing_board);

    if evaluation == board::PLAYER_1_WIN || evaluation == board::PLAYER_2_WIN || depth == max_depth { return evaluation; }

    if !board::are_moves_left(playing_board) { return board::STALEMATE; }

    if is_max {
        let mut best_eval: i8 = i8::MIN;

        for col in 0..board::BOARD_WIDTH {
            if board::is_column_open_num(playing_board, col) {
                // perform move
                let temp_move: (usize, usize) = board::drop_at_column_num(playing_board, col, board::YELLOW_PIECE);
                
                // evaluate the move
                best_eval = best_eval.max(minimax(playing_board, depth + 1, !is_max, max_depth));
                
                // undo previous move
                board::set_square_at(playing_board, temp_move.0, temp_move.1, board::EMPTY);
            }
        }
        
        return best_eval;
    }
    else {
        let mut best_eval: i8 = i8::MAX;

        for col in 0..board::BOARD_WIDTH {
            if board::is_column_open_num(playing_board, col) {
                // perform move
                let temp_move: (usize, usize) = board::drop_at_column_num(playing_board, col, board::RED_PIECE);

                // evaluate move
                best_eval = best_eval.min(minimax(playing_board, depth + 1, !is_max, max_depth));

                // undo previous move
                board::set_square_at(playing_board, temp_move.0, temp_move.1, board::EMPTY);
            }
        }

        return best_eval;
    }
}

fn get_letter_from_col(col: usize) -> char {
    match col {
        0 => { 'A' },
        1 => { 'B' },
        2 => { 'C' },
        3 => { 'D' },
        4 => { 'E' },
        5 => { 'F' },
        6 => { 'G' },
        _ => { 'A' },
    }
}