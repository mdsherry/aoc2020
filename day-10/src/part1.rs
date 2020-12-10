static INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input: Vec<u32> = util::parse_lines(INPUT).collect();
    input.push(0);
    input.sort_unstable();
    input.push(input[input.len()-1] + 3);
    let mut diff_1 = 0;
    let mut diff_2 = 0;
    let mut diff_3 = 0;
    for window in input.windows(2) {
        match window[1] - window[0] {
            0 => panic!("Dupliate"),
            1 => diff_1 += 1,
            2 => diff_2 += 1,
            3 => diff_3 += 1,
            _ => panic!("Invalid gap")
        }
    }
    println!("{} * {} = {}", diff_1, diff_3, diff_1 * diff_3);
}
