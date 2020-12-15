use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Mask {
    pub on_mask: u64,
    pub off_mask: u64
}

impl Mask {
    pub fn apply(self, other: u64) -> u64 {
        (other | self.on_mask) & !self.off_mask
    }
    pub fn parse(mask: &str) -> Self {
        let mut on_mask = 0;
        let mut off_mask = 0;
        for &c in mask.as_bytes() {
            on_mask <<= 1;
            off_mask <<= 1;
            match c {
                b'X' => continue,
                b'0' => off_mask |= 1,
                b'1' => on_mask |= 1,
                _ => panic!()
            }
        }
        Mask {on_mask, off_mask}
    }
}

fn main() {
    let mut memory = HashMap::new();
    let mut mask = Mask {on_mask: 0, off_mask: 0};
    for line in include_str!("input.txt").lines() {
        if let Some(line) = line.strip_prefix("mask = ") {
            mask = Mask::parse(line)
        } else {
            let mut it = line.split(" = ");
            let first = it.next().unwrap();
            let second = it.next().unwrap();
            let value: u64 = second.parse().unwrap();
            let location: u64 = first.trim_start_matches("mem[").trim_end_matches("]").parse().unwrap();

            memory.insert(location, mask.apply(value));
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
}