
pub fn part1(input: &str) -> u32 {
    0
}

pub fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_part1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let result = part1(input);
        assert_eq!(result, 4);
    }

    //#[test]
    fn test_part2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let result = part2(input);
        assert_eq!(result, 4);
    }
}
