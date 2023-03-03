mod keyword;
mod util;

use nom::{
    bytes::complete::{take_while, take_while1},
    combinator::{self, cut, recognize},
    multi::{many0, separated_list0},
    sequence::pair,
};
pub use util::Parser;
fn main() {
    let input = include_str!("../../Source/1-0-functions.js");

    let (rest, ast) = AstBody::parse_ws(input).unwrap();

    dbg!(ast);
    dbg!(rest);
}

#[derive(Debug)]
pub struct AstBody {
    // TODO imports etc.
    items: Vec<FnDef>,
}

impl Parser for AstBody {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (rest, items) = many0(FnDef::parse_ws)(input)?;

        Ok((rest, AstBody { items }))
    }
}

#[derive(Debug)]
struct FnDef {
    name: String,
    args: Vec<String>,
    body: FnBody,
}

impl Parser for FnDef {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        // first we need the function keyword
        let (rest, _) = keyword::Function::parse(input)?;

        // the name of the function
        let (rest, Ident(name)) = cut(Ident::parse_ws)(rest)?;

        // arguments!
        let (rest, Args(args)) = cut(Args::parse_ws)(rest)?;

        // body
        let (rest, body) = cut(FnBody::parse_ws)(rest)?;

        Ok((rest, FnDef { name, args, body }))
    }
}

#[derive(Debug)]
struct Args(Vec<String>);

impl Parser for Args {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        // (
        let (rest, _) = keyword::Open::parse(input)?;

        // a,b, c
        let (rest, args) = separated_list0(
            keyword::Comma::parse_ws,
            combinator::map(Ident::parse_ws, |i| i.0),
        )(rest)?;

        // )
        let (rest, _) = keyword::Close::parse_ws(rest)?;

        Ok((rest, Args(args)))
    }
}

#[derive(Debug)]
struct FnBody;

impl Parser for FnBody {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (rest, _) = pair(keyword::CurlyOpen::parse, keyword::CurlyClose::parse_ws)(input)?;

        Ok((rest, FnBody))
    }
}

#[derive(Debug)]
struct Ident(String);

impl Parser for Ident {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let startchars = take_while1(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'));
        let endchars = take_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'));

        let (rest, ident) = recognize(pair(startchars, endchars))(input)?;
        let ident = ident.to_string();

        Ok((rest, Ident(ident)))
    }
}

#[cfg(test)]
mod ident_tests {
    use super::*;
    #[test]
    fn simple() {
        let cases = ["hello", "_h", "_", "a", "a367826432748_", "x1", "x_1"];
        for x in cases {
            assert!(
                Ident::parse(x).is_ok(),
                "expect {x} to be a valid identifier"
            );
        }
    }
}
