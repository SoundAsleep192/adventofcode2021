use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut count = 0;
    if let Ok(lines) = read_lines("input.txt") {
        let mut lines_iter = lines.peekable();
        while let (Some(Ok(line)), Some(Ok(next_line))) = (lines_iter.next(), lines_iter.peek()) {
            if let (Ok(num), Ok(next_num)) = (line.parse::<i32>(), next_line.parse::<i32>()) {
                if next_num > num {
                    count += 1;
                }
            }
        }

        println!("{}", count);
    } else {
        println!("Something went wrong");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
