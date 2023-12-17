use std::collections::HashMap;

type Coordinate = (i64,i64);
type InputType = HashMap<Coordinate,i64>;
type OutputType = u64;

#[aoc_generator(day17)]
fn day17_parse(input: &str) -> InputType {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| {
            ((x as i64,y as i64),c.to_digit(10).unwrap() as i64)
        })
    }).collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(day17, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
    }

    #[test]
    fn day17_part1() {
        assert_eq!(part1(&day17_parse(get_test_input())), 0);
    }

    #[test]
    fn day17_part2() {
        assert_eq!(part2(&day17_parse(get_test_input())), 0);
    }
}
