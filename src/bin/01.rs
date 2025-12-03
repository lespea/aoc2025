use std::num::NonZero;

use anyhow::{Context, bail};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut pos = 50;
    let mut zeros = 0;

    for line in input.lines() {
        pos = Move::try_from(line).unwrap().spin(pos);
        if pos == 0 {
            zeros += 1;
        }
    }

    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Move {
    dir: Dir,
    steps: NonZero<u16>,
}

impl Move {
    fn spin(&self, mut pos: i16) -> i16 {
        match self.dir {
            Dir::Left => pos -= self.steps.get() as i16,
            Dir::Right => pos += self.steps.get() as i16,
        }

        pos %= 100;
        if pos < 0 { 100 + pos } else { pos }
    }
}

impl TryFrom<&str> for Move {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir, steps) = value.split_at(1);

        let dir = match dir {
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => bail!("invalid direction: {}", dir),
        };

        let steps = steps
            .parse()
            .with_context(|| format!("invalid steps: {}", steps))?;

        Ok(Move { dir, steps })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_spin_wraps() {
        let spins = [
            (Dir::Left, 1, 0, 99),
            (Dir::Left, 101, 0, 99),
            (Dir::Right, 1, 99, 0),
            (Dir::Right, 101, 99, 0),
        ];

        for spin in spins {
            assert_eq!(
                Move {
                    dir: spin.0,
                    steps: NonZero::<u16>::new(spin.1).unwrap()
                }
                .spin(spin.2),
                spin.3
            );
        }
    }
}
