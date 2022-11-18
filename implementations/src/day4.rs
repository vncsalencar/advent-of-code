static INPUT: &str = "iwrupvqb";

fn calculate(leading: usize, starting: u128) -> u128 {
    fn hash(input: String) -> String {
        return format!("{:x}", md5::compute(input));
    }

    let prefix = "0".repeat(leading).to_string();

    let mut n: u128 = starting;
    loop {
        if hash(format!("{INPUT}{n}")).starts_with(&prefix) {
            break;
        };
        n += 1;
    }

    n
}

// set `starting` to speed up computing for when testing locally
fn part1() -> u128 {
    calculate(5, 346385)
}

fn part2() -> u128 {
    calculate(6, 9958217)
}

#[cfg(test)]
mod the_ideal_stocking_stuffer {

    #[test]
    fn part1() {
        let answer = super::part1();
        assert!(answer == 346386);
    }

    #[test]
    fn part2() {
        let answer = super::part2();
        assert!(answer == 9958218);
    }
}
