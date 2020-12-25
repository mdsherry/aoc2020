use std::collections::HashSet;

fn do_thing(number: usize, subject: usize) -> usize {
    (number * subject) % 20201227
}

fn main() {
    let mut value = 1;
    
    for i in 1..20000000 {
        value = do_thing(value, 7);
        if value == 1614360 || value == 7734663 {
            println!("{}: {}", i, value);
        }
    }
    
    let mut value = 1;
    for _ in 0..1182212 {
        value = do_thing(value, 7734663);
    }
    println!("{}", value);

    let mut value = 1;
    for _ in 0..4744857 {
        value = do_thing(value, 1614360);
    }
    println!("{}", value);
    // 1614360
// 7734663
    
    println!("Hello, world!");
}
