use nom::{branch::alt, combinator::map, multi::separated_list0, IResult};

use crate::{keyword, util::paren, Ident, Parser};

#[derive(Debug)]
pub enum Expression {
    FunctionCall {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
    BoolLiteral(bool),
    Null,
    Ident(String),
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
