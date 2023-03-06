use nom::{
    bytes::complete::{tag, take_till, take_while},
    FindSubstring, IResult,
};

use crate::keyword;

pub trait Parser
where
    Self: Sized,
{
    fn parse(input: &str) -> IResult<&str, Self>;

    fn parse_ws(input: &str) -> IResult<&str, Self> {
        // skip all the whitespace
        let (rest, _) = take_while(|c| matches!(c, ' ' | '\n' | '\t' | '\r'))(input)?;

        // ignore line comments
        if let Some(rest) = rest.strip_prefix("//") {
            // skip to the end of the line
            let (rest, _) = take_till(|c| c == '\n')(rest)?;

            return Self::parse_ws(rest);
        }

        // ignore multiline comments
        if let Some(rest) = rest.strip_prefix("/*") {
            let pos = rest.find_substring("*/");

            let pos = match pos {
                Some(pos) => pos,
                None => {
                    // this will diverge with some error.
                    tag("*/")(rest)?;
                    unreachable!()
                }
            };

            let pos = pos + 2;

            // rest is now everything that comes after the comment.
            let rest = &rest[pos..];

            return Self::parse_ws(rest);
        }

        Self::parse(rest)
    }
}

/// Wraps a parser into recognizing parenthesis
pub fn paren<T>(parser: impl Fn(&str) -> IResult<&str, T>) -> impl Fn(&str) -> IResult<&str, T> {
    move |input: &str| {
        let (input, _) = keyword::Open::parse(input)?;
        let (input, result) = parser(input)?;
        let (input, _) = keyword::Close::parse_ws(input)?;

        Ok((input, result))
    }
}
