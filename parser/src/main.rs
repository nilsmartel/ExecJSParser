mod keyword;
mod util;

use nom::{
    bytes::complete::{take_while, take_while1},
    combinator::{cut, map, recognize},
    multi::{many0, separated_list0},
    sequence::pair,
};
pub use util::Parser;

fn main() {
    // let input = include_str!("../../Source/1-0-functions.js");
    let input = include_str!("../../Source/1-1-comments.js");

    let (rest, ast) = AstBody::parse_ws(input).unwrap();

    dbg!(ast);
    dbg!(rest);
}

#[derive(Debug)]
pub struct AstBody {
    items: Vec<FnDef>,
}

impl Parser for AstBody {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (rest, items) = many0(FnDef::parse_ws)(input)?;

        Ok((rest, AstBody { items }))
    }
}

#[derive(Debug)]
pub struct FnDef {
    name: String,
    args: Vec<String>,
    body: FnBody,
}

impl Parser for FnDef {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        // function
        let (rest, _) = keyword::Function::parse(input)?;

        // <ident>
        let (rest, Ident(name)) = cut(Ident::parse_ws)(rest)?;

        // (a, b, c)

        let (rest, Args(args)) = cut(Args::parse_ws)(rest)?;

        // {}
        let (rest, body) = cut(FnBody::parse_ws)(rest)?;

        Ok((rest, FnDef { name, args, body }))
    }
}

#[derive(Debug)]
struct FnBody;

impl Parser for FnBody {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        // {
        let (rest, _) = keyword::CurlyOpen::parse(input)?;
        // }
        let (rest, _) = keyword::CurlyClose::parse_ws(rest)?;

        Ok((rest, FnBody))
    }
}

#[derive(Debug)]
struct Ident(String);

impl Parser for Ident {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let first = take_while1(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'));

        let second = take_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'));

        let (rest, ident) = recognize(pair(first, second))(input)?;

        let ident = ident.to_string();

        Ok((rest, Ident(ident)))
    }
}

struct Args(Vec<String>);

impl Parser for Args {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        // (
        let (rest, _) = keyword::Open::parse(input)?;

        let (rest, args) = separated_list0(
            keyword::Comma::parse_ws,
            map(Ident::parse_ws, |ident| ident.0),
        )(rest)?;

        // )
        let (rest, _) = keyword::Close::parse_ws(rest)?;

        Ok((rest, Args(args)))
    }
}
