fn main() {
    let input = common::read_lines("./input.txt");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

fn part1(input: Vec<String>) -> u64 {
    let mut result = 0;
    let rules = get_rules(&input);
    let start = input.iter().position(|x| x == "").unwrap() + 1;

    input.iter().skip(start).for_each(|line| {
        if is_correct(
            &line
                .split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
            &rules,
        ) {
            result += get_middle_number(line);
        }
    });

    result
}

#[allow(unused_variables)]
fn part2(input: Vec<String>) -> u64 {
    let mut result = 0 as u64;
    let rules = get_rules(&input);
    let start = input.iter().position(|x| x == "").unwrap() + 1;

    input.iter().skip(start).for_each(|line| {
        if !is_correct(
            &line
                .split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
            &rules,
        ) {
            let corrected = correct_line(
                &line
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
                &rules,
            );

            result += corrected[corrected.len() / 2 as usize];
        }
    });

    result
}

fn correct_line(pages: &Vec<u64>, rules: &Vec<(u64, u64)>) -> Vec<u64> {
    let mut corrected = pages.clone();

    while !is_correct(&corrected, rules) {
        do_correction(&mut corrected, rules);
    }
    corrected
}

fn do_correction(corrected: &mut Vec<u64>, rules: &Vec<(u64, u64)>) {
    for (i, page) in corrected.iter().enumerate() {
        for j in 0..i {
            if rules.contains(&(*page, corrected[j])) {
                corrected.swap(i, j);
                return;
            }
        }
        for j in i..corrected.len() {
            if rules.contains(&(corrected[j], *page)) {
                corrected.swap(i, j);
                return;
            }
        }
    }
}

fn is_correct(pages: &Vec<u64>, rules: &Vec<(u64, u64)>) -> bool {
    for (i, page) in pages.iter().enumerate() {
        for j in 0..i {
            if rules.contains(&(*page, pages[j])) {
                return false;
            }
        }
        for j in i..pages.len() {
            if rules.contains(&(pages[j], *page)) {
                return false;
            }
        }
    }
    return true;
}

fn get_rules(input: &Vec<String>) -> Vec<(u64, u64)> {
    let mut rules = Vec::new();
    for line in input {
        if line == "" {
            break;
        }

        let mut parts = line.split("|");
        let first = parts.next().unwrap().parse::<u64>().unwrap();
        let second = parts.next().unwrap().parse::<u64>().unwrap();
        rules.push((first, second));
    }
    rules
}

fn get_middle_number(input: &str) -> u64 {
    let numbers: Vec<u64> = input
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    numbers[numbers.len() / 2]
}

#[cfg(test)]
mod tests {

    fn get_test_input() -> Vec<String> {
        vec![
            "47|53".to_string(),
            "97|13".to_string(),
            "97|61".to_string(),
            "97|47".to_string(),
            "75|29".to_string(),
            "61|13".to_string(),
            "75|53".to_string(),
            "29|13".to_string(),
            "97|29".to_string(),
            "53|29".to_string(),
            "61|53".to_string(),
            "97|53".to_string(),
            "61|29".to_string(),
            "47|13".to_string(),
            "75|47".to_string(),
            "97|75".to_string(),
            "47|61".to_string(),
            "75|61".to_string(),
            "47|29".to_string(),
            "75|13".to_string(),
            "53|13".to_string(),
            "".to_string(),
            "75,47,61,53,29".to_string(),
            "97,61,53,29,13".to_string(),
            "75,29,13".to_string(),
            "75,97,47,61,53".to_string(),
            "61,13,29".to_string(),
            "97,13,75,29,47".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_test_input()), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input()), 123);
    }

    #[test]
    fn test_is_correct() {
        let rules = super::get_rules(&get_test_input());
        assert_eq!(
            super::is_correct(
                &"75,47,61,53,29"
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
                &rules
            ),
            true
        );
        assert_eq!(
            super::is_correct(
                &"97,61,53,29,13"
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
                &rules
            ),
            true
        );
        assert_eq!(
            super::is_correct(
                &"75,29,13"
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
                &rules
            ),
            true
        );
        assert_eq!(
            super::is_correct(
                &"75,97,47,61,53"
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
                &rules
            ),
            false
        );
        assert_eq!(
            super::is_correct(
                &"61,13,29"
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
                &rules
            ),
            false
        );
        assert_eq!(
            super::is_correct(
                &"97,13,75,29,47"
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
                &rules
            ),
            false
        );
    }

    #[test]
    fn test_get_rules() {
        let input = vec!["47|53".to_string(), "97|13".to_string()];
        let rules = super::get_rules(&input);
        assert_eq!(rules, vec![(47, 53), (97, 13)]);
    }

    #[test]
    fn test_get_middle_number() {
        assert_eq!(super::get_middle_number("1,2,3,4,5"), 3);
        assert_eq!(super::get_middle_number("1,2,3,4,5,6"), 4);
    }

    #[test]
    fn test_correct_line() {
        let rules = super::get_rules(&get_test_input());

        assert_eq!(
            super::correct_line(
                &"75,97,47,61,53"
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
                &rules
            ),
            "97,75,47,61,53"
                .split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        );

        assert_eq!(
            super::correct_line(
                &"61,13,29"
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
                &rules
            ),
            "61,29,13"
                .split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        );

        assert_eq!(
            super::correct_line(
                &"97,13,75,29,47"
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
                &rules
            ),
            "97,75,47,29,13"
                .split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        );
    }
}
