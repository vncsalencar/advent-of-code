fn part1() -> isize {
    let input = super::get_input("day1");

    input.chars().fold(0, |acc, cur| -> isize {
        if cur == '(' { return acc + 1 }
        acc - 1 
    })
}

fn part2() -> isize {
    let input = super::get_input("day1");
    let mut chars = input.chars();
    let mut floor = 0;
    let mut position = 0;

    loop {
        let next = chars.next();
        if next.is_none() { break };
        position += 1;

        floor += if next.unwrap() == '(' { 1 } else { -1 };

        if floor == -1 {
            break;
        }
    }

    position
}

#[cfg(test)]
mod not_quite_lisp {

    #[test]
    fn part1() {
        let answer = super::part1();
        assert!(answer == 74);
    }

    #[test]
    fn part2() {
        let answer = super::part2();
        assert!(answer == 1795);
    }

}
