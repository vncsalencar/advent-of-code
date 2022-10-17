fn part1() -> u32 {
    let input = super::get_input("day2");

    input
        .lines()
        .map(|line| {
            let dimensions = line
                .split('x')
                .map(|dimension| {
                    dimension
                        .parse::<u32>()
                        .expect(format!("Error parsing dimension {dimension}").as_str())
                })
                .collect::<Vec<u32>>();

            let l = 2 * dimensions[0] * dimensions[1];
            let w = 2 * dimensions[1] * dimensions[2];
            let h = 2 * dimensions[0] * dimensions[2];

            let wrapping_paper = l + w + h + ([l, w, h].iter().min().unwrap() / 2);
            wrapping_paper
        })
        .sum()
}

fn part2() -> u64 {
    let input = super::get_input("day2");

    input
        .lines()
        .map(|line| {
            let mut dimensions = line
                .split('x')
                .map(|dimension| {
                    dimension
                        .parse::<u64>()
                        .expect(format!("Error parsing dimension {dimension}").as_str())
                })
                .collect::<Vec<u64>>();
            dimensions.sort();

            let l = 2 * dimensions[0];
            let w = 2 * dimensions[1];
            let h = dimensions[0] * dimensions[1] * dimensions[2];

            let wrapping_paper = l + w + h;
            wrapping_paper
        })
        .sum()
}

#[cfg(test)]
mod i_was_told_there_would_be_no_math {

    #[test]
    fn part1() {
        let answer = super::part1();
        assert!(answer == 1598415);
    }

    #[test]
    fn part2() {
        let answer = super::part2();
        assert!(answer == 3812909);
    }
}
