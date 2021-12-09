use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static FILENAME: &str = "input.txt";

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut common_bits = [0; 12];

    if let Ok(lines) = read_lines(FILENAME) {
        let mut lines_iter = lines;
        while let Some(Ok(line)) = lines_iter.next() {
            for (index, character) in line.chars().enumerate() {
                match character {
                    '0' => common_bits[index] -= 1,
                    '1' => common_bits[index] += 1,
                    _ => (),
                }
            }
        }
    } else {
        panic!("Something went wrong");
    }

    let gamma_rate_str = common_bits
        .map(|bit| if bit > 0 { '1' } else { '0' })
        .iter()
        .collect::<String>();

    let epsilon_rate_str = common_bits
        .map(|bit| if bit > 0 { '0' } else { '1' })
        .iter()
        .collect::<String>();

    let gamma_rate = parse_from_binary(gamma_rate_str);
    let epsilon_rate = parse_from_binary(epsilon_rate_str);
    println!("{}", gamma_rate * epsilon_rate);
}

fn part_two() {
    let mut all: Vec<String> = Vec::new();


    if let Ok(lines) = read_lines(FILENAME) {
        for (line_number, line) in lines.enumerate() {
            if let Ok(binary) = line {
                if !(binary.len() == 12) {
                    panic!("Bad input length at line {}", line_number + 1);
                }
                all.push(binary);
            } else {
                panic!("Bad input at line {}", line_number + 1);
            }
        }
    } else {
        panic!("Could not read the file");
    }

    let oxy_rating = find_rating(&all, true);
    let co2_rating =  find_rating(&all, false);

    println!("{}", oxy_rating * co2_rating);
  
}

fn find_rating(all: &Vec<String>, switch: bool) -> i32 {
    let mut zeros: Vec<usize> = Vec::new();
    let mut ones: Vec<usize> = Vec::new();
    let mut chosen: Vec<usize> = (0..all.len()).collect();

    for char_index in 0..12 {
        for num_index in chosen.iter() {
            let num = &all[num_index.clone()];
            match num.chars().nth(char_index).unwrap() {
                '0' => zeros.push(num_index.clone()),
                '1' => ones.push(num_index.clone()),
                _ => panic!("bad char of binary number"),
            }
        }

        chosen.clear();

        if (zeros.len() > ones.len()) == switch {
            ones.clear();
            chosen.append(&mut zeros);
        } else {
            zeros.clear();
            chosen.append(&mut ones);
        }

        if chosen.len() == 1 {
            break;
        }
    }

    return parse_from_binary(all[chosen[0]].clone());
}

fn parse_from_binary(binary: String) -> i32 {
    return i32::from_str_radix(&binary, 2).unwrap_or(-1);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
