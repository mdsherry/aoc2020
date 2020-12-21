use nom::{IResult, branch::alt, bytes::complete::tag, character::{complete::digit1, streaming::multispace0}, combinator::complete, multi::many1, sequence::{delimited, preceded}};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number(i64),
    Plus,
    Times,
    Subexpr(Vec<Expr>)
}

fn parens(i: &str) -> IResult<&str, Expr> {
    // eprintln!("parens? {}", i);
    let (i, result) = preceded(multispace0, delimited(tag("("), expr, tag(")")))(i)?;
    Ok((i, Expr::Subexpr(result)))
}

fn digit(i: &str) -> IResult<&str, Expr> {
    // eprintln!("digit? {}.", i);
    let (i, num) = preceded(multispace0, digit1)(i)?;
    // eprintln!("digit! {}", num);
    Ok((i, Expr::Number(num.parse().unwrap())))
}

fn times(i: &str) -> IResult<&str, Expr> {
    // eprintln!("times? {}", i);
    let (i, _) = preceded(multispace0, tag("*"))(i)?;
    Ok((i, Expr::Times))
}

fn plus(i: &str) -> IResult<&str, Expr> {
    // eprintln!("plus? {}", i);
    let (i, _) = preceded(multispace0, tag("+"))(i)?;
    Ok((i, Expr::Plus))
}

fn expr(s: &str) -> IResult<&str, Vec<Expr>> {
    let (i, exprs) = many1(complete(alt((digit, times, plus, parens))))(s)?;
    Ok((i, exprs))
}

fn eval(exprs: &[Expr]) -> i64 {
    let mut left = 0;
    let mut op = None;
    for expr in exprs {
        // dbg!((expr, left, &op));
        match expr {
            Expr::Number(n) => {
                match op {
                    None => left = *n,
                    Some(Expr::Plus) => left = left + *n,
                    Some(Expr::Times) => left = left * *n,
                    _ => println!("Invalid op")
                }
                op = None;
            }
            seen_op@Expr::Plus | seen_op@Expr::Times => {
                op = Some(seen_op.clone());
            }
            Expr::Subexpr(exprs) => {
                let n = eval(exprs);
                match op {
                    None => left = n,
                    Some(Expr::Plus) => left = left + n,
                    Some(Expr::Times) => left = left * n,
                    _ => println!("Invalid op")
                }
            }
        }
    }
    left
}

fn compute(s: &str) -> i64 {
    let exprs = expr(s).unwrap();
    eval(&exprs.1)
}

fn main() {
    let mut total = 0;
    for line in include_str!("input.txt").lines() {
        total += compute(line);
    }
    
    println!("{}", total);
}

#[cfg(test)]
mod test {
    use super::compute;
    #[test]
    fn blah() {
        assert_eq!(13632, compute("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }
}