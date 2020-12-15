static START: usize = 1002632;
static BUS_TIMES: &str = "23,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,829,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,677,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19";
fn main() {
    let bus_times: Vec<_> = BUS_TIMES.split(',').enumerate().filter(|s| s.1 != "x").map(|s| (s.0, s.1.parse::<usize>().unwrap())).collect();
    let mut residues = vec![];
    let mut modulii = vec![];
    for (i, time) in bus_times.iter().copied() {
        // println!("{} mod {}", (100 * time - i) % time, time);
        residues.push(((100 * time - i) % time) as i64);
        modulii.push(time as i64);
    }
    println!("{:?}", chinese_remainder(&residues, &modulii));
    // First: 0 mod 23
    // Second: 41 - 13 = 28 mod 41
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}