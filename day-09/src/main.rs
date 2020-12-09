mod part1;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let input: Vec<u64> = util::parse_lines(INPUT).collect();
    const target: u64 = 375054920;
    'outer: for i in 0..(input.len() - 1) {
        let mut total = input[i];
        if total > target {
            continue;
        }
        for j in (i+1)..input.len() {
            total += input[j];
            if total == target {
                let smallest = input[i..(j+1)].iter().min().unwrap();
                let largest = input[i..(j+1)].iter().max().unwrap();
                println!("{} + {} = {}", smallest, largest, smallest + largest);
                break 'outer;
            } else if total > target {
                continue 'outer;
            }
        }
    }
}
