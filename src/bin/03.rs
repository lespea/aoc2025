use std::num::{NonZero, NonZeroU8};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;

    for line in input.lines() {
        sum += sum_line(line)
    }

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[derive(Debug, Default)]
struct NumTracker {
    first: u8,
    second: Option<NonZeroU8>,
}

impl NumTracker {
    fn num(&self) -> Option<u64> {
        if let Some(sec) = self.second {
            let l = self.first as u64 * 10 + (sec.get() as u64);
            Some(l)
        } else {
            None
        }
    }

    fn update(&mut self, num: u8) -> Option<Self> {
        if let Some(sec) = self.second {
            if num > sec.get() {
                self.second = NonZero::new(num);
            }
        } else {
            self.second = NonZero::new(num);
        }

        if num > self.first {
            Some(Self {
                first: num,
                second: None,
            })
        } else {
            None
        }
    }
}

fn sum_line(line: &str) -> u64 {
    let mut last = NumTracker::default();

    let mut cur = NumTracker::default();

    for ascii_num in line.trim_ascii().bytes() {
        let num = ascii_num - b'0';
        if num > 9 {
            panic!("invalid input: {ascii_num}");
        }

        if let Some(new_cur) = cur.update(num) {
            last = cur;
            cur = new_cur;
        }
    }

    if cur.second.is_none() {
        last.num().unwrap()
    } else {
        cur.num().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
