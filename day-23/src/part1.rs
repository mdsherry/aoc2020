use std::collections::VecDeque;

fn do_the_mario(cups: &mut VecDeque<u32>) {
    // Active is always in front
    let current = cups.pop_front().unwrap();
    let a = cups.pop_front().unwrap();
    let b = cups.pop_front().unwrap();
    let c = cups.pop_front().unwrap();
    let mut target = if current == 1 { 9 } else { current - 1 };
    cups.push_front(current);
    while !cups.contains(&target) {
        target = if target == 1 { 9 } else { target - 1 };
    }
    let target_idx = cups.iter().position(|x| *x == target).unwrap();
    cups.insert(target_idx + 1, c);
    cups.insert(target_idx + 1, b);
    cups.insert(target_idx + 1, a);
    cups.rotate_left(1);
}
fn main() {
    let mut cups: VecDeque<u32> = vec![5,2,3,7,6,4,8,1,9].into_iter().collect();
    
    // let mut cups = vec![3,8,9,1,2,5,4,6,7].into_iter().collect();
    for i in 0..100 {
        do_the_mario(&mut cups);
    }
    // println!("{:?}", cups);

}
