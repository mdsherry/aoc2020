mod part1;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input: Vec<u32> = util::parse_lines(INPUT).collect();
    input.push(0);
    input.sort_unstable();
    input.push(input[input.len()-1] + 3);
    let mut memo = vec![0; input.len()];
    memo[input.len() - 1] = 1;
    for (i, &joltage) in input.iter().enumerate().rev().skip(1) {
        let mut paths = 0usize;
        for j in (i+1)..input.len().min(i + 4) {
            if input[j] - joltage <= 3 {
                paths += memo[j];
            } else {
                break;
            }
        }
        memo[i] = paths;
    }
    println!("{:?}", memo);
    println!("{}", memo[0]);
}
