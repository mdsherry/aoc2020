use std::collections::VecDeque;

#[derive(Debug, Default)]
struct ScreenedVec {
    vec: VecDeque<u32>,
    mask: u128
}

trait ToMask {
    fn mask(self) -> u128;
}

impl ToMask for u32 {
    fn mask(self) -> u128 {
        1 << ((self >> 11) & 0x7f)
    }
}
impl ToMask for &u32 {
    fn mask(self) -> u128 {
        (*self).mask()
    }
}
struct FancyVec {
    vecs: VecDeque<ScreenedVec>
}


impl FancyVec {
    fn from(vec: Vec<u32>) -> Self {
        let mut vecs = VecDeque::new();
        let mut working_vec = VecDeque::new();
        let mut mask = 0;
        vec.into_iter().for_each(|value| {
            working_vec.push_back(value);
            mask |= value.mask();
            if working_vec.len() == 64 {
                let mut new_vec = VecDeque::new();
                std::mem::swap(&mut new_vec, &mut working_vec);
                vecs.push_back(ScreenedVec { vec: new_vec, mask });
            }
        });
        vecs.push_back(ScreenedVec { vec: working_vec, mask });
        FancyVec { vecs }
    }

    fn pop(&mut self) -> u32 {
        let a = self.vecs[0].vec.pop_front().unwrap();
        while self.vecs[0].vec.is_empty() {
            self.vecs.pop_front();
        }
        a
    }

    fn push(&mut self, value: u32) {
        if self.vecs.back().unwrap().vec.len() > 64 {
            self.vecs.push_back(Default::default());
        }
        let back_vec = self.vecs.back_mut().unwrap();
        back_vec.vec.push_back(value);
        back_vec.mask |= value.mask();
    }

    fn insert_after(&mut self, target: u32, three: [u32; 3], timer: &mut Timer) -> bool {
        let start = std::time::Instant::now();
        let target_mask = target.mask();
        for (i, vec) in self.vecs.iter_mut().enumerate() {
            if vec.mask & target_mask != 0 {
                if let Some(idx) = vec.vec.iter().position(|v| v == &target) {
                    timer.searching += start.elapsed().as_micros();
                    let [a, b, c] = three;
                    vec.vec.insert(idx, c);
                    vec.vec.insert(idx, b);
                    vec.vec.insert(idx, a);
                    vec.mask |= a.mask() | b.mask() | c.mask();
                    if vec.vec.len() >= 256 {
                        let start = std::time::Instant::now();
                        let fourth: VecDeque<_> = vec.vec.drain(192..).collect();
                        let third: VecDeque<_> = vec.vec.drain(128..).collect();
                        let second: VecDeque<_> = vec.vec.drain(64..).collect();
                        let first: VecDeque<_> = vec.vec.drain(0..).collect();
                        self.vecs.remove(i);
                        let fourth_mask = fourth.iter().fold(0, |mask, value| mask | value.mask());
                        let third_mask = third.iter().fold(0, |mask, value| mask | value.mask());
                        let second_mask = second.iter().fold(0, |mask, value| mask | value.mask());
                        let first_mask = first.iter().fold(0, |mask, value| mask | value.mask());

                        self.vecs.insert(i, ScreenedVec{ vec: fourth, mask: fourth_mask });
                        self.vecs.insert(i, ScreenedVec{ vec: third, mask: third_mask });
                        self.vecs.insert(i, ScreenedVec{ vec: second, mask: second_mask });
                        self.vecs.insert(i, ScreenedVec{ vec: first, mask: first_mask });
                        timer.splitting += start.elapsed().as_micros();
                    }
                    return true;
                }
            }
        }
        return false;
    }

}

#[derive(Debug, Clone, Default)]
struct Timer {
    searching: u128,
    splitting: u128,
    per_round: u128
}

fn do_the_mario(cups: &mut FancyVec, timer: &mut Timer) {
    let start = std::time::Instant::now();
    // Active is always in front
    let current = cups.pop();
    let a = cups.pop();
    let b = cups.pop();
    let c = cups.pop();
    let mut target = if current == 1 { 1_000_000 } else { current - 1 };
    while a == target || b == target || c == target {
        target = if target == 1 { 1_000_000 } else { target - 1 };
    }
    cups.insert_after(target, [a, b, c], timer);
    cups.push(current);
    timer.per_round += start.elapsed().as_micros();
}
fn main() {
    let mut cups: Vec<u32> = vec![5,2,3,7,6,4,8,1,9].into_iter().collect();
    cups.extend(10..1_000_001);
    let mut cups = FancyVec::from(cups);
    // let mut cups = vec![3,8,9,1,2,5,4,6,7].into_iter().collect();
    let mut timer = Timer::default();
    for i in 0..100_000_000 {
        if i % 1_000 == 0 {
            println!("{:?}", timer);
            // let cupcups: Vec<_> = cups.vecs.iter().take(10).collect();
            // println!("{:?}", cupcups);
            // let cupcups: Vec<_> = cups.vecs.iter().skip(cups.vecs.len() - 10).collect();
            // println!("{:?}", cupcups);
            println!("{}", cups.vecs.len());
            timer = Timer::default();
        }
        do_the_mario(&mut cups, &mut timer);
    }
    // println!("{:?}", cups);

}
