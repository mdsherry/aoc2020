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

    fn check<'a, 'b>(&self, rule_store: &'b HashMap<usize, Rule>, s: &'a str) -> Vec<&'a str> {
        match self {
            Rule::Lit(ch) => s.strip_prefix(*ch).into_iter().collect(),
            Rule::Id(id) => rule_store[id].check(rule_store, s),
            Rule::Seq(rules) => {
                let mut state = vec![s];
                for rule in rules {
                    let mut new_state = vec![];
                    new_state.extend(state.iter().flat_map(|s| rule.check(rule_store, s)));
                    std::mem::swap(&mut state, &mut new_state);
                }
                state
            },
            Rule::Alt(rules) => {
                rules.iter().flat_map(|rule| rule.check(rule_store, s)).collect()
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
    rule_store.insert(8, Rule::from_str("42 8 | 42"));
    rule_store.insert(11, Rule::from_str("42 11 31| 42 31"));
    // println!("{:#?}", rule_store);

    let startend = Rule::Seq(vec![Rule::Lit('^'), Rule::Id(0), Rule::Lit('$')]);
    
    let good_count = input.lines().skip_while(|s| !s.is_empty()).skip(1).filter(|&s|  {
        let s = format!("^{}$", s);
        let result = startend.check(&rule_store, &s);
        result.iter().any(|r| r.is_empty())
    }).count();

    println!("{}", good_count);
}
