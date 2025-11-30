use aoc_utils::read_input;

const BASE: &str = env!("CARGO_MANIFEST_DIR");
const DAY: &str = "d06";

fn main() {
    let input = read_input(BASE, DAY);
    println!("{DAY} part1: {}", part1(&input));
    println!("{DAY} part2: {}", part2(&input));
}

fn part1(_input: &[String]) -> i64 {
    0
}

fn part2(_input: &[String]) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let input = read_input(BASE, DAY);
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 0);
    }
}
