use crate::types::Board;

pub fn solve_board(board: &Board<u8>, sequence: &Vec<u8>) -> i32 {
    let mut checkboard = Board::default();

    for (number_of_attempts, number) in sequence.iter().enumerate() {
        for (line_index, line) in board.iter().enumerate() {
            for (column_index, cell) in line.iter().enumerate() {
                if *cell == *number {
                    checkboard[line_index][column_index] = true;

                    if check_line(line_index, &checkboard)
                        || check_column(column_index, &checkboard)
                    {
                        return (number_of_attempts + 1) as i32;
                    }
                }
            }
        }
    }

    return -1;
}

pub fn calculate_score(winning_board: &Board<u8>, sequence: &Vec<u8>) -> u32 {
    let mut checkboard = Board::default();
    let mut number_that_won: u32 = 0;

    'outer: for number in sequence.iter() {
        for (line_index, line) in winning_board.iter().enumerate() {
            for (column_index, cell) in line.iter().enumerate() {
                if *cell == *number {
                    checkboard[line_index][column_index] = true;

                    if check_line(line_index, &checkboard)
                        || check_column(column_index, &checkboard)
                    {
                        number_that_won = *number as u32;
                        break 'outer;
                    }
                }
            }
        }
    }

    let mut unmarked_numbers_sum: u32 = 0;

    for (line_index, line) in winning_board.iter().enumerate() {
        for (column_index, cell) in line.iter().enumerate() {
            if checkboard[line_index][column_index] == false {
                unmarked_numbers_sum += *cell as u32;
            }
        }
    }

    return unmarked_numbers_sum * number_that_won;
}

fn check_line(line_index: usize, checkboard: &Board<bool>) -> bool {
    for cell in checkboard[line_index].iter() {
        if *cell == false {
            return false;
        }
    }
    return true;
}

fn check_column(column_index: usize, checkboard: &Board<bool>) -> bool {
    for line in checkboard {
        if line[column_index] == false {
            return false;
        }
    }
    return true;
}
