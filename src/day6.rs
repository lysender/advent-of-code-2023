
use std::{collections::HashMap, ops::Range, ops::RangeBounds};
use indicatif::ProgressIterator;

pub fn part1(input: &str) -> u64 {
   todo!()
}

pub fn part2(input: &str) -> u64 {
   todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
      let input = "Time:      7  15   30
Distance:  9  40  200";

      // Test final output
      let result = part1(input);
      assert_eq!(result, 288);
    }

    #[test]
    fn test_part2() {
      let input = "Time:      7  15   30
Distance:  9  40  200";

      let result = part2(input);
      assert_eq!(result, 46);
    }
}
