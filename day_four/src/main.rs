use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static FILENAME: &str = "input.txt";

type Board<T> = [[T; 5]; 5];

fn main() {
part_one();
    // part_two();
}

fn part_one() {
    let mut sequence: Vec<u8> = Vec::new();
    let mut fastest_solve_factor: i32 = i32::MAX;
    let mut best_board_index: i32 = 0;

    if let Ok(lines) = read_lines(FILENAME) {
        let mut lines_iter = lines.enumerate();

        if let Some((_, Ok(first_line))) = lines_iter.next() {
            let mut first_line_numbers: Vec<u8> = first_line
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();

            sequence.append(&mut first_line_numbers);
        }

        let mut board_index: i32 = 0;

        while let Some((line_index, _)) = lines_iter.next() {
            let mut numberboard = Board::default();

            for i in 0..5 {
                if let Some((_, Ok(line))) = lines_iter.next() {
                    let line_numbers: Vec<u8> = line
                        .split_whitespace()
                        .map(|num| {
                            num.parse()
                                .expect(format!("bad number at line {}", line_index + 1).as_str())
                        })
                        .collect();

                    for (cell_index, cell) in numberboard[i].iter_mut().enumerate() {
                        *cell = line_numbers[cell_index];
                    }
                }
            }

            let solve_factor = solve_board(&numberboard, &sequence);

            if solve_factor < fastest_solve_factor {
                fastest_solve_factor = solve_factor;
                best_board_index = board_index;
            }

            board_index += 1;
        }

        println!("{}", best_board_index);
    }
}

fn solve_board(board: &Board<u8>, sequence: &Vec<u8>) -> i32 {
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
