use nom::{IResult, Parser, branch::alt, character::complete::{alpha1, alphanumeric1, multispace0, one_of}, combinator::recognize, error::ParseError, multi::many0, sequence::{pair, preceded, terminated}};
use nom_locate::position;
use nom_supreme::{error::ErrorTree, tag::complete::tag};

use crate::{Parse, Span};

pub fn ws<'a, F: 'a, O, E: ParseError<Span<'a>>>(
    inner: F,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
where
    F: Parser<Span<'a>, O, E>,
{
    preceded(multispace0, inner)
}

pub fn trailing_ws<'a, F: 'a, O, E: ParseError<Span<'a>>>(
    inner: F,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
where
    F: Parser<Span<'a>, O, E>,
{
    terminated(inner, multispace0)
}

#[derive(Debug)]
pub struct Ident<'a> {
    ident: Span<'a>,
}

impl<'a> Parse<'a> for Ident<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        let (s, ident) = ws(recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )))(s)?;

        Ok((s, Self { ident }))
    }
}

#[derive(Debug)]
pub struct Punct<'a> {
    pos: Span<'a>,
    punct: char,
}

impl<'a> Parse<'a> for Punct<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        let (s, pos) = position(s)?;
        let (s, punct) = ws(one_of("+-*=.,!%^&|/~<>:;"))(s)?;

        Ok((s, Self { pos, punct }))
    }
}

pub fn matches<F, I, O>(mut f: F, i: &I) -> bool
where
    I: Clone,
    F: Parser<I, O, ErrorTree<I>>,
{
    f.parse(i.clone()).is_ok()
}
