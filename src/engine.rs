use crate::{Board, Player};

// Max depth for 3x3 is = 9 (all cells filled)
// Max depth for 5x9 is = 45 (all cells filled)
// Score: +100 for X win, -100 for O win, 0 for draw
// Depth is subtracted from score to prefer faster wins and slower losses

pub fn minimax(
    board: &mut Board,
    depth: i32,
    max_depth: i32,
    is_maximizing: bool,
    computer_player: Player,
) -> i32 {
    // Base Check: Game Over
    if let Some(winner) = board.check_winner() {
        return if winner == computer_player {
            100 - depth
        } else {
            -100 + depth
        };
    }

    if board.is_draw() {
        return 0;
    }

    if depth >= max_depth {
        return 0;
    }

    if is_maximizing {
        // This boolean tells the function:
        // - ✅ computer's turn (try to get a high score)
        // -    human's turn (try to get a low score).

        let mut best_score = i32::MIN;
        for (r, c) in board.get_legal_moves() {
            board.make_move(r, c);
            let score = minimax(board, depth + 1, max_depth, false, computer_player);
            board.undo_move(r, c);
            best_score = best_score.max(score);
        }
        best_score
    }
    // minimize (human's turn)
    else {
        let mut best_score = i32::MAX;
        for (r, c) in board.get_legal_moves() {
            board.make_move(r, c);
            let score = minimax(board, depth + 1, max_depth, true, computer_player);
            board.undo_move(r, c);
            best_score = best_score.min(score);
        }
        best_score
    }
}

pub fn minimax_optimized(
    board: &mut Board,
    depth: i32,
    max_depth: i32,
    mut alpha: i32, // The best score Max can guarantee
    mut beta: i32,
    is_maximizing: bool,
    computer_player: Player,
) -> i32 {
    if let Some(winner) = board.check_winner() {
        return if winner == computer_player {
            100 - depth
        } else {
            -100 + depth
        };
    }

    if board.is_draw() {
        return 0;
    }

    if depth >= max_depth {
        return 0;
    }

    if is_maximizing {
        // This boolean tells the function:
        // - ✅ computer's turn (try to get a high score)
        // -    human's turn (try to get a low score).

        let mut best_score = i32::MIN;
        for (r, c) in board.get_legal_moves() {
            board.make_move(r, c);
            let score = minimax_optimized(
                board,
                depth + 1,
                max_depth,
                alpha,
                beta,
                false,
                computer_player,
            );
            board.undo_move(r, c);
            best_score = best_score.max(score);
            alpha = alpha.max(score);

            // The prune
            if beta <= alpha {
                break;
            }
        }
        best_score
    }
    // minimize (human's turn)
    else {
        let mut best_score = i32::MAX;
        for (r, c) in board.get_legal_moves() {
            board.make_move(r, c);
            let score = minimax_optimized(
                board,
                depth + 1,
                max_depth,
                alpha,
                beta,
                true,
                computer_player,
            );
            board.undo_move(r, c);
            best_score = best_score.min(score);
            beta = beta.min(best_score);

            // The prune
            if beta <= alpha {
                break;
            }
        }
        best_score
    }
}

#[derive(Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub fn get_best_move(board: &mut Board, level: Difficulty) -> Option<(usize, usize)> {
    let max_depth = match level {
        Difficulty::Easy => 1,
        Difficulty::Medium => 3,
        Difficulty::Hard => i32::MAX,
    };

    let computer_player = board.current_player;

    let mut best_score = i32::MIN;
    let mut best_move = None;

    for (r, c) in board.get_legal_moves() {
        board.make_move(r, c);
        let score = minimax_optimized(
            board,
            0,
            max_depth,
            i32::MIN,
            i32::MAX,
            false,
            computer_player,
        );
        board.undo_move(r, c);

        if score > best_score {
            best_score = score;
            best_move = Some((r, c));
        }
    }

    best_move
}
