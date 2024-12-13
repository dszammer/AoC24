use std::vec;

fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input.txt")));
}

fn part1(input: Vec<String>) -> u64 {
    let mut vec_left: Vec<u64> = Vec::new();
    let mut vec_right: Vec<u64> = Vec::new();

    input.iter().for_each(|line: &String| {
        vec_left.push(
            line.split("   ").collect::<Vec<&str>>()[0]
                .parse::<u64>()
                .unwrap(),
        );
        vec_right.push(
            line.split("   ").collect::<Vec<&str>>()[1]
                .parse::<u64>()
                .unwrap(),
        );
    });

    vec_left.sort();
    vec_right.sort();

    let mut result: u64 = 0;

    vec_left.iter().zip(vec_right.iter()).for_each(|(l, r)| {
        result += r.abs_diff(*l);
    });

    result
}

#[allow(unused_variables)]
fn part2(input: Vec<String>) -> u64 {
    let mut vec_left: Vec<u64> = Vec::new();
    let mut vec_right: Vec<u64> = Vec::new();

    input.iter().for_each(|line: &String| {
        vec_left.push(
            line.split("   ").collect::<Vec<&str>>()[0]
                .parse::<u64>()
                .unwrap(),
        );
        vec_right.push(
            line.split("   ").collect::<Vec<&str>>()[1]
                .parse::<u64>()
                .unwrap(),
        );
    });

    let mut result = 0;

    vec_left.iter().for_each(|l| {
        let mult: u64 = vec_right
            .iter()
            .filter(|r| *r == l)
            .count()
            .try_into()
            .unwrap();

        result += l * mult;
    });

    result
}

#[cfg(test)]
mod tests {

    fn get_test_input() -> Vec<String> {
        vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_test_input()), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input()), 31);
    }
}
