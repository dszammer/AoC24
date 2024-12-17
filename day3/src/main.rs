fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input.txt")));
}

fn part1(input: Vec<String>) -> u64 {
    let mut sum = 0;

    input.iter().for_each(|line: &String| {
        sum += solve_multipikations(line);
    });

    sum
}

#[allow(unused_variables)]
fn part2(input: Vec<String>) -> u64 {
    let mut sum = 0;
    let input_joined = input.join("");

    let temp_split = input_joined.split("don't()").collect::<Vec<&str>>();

    temp_split.iter().enumerate().for_each(|(s, m)| {
        if s == 0 {
            sum += solve_multipikations(m);
        } else {
            let temp = temp_split[s].split("do()").collect::<Vec<&str>>();
            temp.iter().skip(1).for_each(|t| {
                sum += solve_multipikations(t);
            });
        }
    });

    sum
}

fn solve_multipikations(s: &str) -> u64 {
    let mut sum = 0;

    let vec = s.split("mul(").collect::<Vec<&str>>();
    vec.iter().for_each(|s| {
        let temp = s.split(")").collect::<Vec<&str>>();
        if temp.len() > 1 {
            let nums = temp[0].split(",").collect::<Vec<&str>>();
            let mut result = 1;
            nums.iter().for_each(|n| {
                if let Ok(n) = n.parse::<u64>() {
                    result *= n;
                } else {
                    result = 0;
                }
            });
            sum += result;
        }
    });

    sum
}

#[cfg(test)]
mod tests {

    fn get_test_input() -> Vec<String> {
        vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()]
    }

    fn get_test_input2() -> Vec<String> {
        vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_test_input()), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input2()), 48);
    }
}
