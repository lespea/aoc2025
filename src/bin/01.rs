use std::num::NonZero;

use anyhow::{Context, bail};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut pos = 50u16;
    let mut zeros = 0;

    for line in input.lines() {
        let res = Move::try_from(line).unwrap().spin(pos);
        pos = res.pos;
        if pos == 0 {
            zeros += 1;
        }
    }

    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pos = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let m = Move::try_from(line).unwrap();
        let res = m.spin(pos);
        pos = res.pos;
        zeros += res.zeros;
        // dbg!(m, res, zeros);
    }

    Some(zeros)
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

#[derive(Debug, Clone, Copy)]
struct SpinResults {
    pos: u16,
    zeros: u64,
}

impl Move {
    fn spin(&self, pos: u16) -> SpinResults {
        let steps = self.steps.get();

        let mut zeros = (steps / 100) as u64;
        let steps = steps % 100;

        let diff = match self.dir {
            Dir::Left => -(steps as i16),
            Dir::Right => steps as i16,
        };

        let final_pos = self.resolve((pos as i16) + diff, pos == 0, &mut zeros);

        SpinResults {
            pos: final_pos,
            zeros,
        }
    }

    #[inline]
    fn resolve(&self, pos: i16, start_zero: bool, zeros: &mut u64) -> u16 {
        if pos == 0 {
            if !start_zero {
                *zeros += 1;
            }
            0
        } else if pos > 0 {
            if pos >= 100 {
                *zeros += 1;
                (pos - 100) as u16
            } else {
                pos as u16
            }
        } else {
            if !start_zero {
                *zeros += 1;
            }

            let pos = if pos <= -100 {
                *zeros += 1;
                pos + 100
            } else {
                pos
            };

            if pos < 0 { (100 + pos) as u16 } else { 0 }
        }
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
        assert_eq!(result, Some(6));
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
                .spin(spin.2)
                .pos,
                spin.3
            );
        }
    }
}
