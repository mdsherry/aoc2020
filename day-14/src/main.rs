use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Mask {
    pub on_mask: u64,
    pub off_mask: u64,
    pub float_mask: u64,
}

impl Mask {
    pub fn apply(self, other: u64) -> u64 {
        (other | self.on_mask) & !self.off_mask
    }
    pub fn values(self, other: u64) -> MaskIterator {
        let mut positions = vec![];
        for idx in 0..36 {
            if self.float_mask & (1 << idx) != 0 {
                positions.push(idx);
            }
        }
        MaskIterator {
            value: self.apply(other),
            idx: 0,
            positions,
        }
    }
    pub fn parse(mask: &str) -> Self {
        let mut on_mask = 0;
        let mut off_mask = 0;
        let mut float_mask = 0;
        for &c in mask.as_bytes() {
            on_mask <<= 1;
            off_mask <<= 1;
            float_mask <<= 1;
            match c {
                b'X' => float_mask |= 1,
                b'0' => continue,
                b'1' => on_mask |= 1,
                _ => panic!(),
            }
        }
        Mask {
            on_mask,
            off_mask,
            float_mask,
        }
    }
}

struct MaskIterator {
    value: u64,
    idx: u64,
    positions: Vec<u16>,
}

impl Iterator for MaskIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = self.value;
        if self.idx >= (1 << self.positions.len()) {
            None
        } else {
            for (i, idx) in self.positions.iter().copied().enumerate() {
                if self.idx & (1 << i) == 0 {
                    value &= !(1 << idx);
                } else {
                    value |= 1 << idx;
                }
            }
            self.idx += 1;
            Some(value)
        }
    }
}

fn main() {
    let mut memory = HashMap::new();
    let mut mask = Mask {
        on_mask: 0,
        off_mask: 0,
        float_mask: 0,
    };
    for line in include_str!("input.txt").lines() {
        if let Some(line) = line.strip_prefix("mask = ") {
            mask = Mask::parse(line)
        } else {
            let mut it = line.split(" = ");
            let first = it.next().unwrap();
            let second = it.next().unwrap();
            let value: u64 = second.parse().unwrap();
            let location: u64 = first
                .trim_start_matches("mem[")
                .trim_end_matches("]")
                .parse()
                .unwrap();

            for location in mask.values(location) {
                memory.insert(location, value);
            }
        }
    }
    let result: u64 = memory.values().sum();
    println!("{}", result)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mask() {
        // let mask = Mask { on_mask: 0b1000000, off_mask: 0b10 };
        let mask = Mask::parse("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(0b1001001, mask.apply(11));
    }

    #[test]
    fn test_multimask() {
        let mask = Mask::parse("00000000000000000000000000000000X0XX");
        let v: Vec<_> = mask.values(26).collect();
        assert_eq!(vec![16, 17, 18, 19, 24, 25, 26, 27], v);
        // let mask = Mask::parse("000000000000000000000000000000001011");
        // let v: Vec<_> = mask.values(26).collect();
        // assert_eq!(vec![16, 17, 18, 19, 24, 25, 26, 27], v);
    }
}
