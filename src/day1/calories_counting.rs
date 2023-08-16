use crate::utils;

/// [calories counting](https://adventofcode.com/2022/day/1)
pub fn count_calories() -> u32 {
    let mut most_calories = 0;
    let mut current_calories = 0;
    if let Ok(lines) = utils::read_lines("src/day1/input.txt") {
        for line in lines.into_iter().flatten() {
            if line.trim().is_empty() {
                most_calories = most_calories.max(current_calories);
                current_calories = 0;
            } else {
                current_calories += line.parse::<u32>().unwrap();
            }
        }
        most_calories = most_calories.max(current_calories);
    }
    most_calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result() {
        assert_eq!(count_calories(), 72478);
    }
}
