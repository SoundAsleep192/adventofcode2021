use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::constants::FILENAME;
use crate::types::Board;

pub fn get_sequence() -> Vec<u8> {
    let mut sequence: Vec<u8> = Vec::new();

    if let Ok(mut lines) = read_lines(FILENAME) {
        if let Some(Ok(first_line)) = lines.next() {
            let mut first_line_numbers: Vec<u8> = first_line
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();

            sequence.append(&mut first_line_numbers);
        }
    }

    return sequence;
}

pub fn iterate_over_boards<F>(mut callback: F)
where
    F: FnMut(usize, Board<u8>),
{
    if let Ok(lines) = read_lines(FILENAME) {
        let mut lines_iter = lines.enumerate().skip(1);
        let mut board_index: usize = 0;

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

            callback(board_index, numberboard);

            board_index += 1;
        }
    }
}

pub fn get_nth_board(nth: usize) -> Board<u8> {
    let mut numberboard = Board::default();

    if let Ok(lines) = read_lines(FILENAME) {
        let mut lines_iter = lines.enumerate().skip(1 + (nth * 6));

        if let Some((line_index, _)) = lines_iter.next() {
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
        }
    }

    return numberboard;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
