
use nom::{IResult, bytes::complete::take_while};

pub trait Parser
where
    Self: Sized,
{
    fn parse(input: &str) -> IResult<&str, Self>;

    fn parse_ws(input: &str) -> IResult<&str, Self> {
        // skip all the whitespace
        let (rest, _) = take_while(|c| matches!(c, ' ' | '\n' | '\t' | '\r'))(input)?;

        Self::parse(rest)
    }
}