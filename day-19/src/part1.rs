use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule {
    Lit(char),
    Id(usize),
    Seq(Vec<Rule>),
    Alt(Vec<Rule>),
}

impl Rule {
    fn from_str(s: &str) -> Rule {
        if s.starts_with('"') {
            Rule::Lit(s[1..].chars().next().unwrap())
        } else if s.contains('|') {
            Rule::Alt(s.split('|').map(|s| Rule::from_str(s.trim())).collect())
        } else if s.contains(' ') {
            Rule::Seq(s.split(' ').map(Rule::from_str).collect())
        } else {
            Rule::Id(s.parse().unwrap())
        }
    }

    fn check<'a, 'b>(&self, rule_store: &'b HashMap<usize, Rule>, s: &'a str) -> Option<&'a str> {
        match self {
            Rule::Lit(ch) => {
                if let Some(new_s) = s.strip_prefix(*ch) {
                    Some(new_s)
                } else {
                    None
                }
            }
            Rule::Id(id) => rule_store[id].check(rule_store, s),
            Rule::Seq(rules) => rules.iter().fold(Some(s), |maybe_s, rule| {
                maybe_s.and_then(|s| rule.check(rule_store, s))
            }),
            Rule::Alt(rules) => {
                for rule in rules {
                    let result = rule.check(rule_store, s);
                    if result.is_some() {
                        return result;
                    }
                }
                None
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut rule_store: HashMap<_, _> = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            let (rule_id, rule_text) = s.split(": ").collect_tuple().unwrap();
            let id = rule_id.parse::<usize>().unwrap();
            let rule = Rule::from_str(rule_text);
            (id, rule)
        })
        .collect();
    // rule_store.insert(8, Rule::from_str("42 8 | 42"));
    // rule_store.insert(11, Rule::from_str("42 31| 42 11 31"));
    // println!("{:#?}", rule_store);

    let good_count = input.lines().skip_while(|s| !s.is_empty()).skip(1).filter(|&s|  {
        let result = rule_store[&0].check(&rule_store, s);
        if let Some(s) = result { s.is_empty() } else { false }
    }).count();

    println!("{}", good_count);
}
