use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::not,
};

macro_rules! kw {
    ($name: ident, $content: expr) => {
        pub struct $name;

        impl crate::Parser for $name {
            fn parse(input: &str) -> nom::IResult<&str, Self> {
                let (rest, _) = tag($content)(input)?;

                Ok((rest, $name))
            }
        }
    };
}

macro_rules! kwt {
    ($name: ident, $content: expr) => {
        pub struct $name;

        impl crate::Parser for $name {
            fn parse(input: &str) -> nom::IResult<&str, Self> {
                let (rest, _) = tag($content)(input)?;
                let cond = |c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9');
                let (rest, _) = not(take_while_m_n(1, 1, cond))(rest)?;

                Ok((rest, $name))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::Parser;

    use super::*;
    #[test]
    fn terminated_keyword() {
        let cases = ["false", "false xyz", "false abc", "false-abc"];

        for c in cases {
            let p = False::parse(c);
            assert!(p.is_ok(), "expect terminated parser to succeed");
        }

        let cases = ["false8", "falsexyz", "false_abc"];

        for c in cases {
            let p = False::parse(c);
            assert!(p.is_err(), "expect terminated parser to fail");
        }
    }

    #[test]
    fn ass() {
        let r = Assign::parse_ws("  =");
        assert!(r.is_ok(), "expect = to be parsable");
    }

    #[test]
    fn letitbe() {
        let r = Let::parse_ws("let x =");
        assert!(r.is_ok(), "expect let to be parsable");
        let (rest, _) = r.unwrap();
        assert_eq!(rest, " x =")
    }
}

kwt!(Function, "function");
kwt!(False, "false");
kwt!(True, "true");
kwt!(Return, "return");
kwt!(Let, "let");
kwt!(Null, "null");

kw!(Open, "(");
kw!(Close, ")");
kw!(Assign, "=");

kw!(CurlyOpen, "{");
kw!(CurlyClose, "}");

kw!(BracketOpen, "[");
kw!(BracketClose, "]");

kw!(Comma, ",");