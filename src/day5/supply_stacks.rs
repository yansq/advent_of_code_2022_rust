use crate::utils::read_lines;

fn init(stacks: &mut [Vec<char>]) {
    if let Ok(lines) = read_lines("src/day5/inited_states.txt") {
        for (index, value) in lines.into_iter().flatten().enumerate() {
            for ch in value.chars() {
                stacks[index].push(ch)
            }
        }
    }
}

fn operate(stacks: &mut [Vec<char>], count: u32, from: usize, to: usize) {
    for _ in 0..count {
        let ch = stacks[from].pop().unwrap();
        stacks[to].push(ch);
    }
}

pub fn top_stacks() -> String {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    init(&mut stacks);

    if let Ok(lines) = read_lines("src/day5/operators.txt") {
        for line in lines.into_iter().flatten() {
            let l: Vec<&str> = line.splitn(6, ' ').collect();
            let (count, from, to) = match &l[..] {
                &[_, count, _, from, _, to] => (
                    count.parse::<u32>().unwrap(),
                    from.parse::<usize>().unwrap() - 1,
                    to.parse::<usize>().unwrap() - 1,
                ),
                _ => unreachable!(),
            };
            operate(&mut stacks, count, from, to);
        }
    }

    let mut res = String::new();
    for stack in stacks {
        res.push(*stack.last().unwrap());
    }
    println!("{res}");
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result() {
        assert_eq!(top_stacks(), "ZRLJGSCTR");
    }
}
