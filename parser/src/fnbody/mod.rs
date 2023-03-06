mod statement;
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::IResult;

use crate::expr::Expression;
use crate::{keyword, Parser};

#[derive(Debug)]
pub struct FnBody(Vec<BodyItem>);

impl Parser for FnBody {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        // {
        let (input, _) = keyword::CurlyOpen::parse(input)?;

        let (input, items) = many0(BodyItem::parse_ws)(input)?;

        // }
        let (input, _) = keyword::CurlyClose::parse_ws(input)?;

        Ok((input, FnBody(items)))
    }
}

#[derive(Debug)]
pub enum BodyItem {
    Expression(Expression),
    Statement(statement::Statement),
}

impl Parser for BodyItem {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(Expression::parse, BodyItem::Expression),
            map(statement::Statement::parse, BodyItem::Statement),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bodyitems() {
        let case = "{
  let nums = str(num)
  let nums2 = str(nums, nums)

  print(nums2)

  return true
}";
        let r = BodyItem::parse(case);
        assert!(r.is_ok(), "functionbody can be parsed");
        let (rest, _) = r.unwrap();
        assert_eq!(rest, "", "nothing remains");
    }
}
