use std::collections::{HashMap, VecDeque};


#[derive(Debug, Clone, Default)]
struct Timer {
    searching: u128,
    splitting: u128,
    per_round: u128
}


fn do_the_mario(cups: &mut VecDeque<u32>, after: &mut HashMap<u32, [u32; 3]>, timer: &mut Timer) {
    let start = std::time::Instant::now();
    // Active is always in front
    let current = cups.pop_front().unwrap();
    if let Some([a, b, c]) = after.remove(&current) {
        cups.push_front(c);
        cups.push_front(b);
        cups.push_front(a);
    }
    let a = cups.pop_front().unwrap();
    if let Some([a, b, c]) = after.remove(&a) {
        cups.push_front(c);
        cups.push_front(b);
        cups.push_front(a);
    }
    let b = cups.pop_front().unwrap();
    if let Some([a, b, c]) = after.remove(&b) {
        cups.push_front(c);
        cups.push_front(b);
        cups.push_front(a);
    }
    let c = cups.pop_front().unwrap();
    if let Some([a, b, c]) = after.remove(&c) {
        cups.push_front(c);
        cups.push_front(b);
        cups.push_front(a);
    }

    
    
    let mut target = if current == 1 { 1_000_000 } else { current - 1 };
    while a == target || b == target || c == target {
        target = if target == 1 { 1_000_000 } else { target - 1 };
    }
    after.insert(target, [a, b, c]);
    cups.push_back(current);
    timer.per_round += start.elapsed().as_micros();
}

fn main() {
    let mut cups: VecDeque<u32> = vec![5,2,3,7,6,4,8,1,9].into_iter().collect();
    cups.extend(10..1_000_001);
    
    // let mut cups = vec![3,8,9,1,2,5,4,6,7].into_iter().collect();
    let mut timer = Timer::default();
    let mut after = HashMap::new();
    for i in 0..10_000_000 {
        if i % 1_000_000 == 0 {
            println!("{}: {:?}", i / 1_000_000, timer);
            // println!("{}", cups.len());
            timer = Timer::default();
        }
        do_the_mario(&mut cups, &mut after, &mut timer);
    }
    let mut result = vec![];
    while let Some(value) = cups.pop_front() {
        result.push(value);
        if let Some([a, b, c]) = after.remove(&value) {
            cups.push_front(c);
            cups.push_front(b);
            cups.push_front(a);
        }
    }
    
    let mut it = result.into_iter().skip_while(|v| *v != 1).skip(1).take(2);
    let a = it.next().unwrap();
    let b = it.next().unwrap();
    println!("{} * {} = {}", a, b, a as u64 * b as u64);

    
    // println!("{:?}", cups);

}
