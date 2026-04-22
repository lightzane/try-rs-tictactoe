use std::io;
use try_rs_tictactoe::engine::{Difficulty, get_best_move};
use try_rs_tictactoe::{Board, Player};

fn main() {
    // 1. Initialize an empty 3x3 board where 'x' starts
    let mut board = Board::from_string("3/3/3 x");
    let ai_difficulty = Difficulty::Hard;

    println!("--- 🦀 Rust Tic-Tac-Toe vs AI ---");

    loop {
        board.print_visual();
        println!("\nCurrent Board: {}", board);

        if board.is_game_over() {
            break;
        }

        if board.current_player == Player::X {
            // --- HUMAN TURN ---
            println!("Your turn (x). Enter move as 'row col' (e.g., 0 0):");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let coords: Vec<usize> = input
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if coords.len() == 2 && coords[0] < board.rows && coords[1] < board.cols {
                board.make_move(coords[0], coords[1]);
            } else {
                println!("Invalid input! Try again.");
                continue;
            }
        } else {
            // --- AI TURN ---
            println!("AI is thinking...");
            if let Some((r, c)) = get_best_move(&mut board, ai_difficulty) {
                println!("AI (o) chooses: {} {}", r, c);
                board.make_move(r, c);
            }
        }
    }

    // --- GAME END ---
    println!("\nFinal Board: {}", board);
    match board.check_winner() {
        Some(Player::X) => println!("🎉 You win!"),
        Some(Player::O) => println!("🤖 AI wins!"),
        None => println!("🤝 It's a draw!"),
    }
}
