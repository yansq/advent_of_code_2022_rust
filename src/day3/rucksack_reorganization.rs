use std::collections::HashSet;

use crate::utils::read_lines;

fn priority(ch: char) -> u32 {
    match ch {
        'a'..='z' => ch as u32 - 96,
        'A'..='Z' => ch as u32 - 38,
        _ => 0,
    }
}

pub fn result() -> u32 {
    let mut priority_sum = 0;

    if let Ok(lines) = read_lines("src/day3/input.txt") {
        let mut set = HashSet::new();
        for line in lines.into_iter().flatten() {
            let length = line.len() / 2;
            for (index, value) in line.chars().enumerate() {
                if index < length {
                    set.insert(value);
                } else if set.contains(&value) {
                    priority_sum += priority(value);
                    break;
                }
            }
            set.clear();
        }
    }

    priority_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result() {
        assert_eq!(result(), 8176);
    }
}
