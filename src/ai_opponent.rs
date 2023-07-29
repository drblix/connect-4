#[path ="board.rs"]
mod board;

const NO_COL: usize = 8;

/**
 A recursive operation that retrieves the best possible move considering all possible future moves (up to a certain depth)
 * `playing_board` - the board currently being used in the game
 * `depth` - how far the search should go (higher numbers have a significant performance drop-off)
 * `is_max` - are we maximizing the AI?
 * `alpha` - alpha flag
 * `beta` - beta flag
 */
pub fn minimax(playing_board: &mut [char; board::BOARD_SIZE], depth: u16, is_max: bool, mut alpha: i16, mut beta: i16) -> (usize, i16) {
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
            return (NO_COL, board::evaluate_board(playing_board, board::YELLOW_PIECE));
        }
    }

    // Maximizing the AI
    if is_max {
        let mut eval: i16 = i16::MIN;
        // fetching an initial random column
        let mut column: usize = open_columns[fastrand::usize(..open_columns.len())];

        for col in open_columns {
            // Make initial move
            let temp_move: (usize, usize) = board::drop_at_column(playing_board, col, board::YELLOW_PIECE);

            // Evaluate said move
            let new_eval: (usize, i16) = minimax(playing_board, depth - 1, false, alpha, beta);

            // Undo previous move
            board::set_square_at(playing_board, temp_move.0, temp_move.1, board::EMPTY);

            // Determines if this move is more optimal than the current one
            if new_eval.1 > eval {
                eval = new_eval.1;
                column = col;
            }

            // alpha-beta pruning
            alpha = alpha.max(eval);
            if eval > beta { break; } 
        }

        return (column, eval);
    }
    // Minimizing the player
    else {
        let mut eval: i16 = i16::MAX;
        // fetching an initial random column
        let mut column: usize = open_columns[fastrand::usize(..open_columns.len())];

        for col in open_columns {
            // Make initial move
            let temp_move: (usize, usize) = board::drop_at_column(playing_board, col, board::RED_PIECE);

            // Evaluate said move
            let new_eval: (usize, i16) = minimax(playing_board, depth - 1, true, alpha, beta);

            // Undo previous move
            board::set_square_at(playing_board, temp_move.0, temp_move.1, board::EMPTY);

            // Determines if this move is more optimal than the current one
            if new_eval.1 < eval {
                eval = new_eval.1;
                column = col;
            }

            // alpha-beta pruning
            beta = beta.min(eval);
            if eval < alpha { break; }
        }

        return (column, eval);
    }
}

/**
 Checks if the board is in a terminal state (no more possible moves can be made, or the board is inevitably a win) <br/>
 Returns a tuple for 3 booleans: `0` = player wins, `1` = AI wins, `2` = no more moves available
 * `playing_board` - the board currently being used in the game
 */
fn is_terminal_node(playing_board: &[char; board::BOARD_SIZE]) -> (bool, bool, bool) {
    return (board::is_winning_board(playing_board, board::RED_PIECE), 
            board::is_winning_board(playing_board, board::YELLOW_PIECE), 
            board::get_open_columns(playing_board).len() == 0);
}