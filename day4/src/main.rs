fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input.txt")));
}

fn part1(input: Vec<String>) -> u64 {
    let mut occurences = 0;

    // original lines

    input.iter().for_each(|line| {
        occurences += line.matches("XMAS").count() as u64;
    });

    input.iter().for_each(|line| {
        occurences += line.matches("SAMX").count() as u64;
    });

    // rotate 90 degrees

    let rotate: Vec<String> = rotate90(&input);

    rotate.iter().for_each(|line| {
        occurences += line.matches("XMAS").count() as u64;
    });

    rotate.iter().for_each(|line| {
        occurences += line.matches("SAMX").count() as u64;
    });

    //rotate 45 degrees

    let rotate: Vec<String> = rotate45(&input);

    rotate.iter().for_each(|line| {
        occurences += line.matches("XMAS").count() as u64;
    });

    rotate.iter().for_each(|line| {
        occurences += line.matches("SAMX").count() as u64;
    });

    //rotate 135 degrees

    let rotate135: Vec<String> = rotate45(&rotate90(&input));

    rotate135.iter().for_each(|line| {
        occurences += line.matches("XMAS").count() as u64;
    });

    rotate135.iter().for_each(|line| {
        occurences += line.matches("SAMX").count() as u64;
    });

    occurences
}

#[allow(unused_variables)]
fn part2(input: Vec<String>) -> u64 {
    0
}

fn rotate90(input: &[String]) -> Vec<String> {
    let mut rotate90: Vec<String> = Vec::new();

    for i in 0..input[0].len() {
        let mut new_line = String::new();
        for j in 0..input.len() {
            new_line.push(input[j].as_bytes()[input[0].len() - i - 1] as char);
        }
        rotate90.push(new_line);
    }

    rotate90
}

fn rotate45(input: &[String]) -> Vec<String> {
    let mut rotate45: Vec<String> = Vec::new();

    let n = input.len();
    let m = input[0].len();

    for c in 0..(2 * n - 1) {
        let mut new_line = String::new();

        for i in 0..m {
            for (j, _) in input.iter().enumerate() {
                if i + j == c {
                    new_line.push(input[j].as_bytes()[i] as char);
                }
            }
        }

        if !new_line.is_empty() {
            rotate45.push(new_line);
        }
    }

    rotate45
}

#[cfg(test)]
mod tests {

    fn get_test_input() -> Vec<String> {
        vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ]
    }

    fn get_test_input_rotated_90() -> Vec<String> {
        vec![
            "MAMXMASAMX".to_string(),
            "SSMMMMSAMS".to_string(),
            "AMASAAXAMA".to_string(),
            "MSAMXXSSMM".to_string(),
            "XMMSMXAAXX".to_string(),
            "XXXAAMSMMA".to_string(),
            "SMSMSMMAMX".to_string(),
            "MAXAAASXMM".to_string(),
            "MSMSMXMAAX".to_string(),
            "MMAMXXSSMM".to_string(),
        ]
    }

    fn get_test_input_rotated45() -> Vec<String> {
        vec![
            "M".to_string(),
            "MM".to_string(),
            "ASM".to_string(),
            "MMAS".to_string(),
            "XSXMX".to_string(),
            "XMASXX".to_string(),
            "SXAMXMM".to_string(),
            "SMASAMSA".to_string(),
            "MASMASAMS".to_string(),
            "MAXMMMMASM".to_string(),
            "XMASXXSMA".to_string(),
            "MMMAXAMM".to_string(),
            "XMASAMX".to_string(),
            "AXSXMM".to_string(),
            "XMASA".to_string(),
            "MMAS".to_string(),
            "AMA".to_string(),
            "SM".to_string(),
            "X".to_string(),
        ]
    }

    #[test]
    fn test_rotate_45() {
        let rotate45: Vec<String> = super::rotate45(&get_test_input());
        assert_eq!(rotate45, get_test_input_rotated45());
    }

    #[test]
    fn test_rotate_90() {
        let rotate90: Vec<String> = super::rotate90(&get_test_input());

        assert_eq!(rotate90, get_test_input_rotated_90());
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_test_input()), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input()), 0);
    }
}
