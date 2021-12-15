mod utils;
use crate::utils::board_processing::{calculate_score, solve_board};
use crate::utils::file_processing::{get_nth_board, get_sequence, iterate_over_boards};

mod constants;
mod types;

fn main() {
    part_one();
    // part_two();
}

fn part_one() {
    let sequence: Vec<u8> = get_sequence();
    let mut fastest_solve_factor: i32 = i32::MAX;
    let mut best_board_index: usize = 0;

    iterate_over_boards(|board_index, numberboard| {
        let solve_factor = solve_board(&numberboard, &sequence);

        if solve_factor < fastest_solve_factor {
            fastest_solve_factor = solve_factor;
            best_board_index = board_index;
        }
    });

    let winning_board = get_nth_board(best_board_index);
    let score = calculate_score(&winning_board, &sequence);
    println!("Score = {}", score);
}
