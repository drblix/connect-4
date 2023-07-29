use crate::board::evaluate_board;

#[path ="board.rs"]
mod board;

pub const NO_COL: usize = 8;

// TODO:
// alpha-beta pruning optimization
pub fn minimax(playing_board: &mut [char; board::BOARD_SIZE], depth: u16, is_max: bool) -> (usize, i16) {
    let open_columns: Vec<usize> = board::get_open_columns(playing_board);

    // .0 = player won
    // .1 = AI won
    // .2 = no more valid moves
    let is_terminal: (bool, bool, bool) = is_terminal_node(playing_board);

    if (is_terminal.0 || is_terminal.1 || is_terminal.2) || depth == 0 {
        if is_terminal.0 {
            return (NO_COL, i16::MIN);
        }
        else if is_terminal.1 {
            return (NO_COL, i16::MAX);
        }
        else if is_terminal.2 {
            return (NO_COL, 0);
        }
        // Reached end of depth
        else {
            return (NO_COL, evaluate_board(playing_board, board::YELLOW_PIECE));
        }
    }

    // Maximizing the AI
    if is_max {
        let mut eval: i16 = i16::MIN;
        let mut column: usize = 0;

        for col in open_columns {
            // Make initial move
            let temp_move: (usize, usize) = board::drop_at_column(playing_board, col, board::YELLOW_PIECE);

            // Evaluate said move
            let new_eval: (usize, i16) = minimax(playing_board, depth - 1, false);

            // Undo previous move
            board::set_square_at(playing_board, temp_move.0, temp_move.1, board::EMPTY);

            if new_eval.1 > eval {
                eval = new_eval.1;
                column = col;
            }
        }

        return (column, eval);
    }
    // Minimizing the player
    else {
        let mut eval: i16 = i16::MAX;
        let mut column: usize = 0;

        for col in open_columns {
            // Make initial move
            let temp_move: (usize, usize) = board::drop_at_column(playing_board, col, board::RED_PIECE);

            // Evaluate said move
            let new_eval: (usize, i16) = minimax(playing_board, depth - 1, true);

            // Undo previous move
            board::set_square_at(playing_board, temp_move.0, temp_move.1, board::EMPTY);

            if new_eval.1 < eval {
                eval = new_eval.1;
                column = col;
            }
        }

        return (column, eval);
    }
}

fn is_terminal_node(playing_board: &[char; board::BOARD_SIZE]) -> (bool, bool, bool) {
    return (board::evaluate_board(playing_board, board::RED_PIECE) >= 950, 
            board::evaluate_board(playing_board, board::YELLOW_PIECE) >= 950, 
            board::get_open_columns(playing_board).len() == 0);
}