
static INPUT: &str = include_str!("input.txt");

fn main() {
    let input: Vec<u64> = util::parse_lines(INPUT).collect();
    'outer: for (idx, window) in input.windows(26).enumerate() {
        let target = window[25];
        for i in 0..24 {
            for j in (i+1)..25 {
                if window[i] + window[j] == target {
                    continue 'outer;
                }
            }
        }
        println!("Miss at {} (target: {})", idx, target);
    }
}
