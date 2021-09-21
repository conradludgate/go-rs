use nom::{
    branch::alt,
    character::complete::{none_of, one_of},
    combinator::recognize,
    error::{context, ParseError},
    multi::{count, many0},
    sequence::{delimited, pair},
    IResult,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag};

use super::{
    extra::{matches, ws},
    Parse, Span,
};

#[derive(Debug)]
pub enum StrLit<'a> {
    Rune(RuneLit<'a>),
    String(StringLit<'a>),
    Raw(RawLit<'a>),
}

impl<'a> Parse<'a> for StrLit<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        if matches(ws(tag("'")), &s) {
            let (s, hex) = context("StrLit", RuneLit::parse)(s)?;
            Ok((s, Self::Rune(hex)))
        } else if matches(ws(tag("\"")), &s) {
            let (s, bin) = context("StrLit", StringLit::parse)(s)?;
            Ok((s, Self::String(bin)))
        } else if matches(ws(tag("`")), &s) {
            let (s, oct) = context("StrLit", RawLit::parse)(s)?;
            Ok((s, Self::Raw(oct)))
        } else {
            Err(nom::Err::Error(ErrorTree::from_error_kind(
                s,
                nom::error::ErrorKind::Alt,
            )))
        }
    }
}

#[derive(Debug)]
pub struct RuneLit<'a> {
    rune: Span<'a>,
}

impl<'a> Parse<'a> for RuneLit<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        let (s, rune) = context(
            "RuneLit",
            ws(recognize(delimited(
                tag("'"),
                alt((
                    recognize(pair(tag("\\U"), count(one_of("0123456789abcdefABCDEF"), 8))),
                    recognize(pair(tag("\\u"), count(one_of("0123456789abcdefABCDEF"), 4))),
                    recognize(pair(tag("\\x"), count(one_of("0123456789abcdefABCDEF"), 2))),
                    recognize(pair(tag("\\"), count(one_of("01234567"), 3))),
                    recognize(pair(tag("\\"), one_of("\\'abfnrtv"))),
                    recognize(none_of("\\'\n")),
                )),
                tag("'"),
            ))),
        )(s)?;

        Ok((s, Self { rune }))
    }
}

#[derive(Debug)]
pub struct StringLit<'a> {
    strlit: Span<'a>,
}

impl<'a> Parse<'a> for StringLit<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        let (s, strlit) = context(
            "StringLit",
            ws(recognize(delimited(
                tag("\""),
                many0(alt((
                    recognize(pair(tag("\\U"), count(one_of("0123456789abcdefABCDEF"), 8))),
                    recognize(pair(tag("\\u"), count(one_of("0123456789abcdefABCDEF"), 4))),
                    recognize(pair(tag("\\x"), count(one_of("0123456789abcdefABCDEF"), 2))),
                    recognize(pair(tag("\\"), count(one_of("01234567"), 3))),
                    recognize(pair(tag("\\"), one_of("\\\"abfnrtv"))),
                    recognize(none_of("\\\"\n")),
                ))),
                tag("\""),
            ))),
        )(s)?;

        Ok((s, Self { strlit }))
    }
}

#[derive(Debug)]
pub struct RawLit<'a> {
    rawlit: Span<'a>,
}

impl<'a> Parse<'a> for RawLit<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        let (s, rawlit) = context(
            "RawLit",
            ws(recognize(delimited(
                tag("`"),
                many0(none_of("`")),
                tag("`"),
            ))),
        )(s)?;

        Ok((s, Self { rawlit }))
    }
}
