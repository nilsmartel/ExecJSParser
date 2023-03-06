
use nom::combinator::cut;

use crate::Ident;

use nom::branch::alt;

use crate::keyword;

use nom::sequence::preceded;

use nom::combinator::map;

use nom::IResult;

use crate::Parser;

use super::Expression;

#[derive(Debug)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return(Expression),
}

impl Parser for Statement {
    fn parse(input: &str) -> IResult<&str, Self> {
        let return_parser = map(
            preceded(keyword::Return::parse, Expression::parse_ws),
            Statement::Return,
        );

        alt((return_parser, parse_let))(input)
    }
}

pub(crate) fn parse_let(input: &str) -> IResult<&str, Statement> {
    let (input, _) = keyword::Let::parse(input)?;
    let (input, Ident(name)) = cut(Ident::parse_ws)(input)?;
    let (input, _) = cut(keyword::Assign::parse_ws)(input)?;
    let (input, value) = cut(Expression::parse_ws)(input)?;

    Ok((input, Statement::Let { name, value }))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn letitbe() {
        let cases = ["let x = true", "let nums = str(nums)"];
        for c in cases {
            let r = parse_let(c);
            assert!(r.is_ok(), "expect let to be parsed");
            let (rest, _value) = r.unwrap();
            assert_eq!(rest, "", "expect no input to remain")
        }
    }
}
