
use nom::{IResult, bytes::complete::take_while};

pub trait Parser
where
    Self: Sized,
{
    fn parse(input: &str) -> IResult<&str, Self>;

    fn parse_ws(input: &str) -> IResult<&str, Self> {
        // skip all whitespace characters
        let (rest, _) = take_while(|c| c== ' ' || c == '\n' || c == '\r' || c == '\t')(input)?;

        Self::parse(rest)
    }
}