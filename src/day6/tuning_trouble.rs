use std::collections::HashSet;

use crate::utils::read_bytes;

/// bad performance
fn check_distinct(chars: &[char]) -> bool {
    let set: HashSet<_> = chars.iter().collect();
    chars.len() == set.len()
}

pub fn num_processed() -> i32 {
    let mut vector: Vec<char> = Vec::with_capacity(4);

    if let Ok(bytes) = read_bytes("src/day6/input.txt") {
        for (i, b) in bytes.into_iter().flatten().enumerate() {
            let ch = b as char;
            if vector.len() < 4 {
                vector.push(ch);
            } else if check_distinct(&vector) {
                return i as i32;
            } else {
                vector.remove(0);
                vector.push(ch);
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result() {
        assert_eq!(num_processed(), 1804);
    }
}
