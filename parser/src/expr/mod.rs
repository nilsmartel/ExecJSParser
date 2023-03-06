use nom::{branch::alt, combinator::map, multi::separated_list0, IResult};

use crate::{keyword, util::paren, Ident, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    FunctionCall {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
    BoolLiteral(bool),
    Null,
    Ident(String),
}

#[cfg(test)]
mod expression_tests {
    use super::*;

    #[test]
    fn funcall() {
        let input = "hello(hello(), hello)";
        let r = parse_functioncall(input);
        assert!(r.is_ok(), "expect parser to succeed");
        let (rest, _f) = r.unwrap();
        assert_eq!(rest, "", "expect parser consume all content");
    }

    #[test]
    fn boollit1() {
        let input = "true";
        let r = parse_bool(input);
        assert!(r.is_ok(), "expect parser to succeed");
        let (rest, _f) = r.unwrap();
        assert_eq!(rest, "", "expect parser consume all content");
    }

    #[test]
    fn boollit2() {
        let input = "false";
        let r = parse_bool(input);
        assert!(r.is_ok(), "expect parser to succeed");
        let (rest, _f) = r.unwrap();
        assert_eq!(rest, "", "expect parser consume all content");
    }

    #[test]
    fn nully() {
        let input = "null";
        let r = Expression::parse(input);
        assert!(r.is_ok(), "expect parser to succeed");
        let (rest, f) = r.unwrap();
        assert_eq!(rest, "", "expect parser consume all content");

        assert_eq!(f, Expression::Null);
    }
}

impl Parser for Expression {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        alt((
            parse_bool,
            map(keyword::Null::parse, |_| Expression::Null),
            parse_functioncall,
            map(Ident::parse, |Ident(i)| Expression::Ident(i)),
        ))(input)
    }
}

fn parse_bool(input: &str) -> IResult<&str, Expression> {
    alt((
        map(keyword::False::parse, |_| Expression::BoolLiteral(false)),
        map(keyword::True::parse, |_| Expression::BoolLiteral(true)),
    ))(input)
}

fn parse_functioncall(input: &str) -> IResult<&str, Expression> {
    // print
    // but it could also be (something)()
    // So we only accept literals
    // or expressions in parenthesis
    let (input, func) = if let Ok((input, ident)) = Ident::parse(input) {
        (input, Expression::Ident(ident.0))
    } else {
        // we can't fail here.
        let (input, func) = paren(Expression::parse_ws)(input)?;
        (input, func)
    };
    // let's box out func

    let func = Box::new(func);

    // (
    let (input, _) = keyword::Open::parse_ws(input)?;

    let (input, args) = separated_list0(keyword::Comma::parse_ws, Expression::parse_ws)(input)?;

    // )
    let (input, _) = keyword::Close::parse_ws(input)?;

    Ok((input, Expression::FunctionCall { func, args }))
}
