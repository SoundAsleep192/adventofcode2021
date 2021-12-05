use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("part one answer: {}", part_one());
    println!("part two answer: {}", part_two());
}

fn part_one() -> i32 {
    let mut position = 0;
    let mut depth = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        let mut lines_iter = lines;
        while let Some(Ok(line)) = lines_iter.next() {
            let line_words: Vec<&str> = line.split_whitespace().collect();
            if let (direction, Ok(amount)) = (line_words[0], line_words[1].parse::<i32>()) {
                match direction {
                    "forward" => position += amount,
                    "down" => depth += amount,
                    "up" => depth -= amount,
                    _ => panic!("Invalid direction"),
                }
            }
        }
    } else {
        panic!("Something went wrong");
    }

    return position * depth;
}

fn part_two() -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        let mut lines_iter = lines;
        while let Some(Ok(line)) = lines_iter.next() {
            let line_words: Vec<&str> = line.split_whitespace().collect();
            if let (direction, Ok(amount)) = (line_words[0], line_words[1].parse::<i32>()) {
                match direction {
                    "forward" => {
                        position += amount;
                        depth += aim * amount
                    }
                    "down" => aim += amount,
                    "up" => aim -= amount,
                    _ => panic!("Invalid direction"),
                }
            }
        }
    } else {
        panic!("Something went wrong");
    }

    return position * depth;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
