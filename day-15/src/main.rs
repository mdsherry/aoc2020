use std::collections::HashMap;



fn main() {
    let input = vec![16,12,1,0,15,7,11];
    let mut memory: HashMap<_, _> = input[..input.len()-1].iter().copied().enumerate().map(|(a,b)| (b, a + 1)).collect();
    let mut last = *input.last().unwrap();    
    for turn in input.len() + 1..30000001 {
        let new_last = if let Some(prev_turn) = memory.get(&last).copied() {
            turn - 1 - prev_turn
        } else { 0 };
        memory.insert(last, turn - 1);
        last = new_last;
    }
    println!("{}", last);
}
