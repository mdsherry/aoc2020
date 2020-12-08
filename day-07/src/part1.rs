use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("input.txt");

fn main() {
    let rules: HashMap<_, _> = INPUT.lines().map(|line| parse_rule(line)).map(|rule| (rule.container, rule)).collect();
    let mut inv = HashMap::<Bag, Vec<&Rule>>::new();
    for rule in rules.values() {
        for (_, subbag) in &rule.contents {
            inv.entry(*subbag).or_default().push(rule);
        }
    }
    let mut seen = HashSet::new();
    let mut new_seen = Vec::new();
    new_seen.push(Bag { modifier: Modifier::Shiny, colour: Colour::Gold });
    while let Some(bag) = new_seen.pop() {
        if let Some(rules) = inv.get(&bag) {
            for holder in rules {
                if seen.insert(holder.container) {
                    new_seen.push(holder.container);
                }
            }
        }
    }
    println!("{}", seen.len());
}

fn parse_rule(line: &str) -> Rule {
    let line = line.trim_end_matches('.');
    let mut it = line.split(" bags contain ");
    let bag = Bag::parse(it.next().unwrap());
    let rest = it.next().unwrap();
    if rest == "no other bags" {
        Rule { container: bag, contents: vec![] }
    } else {
        let mut contents = vec![];
        for content in rest.split(", ") {
            let mut it = content.splitn(2, " ");
            let count = it.next().unwrap().parse::<usize>().map_err(|e| panic!("Bad number {}", content)).unwrap();
            let bag = Bag::parse(it.next().unwrap().trim_end_matches("bag").trim_end_matches("bags"));
            contents.push((count, bag));
        }
        Rule { container: bag, contents }
    }
}

#[derive(Debug)]
struct Rule {
    container: Bag,
    contents: Vec<(usize, Bag)>
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Bag {
    modifier: Modifier,
    colour: Colour,
}

impl Bag {
    fn parse(s: &str) -> Self {
        let mut it = s.split(" ");
        let modifier = Modifier::parse(it.next().unwrap());
        let colour = Colour::parse(it.next().unwrap());
        Bag { modifier, colour }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Modifier {
    Bright,
    Pale,
    Mirrored,
    Shiny,
    Dotted,
    Drab,
    Striped,
    Wavy,
    Light,
    Clear,
    Dull,
    Plaid,
    Dim,
    Faded,
    Posh,
    Dark,
    Vibrant,
    Muted,
}

impl Modifier {
    fn parse(s: &str) -> Self {
        use Modifier::*;
        match s {
            "bright" => Bright,
            "pale" => Pale,
            "mirrored" => Mirrored,
            "shiny" => Shiny,
            "dotted" => Dotted,
            "drab" => Drab,
            "striped" => Striped,
            "wavy" => Wavy,
            "light" => Light,
            "clear" => Clear,
            "dull" => Dull,
            "plaid" => Plaid,
            "dim" => Dim,
            "faded" => Faded,
            "posh" => Posh,
            "dark" => Dark,
            "vibrant" => Vibrant,
            "muted" => Muted,
                        _ => {eprintln!("Don't know modifier {}", s); Bright}
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Colour {
    Gray,
    Gold,
    Lavender,
    Olive,
    Yellow,
    Salmon,
    Magenta,
    Turquoise,
    Fuchsia,
    Lime,
    Plum,
    Blue,
    Beige,
    Bronze,
    Tomato,
    Red,
    Black,
    White,
    Orange,
    Purple,
    Chartreuse,
    Tan,
    Violet,
    Maroon,
    Brown,
    Crimson,
    Coral,
    Silver,
    Green,
    Indigo,
    Cyan,
    Aqua,
    Teal
}

impl Colour {
    fn parse(s: &str) -> Self {
        use Colour::*;
        match s {
            "gray" => Gray,
            "gold" => Gold,
            "lavender" => Lavender,
            "olive" => Olive,
            "yellow" => Yellow,
            "salmon" => Salmon,
            "magenta" => Magenta,
            "turquoise" => Turquoise,
            "fuchsia" => Fuchsia,
            "lime" => Lime,
            "plum" => Plum,
            "blue" => Blue,
            "beige" => Beige,
            "bronze" => Bronze,
            "tomato" => Tomato,
            "red" => Red,
            "black" => Black,
            "white" => White,
            "orange" => Orange,
            "purple" => Purple,
            "chartreuse" => Chartreuse,
            "tan" => Tan,
            "violet" => Violet,
            "maroon" => Maroon,
            "brown" => Brown,
            "crimson" => Crimson,
            "coral" => Coral,
            "silver" => Silver,
            "green" => Green,
            "indigo" => Indigo,
            "cyan" => Cyan,
            "aqua" => Aqua,
            "teal" => Teal,
            _ => {eprintln!("Don't know color {}", s); Gray}
        }
    }
}