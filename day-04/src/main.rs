static INPUT: &str = include_str!("input.txt");

fn main() {
    let valid_count = INPUT.split("\n\n").filter(|s| is_valid(*s)).count();
    println!("{}", valid_count);
}

#[derive(Copy, Clone, Debug)]
enum Field {
    Byr = 0,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

fn is_valid(input: &str) -> bool {
    println!("Field count: {}", input.split_whitespace().count());
    input
        .split_whitespace()
        .filter_map(parse_field)
        .filter(|(field, value)| validate_value(*field, value))
        .fold(0u8, |mask, (field, _)| mask | (1 << (field as u8)))
        & 0b0111_1111
        == 0b0111_1111
}

fn year_in_range(s: &str, lower: u16, upper: u16) -> bool {
    match s.parse::<u16>() {
        Err(_) => false,
        Ok(year) => year <= upper && year >= lower
    }
}

fn validate_value(field: Field, value: &str) -> bool {
    use Field::*;
    match field {
        Byr => year_in_range(value, 1920, 2002),
        Iyr => year_in_range(value, 2010, 2020),
        Eyr => year_in_range(value, 2020, 2030),
        Hgt => if value.ends_with("cm") {
            year_in_range(value[..value.len() - 2].trim(), 150, 193)
        } else if value.ends_with("in") {
            year_in_range(value[..value.len() - 2].trim(), 59, 76)
        } else {false},
        Hcl => value.starts_with('#') && value.chars().skip(1).all(|c| c.is_ascii_hexdigit()),
        Ecl => match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false
        },
        Pid => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
        Cid => true
    }
}

fn parse_field(fld_value: &str) -> Option<(Field, &str)> {
    use Field::*;
    let mut it = fld_value.split(':');
    let field = it.next().unwrap().trim();
    let value = it.next().unwrap().trim();
    let field = match field {
        "byr" => Byr,
        "iyr" => Iyr,
        "eyr" => Eyr,
        "hgt" => Hgt,
        "hcl" => Hcl,
        "ecl" => Ecl,
        "pid" => Pid,
        "cid" => Cid,
        _ => return None,
    };
    Some((field, value))

}
