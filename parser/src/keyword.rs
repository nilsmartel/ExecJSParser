use nom::bytes::complete::tag;

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

kw!(Function, "function");

kw!(Open, "(");
kw!(Close, ")");

kw!(CurlyOpen, "{");
kw!(CurlyClose, "}");

kw!(BracketOpen, "[");
kw!(BracketClose, "]");

kw!(Comma, ",");