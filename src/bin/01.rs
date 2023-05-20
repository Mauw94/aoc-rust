pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc_rust::read_file("inputs", 1);
    aoc_rust::solve!(1, part_one, input);
    aoc_rust::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc_rust::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc_rust::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
