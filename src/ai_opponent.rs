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
            let move_eval: i8 = minimax(playing_board, 3, false, i8::MIN, i8::MAX);

            // undo previous move
            board::set_square_at(playing_board, temp_move.0, temp_move.1, board::EMPTY);
            
            // println!("Move: {}, Evaluation: {}", col, move_eval);
            // check if this move is best
            if move_eval > best_eval {
                best_eval = move_eval;
                best_move = col;
            }
        }
    }

    // println!("Best move: {}", best_move);
    return best_move;
}

fn minimax(playing_board: &mut [char; board::BOARD_SIZE], depth: u32, is_max: bool, mut alpha: i8, mut beta: i8) -> i8 {
    let evaluation: i8 = board::evaluate_board(playing_board);

    if evaluation == board::PLAYER_1_WIN || evaluation == board::PLAYER_2_WIN || depth == 0 { return evaluation; }
    
    if !board::are_moves_left(playing_board) { return board::STALEMATE; }

    if is_max {
        let mut best_eval: i8 = i8::MIN;

        for col in 0..board::BOARD_WIDTH {
            if board::is_column_open_num(playing_board, col) {
                // perform move
                let temp_move: (usize, usize) = board::drop_at_column_num(playing_board, col, board::YELLOW_PIECE);
                
                // evaluate the move
                best_eval = best_eval.max(minimax(playing_board, depth - 1, false, alpha, beta));
                
                // undo previous move
                board::set_square_at(playing_board, temp_move.0, temp_move.1, board::EMPTY);

                // alpha-beta pruning
                alpha = alpha.max(best_eval);
                if beta <= alpha { break; }
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
                best_eval = best_eval.min(minimax(playing_board, depth - 1, true, alpha, beta));

                // undo previous move
                board::set_square_at(playing_board, temp_move.0, temp_move.1, board::EMPTY);

                // alpha-beta pruning
                beta = beta.min(best_eval);
                if beta <= alpha { break; }
            }
        }

        return best_eval;
    }
}