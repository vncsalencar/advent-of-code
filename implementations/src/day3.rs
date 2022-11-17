use std::collections::HashSet;

fn part1() -> usize {
    let input = super::get_input("day3");
    // left, down, up, right
    let mut directions = (0, 0);
    let mut visited = HashSet::<String>::new();

    input.trim().chars().for_each(|movement| {
        let key = format!("{}:{}", directions.0, directions.1);
        visited.insert(key);

        match movement {
            '<' => directions.0 -= 1,
            'v' => directions.1 -= 1,
            '^' => directions.1 += 1,
            '>' => directions.0 += 1,
            _ => unreachable!("This should not be called"),
        };
    });

    visited.len()
}

fn part2() -> usize {
    let input = super::get_input("day3");

    let mut santa_directions = (0, 0);
    let mut robot_directions = (0, 0);
    let mut visited = HashSet::<String>::new();

    let mut count = 0;

    input.trim().chars().for_each(|movement| {
        let mut directions = if count % 2 == 0 {
            &mut santa_directions
        } else {
            &mut robot_directions
        };

        let key = format!("{}:{}", directions.0, directions.1);
        visited.insert(key);

        match movement {
            '<' => (*directions).0 -= 1,
            'v' => (*directions).1 -= 1,
            '^' => (*directions).1 += 1,
            '>' => (*directions).0 += 1,
            _ => unreachable!("This should not be called"),
        };

        count += 1;
    });

    visited.len()
}

#[cfg(test)]
mod perfectly_spherical_houses_in_a_vacuum {

    #[test]
    fn part1() {
        let answer = super::part1();
        assert!(answer == 2572);
    }

    #[test]
    fn part2() {
        let answer = super::part2();
        assert!(answer == 2631);
    }
}
