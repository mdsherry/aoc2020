#[derive(Debug, Clone)]
struct Field {
    name: &'static str,
    first_low: u32,
    first_high: u32,
    second_low: u32,
    second_high: u32,
}

impl Field {
    fn parse(s: &'static str) -> Field {
        let mut it = s.split(": ");
        let name = it.next().unwrap();
        let rest = it.next().unwrap();
        let mut parts = rest.split(" or ");
        let first = parts.next().unwrap().split("-");
        let second = parts.next().unwrap().split("-");
        fn to_range(mut it: impl Iterator<Item = &'static str>) -> (u32, u32) {
            let lower = it.next().unwrap().parse().unwrap();
            let upper = it.next().unwrap().parse().unwrap();
            (lower, upper)
        }
        let (first_low, first_high) = to_range(first);
        let (second_low, second_high) = to_range(second);
        Field {
            name,
            first_low,
            first_high,
            second_low,
            second_high,
        }
    }

    fn valid(&self, value: u32) -> bool {
        (value >= self.first_low && value <= self.first_high)
            || (value >= self.second_low && value <= self.second_high)
    }
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    let mut fields = vec![];
    for field in INPUT.lines().take_while(|line| !line.is_empty()) {
        fields.push(Field::parse(field));
    }
    println!("{:?}", fields);
    let mut tickets = vec![];
    for ticket in INPUT.lines().skip_while(|line| !line.is_empty()).skip(5) {
        let ticket = ticket
            .split(',')
            .map(|value| value.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        if !ticket
            .iter()
            .copied()
            .all(|value| fields.iter().any(|field| field.valid(value)))
        {
            continue;
        }
        tickets.push(ticket);
    }
    println!("{}", tickets.len());
    let mut masks = vec![];
    for n in 0..fields.len() {
        let mut common_mask = 0xffffffu32;
        for ticket in &tickets {
            let value = ticket[n];
            let mut mask = 0;
            for (i, field) in fields.iter().enumerate() {
                if field.valid(value) {
                    mask |= 1 << i;
                }
            }
            common_mask &= mask;
        }
        // println!("{:b}", common_mask);
        masks.push(common_mask);
    }
    for _ in 0..masks.len() {
        for i in 0..masks.len() {
            if masks[i].count_ones() == 1 {
                for j in 0..masks.len() {
                    if i != j {
                        masks[j] &= !masks[i];
                    }
                }
            }
        }
    }
    // for mask in &masks {
    //     println!("{:b}", mask);
    // }
    let mut departure_indices = vec![];
    for (n, mask) in masks.iter().enumerate() {
        for (i, field) in fields.iter().enumerate() {
            if (1 << i) == *mask {
                if field.name.starts_with("departure") {
                    departure_indices.push(n);
                }
                println!("{}: {}", n, field.name);
                break;
            }
        }
    }
    let my_ticket = INPUT
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(2)
        .next()
        .unwrap();
    let prod = my_ticket
        .split(',')
        .enumerate()
        .filter(|(i, _)| departure_indices.contains(i))
        .map(|(_, v)| v.parse::<u64>().unwrap())
        .inspect(|v| println!("{}", v))
        .fold(1, |a, b| a * b);
    println!("prod: {}", prod);
}
