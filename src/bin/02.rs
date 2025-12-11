use anyhow::{Context, Result};
use std::iter::FusedIterator;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for range in input.trim().split(',') {
        for num in IdIter::new_from_str(range)
            .with_context(|| format!("Invalid input: {range}"))
            .unwrap()
        {
            sum += num;
        }
    }
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[derive(Debug)]
struct IdIter {
    end: u64,
    cur: u64,
    mult: u64,
}

impl IdIter {
    fn new_from_str(input: &str) -> Result<Self> {
        if let Some((start, end)) = input.split_once('-') {
            Ok(IdIter::new(start.len(), start.parse()?, end.parse()?))
        } else {
            Err(anyhow::anyhow!("not a range string: {input}"))
        }
    }

    fn new(mut start_len: usize, start: u64, end: u64) -> Self {
        if !start_len.is_multiple_of(2) {
            start_len += 1;
        }
        start_len /= 2;
        let cur = 10u64.pow((start_len - 1) as u32);
        let mult = cur * 10;

        let mut it = IdIter { end, cur, mult };

        // println!("START {it:?}");

        loop {
            let cur = it.cur;
            let cmul = it.mult;
            if let Some(next) = it.next() {
                if next >= start {
                    it.cur = cur;
                    it.mult = cmul;
                    // println!("END ${it:?}");
                    return it;
                }
            } else {
                // println!("{start} not found in {start}-{end}");
                return it;
            }
        }
    }

    fn make_num(&self) -> u64 {
        self.cur * self.mult + self.cur
    }
}

/// Iterator that converts the current number into an "invalid id" from the puzzle.
/// Once the end is reached, the iterator returns None.
/// We use the mult to calculate the "output" number which will essentially "duplicate" the number.
/// So like 100 -> 100100.  Once the next_mult is reached, the mult we need to increment the mult
/// to the next 10x multiplier.  Because if we start with 100, when we get to 1000, we need to
/// change the multiplier to 1000 so when we're at 1000 the output number will be 10001000.  We're
/// going to use next_mult to just signal when we need to change the multiplier
impl Iterator for IdIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.make_num();
        // dbg!(self.cur, cur);

        // println!("{cur}");

        if cur > self.end {
            // println!("end reached");
            return None;
        }

        self.cur += 1;
        if self.cur >= self.mult {
            self.mult *= 10;
            // dbg!(self.cur, self.mult);
        }

        Some(cur)
    }
}

impl FusedIterator for IdIter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        for (input, expected) in [
            (
                "11-1213",
                vec![11, 22, 33, 44, 55, 66, 77, 88, 99, 1010, 1111, 1212],
            ),
            ("11-22", vec![11, 22]),
            ("95-115", vec![99]),
            ("998-1012", vec![1010]),
            ("1188511880-1188511890", vec![1188511885]),
            ("222220-222224", vec![222222]),
            ("1698522-1698528", vec![]),
            ("446443-446449", vec![446446]),
            ("38593856-38593862", vec![38593859]),
            ("262248430-262271846", vec![]),
        ] {
            assert_eq!(
                IdIter::new_from_str(input).unwrap().collect::<Vec<u64>>(),
                expected
            );
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
