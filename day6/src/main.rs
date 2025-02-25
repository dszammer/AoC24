fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input.txt")));
}

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Debug, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Debug, Clone)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn move_forward(&mut self) {
        self.position = self.next_position();
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn next_position(&self) -> Position {
        match self.direction {
            Direction::Up => Position {
                x: self.position.x,
                y: self.position.y - 1,
            },
            Direction::Right => Position {
                x: self.position.x + 1,
                y: self.position.y,
            },
            Direction::Down => Position {
                x: self.position.x,
                y: self.position.y + 1,
            },
            Direction::Left => Position {
                x: self.position.x - 1,
                y: self.position.y,
            },
        }
    }
}

fn get_obstacles(input: &Vec<String>) -> Vec<Position> {
    let mut obstacles = Vec::<Position>::new();

    input.iter().enumerate().for_each(|(y, line)| {
        line.match_indices('#')
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|occurance| {
                obstacles.push(Position {
                    x: occurance.0 as i64,
                    y: y as i64,
                });
            });
    });

    obstacles
}

fn get_guard(input: &Vec<String>) -> Guard {
    let mut guard_position = Position { x: 0, y: 0 };

    input.iter().enumerate().for_each(|(y, line)| {
        if line.contains("^") {
            guard_position = Position {
                x: line.find('^').unwrap() as i64,
                y: y as i64,
            };
        }
    });

    Guard {
        position: guard_position,
        direction: Direction::Up,
    }
}

fn part1(input: Vec<String>) -> u64 {
    let obstacles = get_obstacles(&input);

    let mut visited = Vec::<Position>::new();

    let map_bounds = Position {
        x: input[0].len() as i64,
        y: input.len() as i64,
    };

    let mut guard = get_guard(&input);
    visited.push(guard.position.clone()); // Add the starting position to the visited list

    while guard.position.x >= 0
        && guard.position.x < map_bounds.x
        && guard.position.y >= 0
        && guard.position.y < map_bounds.y
    {
        if !visited.contains(&guard.position) {
            visited.push(guard.position.clone());
        }
        if !obstacles.contains(&guard.next_position()) {
            guard.move_forward();
        } else {
            guard.turn_right();
        }
    }
    visited.len() as u64
}

#[allow(unused_variables)]
fn part2(input: Vec<String>) -> u64 {
    let obstacles = get_obstacles(&input);

    let map_bounds = Position {
        x: input[0].len() as i64,
        y: input.len() as i64,
    };

    let mut guard = get_guard(&input);

    let mut visited: Vec<Position> = Vec::<Position>::new();

    visited.push(guard.position.clone()); // Add the starting position to the visited list

    while guard.position.x >= 0
        && guard.position.x < map_bounds.x
        && guard.position.y >= 0
        && guard.position.y < map_bounds.y
    {
        if !visited.contains(&guard.position) {
                visited.push(guard.position.clone());
            }

        if !obstacles.contains(&guard.next_position()) {
            guard.move_forward();
        } else {
            guard.turn_right();
        }
    }

    let guard = get_guard(&input);

    let mut loop_ops = Vec::<Position>::new();

    visited.iter().for_each(|position: &Position| {
        let mut this_guard = guard.clone();
        let mut this_obstacles = obstacles.clone();
        let mut loop_visited = Vec::<Guard>::new();

        loop_visited.push(this_guard.clone());

        this_obstacles.push(position.clone());

        while this_guard.position.x >= 0
            && this_guard.position.x < map_bounds.x
            && this_guard.position.y >= 0
            && this_guard.position.y < map_bounds.y
        {
            if !this_obstacles.contains(&this_guard.next_position()) {
                this_guard.move_forward();
            } else {
                this_guard.turn_right();
            }

            if !loop_visited.contains(&this_guard) {
                loop_visited.push(this_guard.clone());
            } else {
                if !loop_ops.contains(&position) {
                    loop_ops.push(position.clone());
                }
                break;
            }
        }
    });

    loop_ops.len() as u64
}

#[cfg(test)]
mod tests {

    fn get_test_input() -> Vec<String> {
        vec![
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_test_input()), 41);
    }

    #[test]
    fn test_guard_move_forward() {
        let mut guard = super::Guard {
            position: super::Position { x: 0, y: 0 },
            direction: super::Direction::Up,
        };

        guard.move_forward();
        assert_eq!(guard.position.x, 0);
        assert_eq!(guard.position.y, -1);

        guard.direction = super::Direction::Right;
        guard.move_forward();
        assert_eq!(guard.position.x, 1);
        assert_eq!(guard.position.y, -1);

        guard.direction = super::Direction::Down;
        guard.move_forward();
        assert_eq!(guard.position.x, 1);
        assert_eq!(guard.position.y, 0);

        guard.direction = super::Direction::Left;
        guard.move_forward();
        assert_eq!(guard.position.x, 0);
        assert_eq!(guard.position.y, 0);
    }

    #[test]
    fn test_guard_turn_right() {
        let mut guard = super::Guard {
            position: super::Position { x: 0, y: 0 },
            direction: super::Direction::Up,
        };

        guard.turn_right();
        assert_eq!(guard.direction, super::Direction::Right);

        guard.turn_right();
        assert_eq!(guard.direction, super::Direction::Down);

        guard.turn_right();
        assert_eq!(guard.direction, super::Direction::Left);

        guard.turn_right();
        assert_eq!(guard.direction, super::Direction::Up);
    }

    #[test]
    fn test_guard_next_position() {
        let guard = super::Guard {
            position: super::Position { x: 0, y: 0 },
            direction: super::Direction::Up,
        };

        let next_position = guard.next_position();
        assert_eq!(next_position.x, 0);
        assert_eq!(next_position.y, -1);

        let guard = super::Guard {
            position: super::Position { x: 0, y: 0 },
            direction: super::Direction::Right,
        };

        let next_position = guard.next_position();
        assert_eq!(next_position.x, 1);
        assert_eq!(next_position.y, 0);

        let guard = super::Guard {
            position: super::Position { x: 0, y: 0 },
            direction: super::Direction::Down,
        };

        let next_position = guard.next_position();
        assert_eq!(next_position.x, 0);
        assert_eq!(next_position.y, 1);

        let guard = super::Guard {
            position: super::Position { x: 0, y: 0 },
            direction: super::Direction::Left,
        };

        let next_position = guard.next_position();
        assert_eq!(next_position.x, -1);
        assert_eq!(next_position.y, 0);
    }

    #[test]
    fn test_get_obstacles() {
        let input = vec!["....#.....".to_string(), ".........#".to_string()];

        let obstacles = super::get_obstacles(&input);

        assert_eq!(obstacles.len(), 2);
        assert_eq!(obstacles.contains(&super::Position { x: 4, y: 0 }), true);
        assert_eq!(obstacles.contains(&super::Position { x: 9, y: 1 }), true);

        let input = vec!["########".to_string(), "########".to_string()];

        let obstacles = super::get_obstacles(&input);

        assert_eq!(obstacles.len(), 16);
        assert_eq!(obstacles.contains(&super::Position { x: 0, y: 0 }), true);
        assert_eq!(obstacles.contains(&super::Position { x: 1, y: 0 }), true);
    }

    #[test]
    fn test_get_guard() {
        let input = get_test_input();

        let guard = super::get_guard(&input);

        assert_eq!(guard.position.x, 4);
        assert_eq!(guard.position.y, 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input()), 6);
    }
}
