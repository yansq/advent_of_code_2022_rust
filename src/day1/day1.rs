use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// [calories couting](https://adventofcode.com/2022/day/1)
fn main() {
    let mut most_calories = 0;
    let mut current_calories = 0;
    if let Ok(lines) = read_lines("src/day1/input.txt") {
        for line in lines.into_iter().flatten() {
            if line.trim().is_empty() {
                most_calories = most_calories.max(current_calories);
                current_calories = 0;
            } else {
                current_calories += line.parse::<u32>().unwrap();
            }
        }
        most_calories = most_calories.max(current_calories);
        println!("{}", most_calories);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
