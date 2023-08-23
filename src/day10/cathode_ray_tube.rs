use crate::utils::read_lines;

pub fn sum() -> i32 {
    let mut vec = vec![1; 230];
    let mut index = 1;

    if let Ok(lines) = read_lines("src/day10/input.txt") {
        for line in lines.into_iter().flatten() {
            if index > 221 {
                break;
            }
            if let Some((_, value)) = line.split_once(' ') {
                let value = value.parse::<i32>().unwrap();
                vec[index + 1] = vec[index];
                vec[index + 2] = vec[index] + value;
                index += 2;
            } else {
                vec[index + 1] = vec[index];
                index += 1;
            }
        }
    }

    if index < 221 {
        for i in index + 1..=221 {
            vec[i] = vec[i - 1];
        }
    }

    vec[20] * 20 + vec[60] * 60 + vec[100] * 100 + vec[140] * 140 + vec[180] * 180 + vec[220] * 220
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result() {
        assert_eq!(sum(), 13860);
    }
}
