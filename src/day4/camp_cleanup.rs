use crate::utils::read_lines;

#[deny(clippy::if_same_then_else)]
pub fn count_pairs() -> u32 {
    let mut num_of_pairs = 0;
    if let Ok(lines) = read_lines("src/day4/input.txt") {
        for line in lines.into_iter().flatten() {
            let (first_range, second_range) = line.split_once(',').unwrap();

            let (first_head_point_, first_tail_point_) = first_range.split_once('-').unwrap();
            let (first_head_point, first_tail_point) = (
                first_head_point_.parse::<u32>().unwrap(),
                first_tail_point_.parse::<u32>().unwrap(),
            );

            let (second_head_point_, second_tail_point_) = second_range.split_once('-').unwrap();
            let (second_head_point, second_tail_point) = (
                second_head_point_.parse::<u32>().unwrap(),
                second_tail_point_.parse::<u32>().unwrap(),
            );

            if (first_head_point <= second_head_point && first_tail_point >= second_tail_point)
                || (second_head_point <= first_head_point && second_tail_point >= first_tail_point)
            {
                num_of_pairs += 1;
            }
        }
    }
    num_of_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result() {
        assert_eq!(count_pairs(), 532);
    }
}
