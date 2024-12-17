fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input.txt")));
}

fn part1(input: Vec<String>) -> u64 {
    let mut save = 0;

    input.iter().for_each(|line: &String| {
        let vec = line
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        if is_save(vec) {
            save += 1;
        }
    });

    save
}

#[allow(unused_variables)]
fn part2(input: Vec<String>) -> u64 {
    let mut save = 0;

    input.iter().for_each(|line: &String| {
        let vec = line
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        if is_save(vec.clone()) {
            save += 1;
        } else {
            println!("{:?}", vec);
            for i in 0..vec.len() {
                let mut temp = vec.clone();
                temp.remove(i);
                if is_save(temp) {
                    save += 1;
                    break;
                }
            }
        }
    });
    save
}

fn is_save(vec: Vec<u64>) -> bool {
    let mut sorted = vec.clone();
    sorted.sort();
    if sorted != vec {
        let mut sorted = vec.clone();
        sorted.sort_by(|a, b| b.cmp(a));
        if sorted != vec {
            return false;
        }
    }

    vec.iter()
        .rev()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| {
            let sub = w[0].abs_diff(*w[1]);
            !(1..=3).contains(&sub)
        })
        .collect::<Vec<_>>()
        .is_empty()
}

#[cfg(test)]
mod tests {

    fn get_test_input() -> Vec<String> {
        vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_test_input()), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input()), 4);
    }

    #[test]
    fn test_is_save() {
        assert_eq!(super::is_save(vec![7, 6, 4, 2, 1]), true);
        assert_eq!(super::is_save(vec![1, 2, 7, 8, 9]), false);
        assert_eq!(super::is_save(vec![9, 7, 6, 2, 1]), false);
        assert_eq!(super::is_save(vec![1, 3, 2, 4, 5]), false);
        assert_eq!(super::is_save(vec![8, 6, 4, 4, 1]), false);
        assert_eq!(super::is_save(vec![1, 3, 6, 7, 9]), true);
    }
}
