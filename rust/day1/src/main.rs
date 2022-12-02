fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("./input.txt");

    let mut calories = input
        .trim_end()
        .split("\n\n")
        .map(|grouped_calories| {
            grouped_calories
                .split("\n")
                .map(|calories| {
                    calories
                        .parse::<i32>()
                        .expect(format!("Could not parse {calories}").as_str())
                })
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    calories.sort();
    calories.reverse();

    println!("# Day 1: Calorie Counting");
    println!("Part 1 = {}", calories[0]);
    println!("Part 2 = {}", calories[0] + calories[1] + calories[2]);

    Ok(())
}
