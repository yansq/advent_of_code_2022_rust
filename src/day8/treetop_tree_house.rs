use crate::utils::read_lines;

pub fn count_tree() -> i32 {
    let mut vec2 = vec![];
    let mut count = 0;

    if let Ok(lines) = read_lines("src/day8/input.txt") {
        for line in lines.into_iter().flatten() {
            let line: Vec<i32> = line.as_bytes().iter().map(|x| (x - 48) as i32).collect();
            vec2.push(line);
        }
    }

    let length = vec2.len();
    let mut dp_vec = vec![vec![(-1, -1, -1, -1); length]; length];
    dp(length, &vec2, &mut dp_vec);

    for i in 0..length {
        for j in 0..length {
            println!("{}", vec2[i][j]);
            println!("{:?}", dp_vec[i][j]);
            if vec2[i][j]
                > dp_vec[i][j]
                    .0
                    .min(dp_vec[i][j].1.min(dp_vec[i][j].2.min(dp_vec[i][j].3)))
            {
                count += 1;
            }
        }
    }

    count
}

fn dp(length: usize, vec: &Vec<Vec<i32>>, dp: &mut Vec<Vec<(i32, i32, i32, i32)>>) {
    // left
    for i in 0..length {
        for j in 1..length {
            dp[i][j].0 = dp[i][j - 1].0.max(vec[i][j - 1]);
        }
    }

    // right
    for i in 0..length {
        for j in (0..length - 1).rev() {
            dp[i][j].1 = dp[i][j + 1].1.max(vec[i][j + 1]);
        }
    }

    //top
    for i in 0..length {
        for j in 1..length {
            dp[j][i].2 = dp[j - 1][i].2.max(vec[j - 1][i]);
        }
    }

    //bottom
    for i in 0..length {
        for j in (0..length - 1).rev() {
            dp[j][i].3 = dp[j + 1][i].3.max(vec[j + 1][i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result() {
        assert_eq!(count_tree(), 532);
    }
}
