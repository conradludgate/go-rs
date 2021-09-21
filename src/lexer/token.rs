use nom::{IResult, character::complete::one_of, error::context, multi::many0, sequence::tuple};
use nom_supreme::tag::complete::tag;

use super::{
    extra::{matches, ws, Ident, Punct},
    numlit::NumLit,
    strlit::StrLit,
    Parse, Span,
};

#[derive(Debug)]
pub enum Token<'a> {
    Ident(Ident<'a>),
    Paren(Surrounded<'a, "(", ")">),
    Brace(Surrounded<'a, "{", "}">),
    Brack(Surrounded<'a, "[", "]">),
    Punct(Punct<'a>),
    StrLit(StrLit<'a>),
    NumLit(NumLit<'a>),
}

impl<'a> Parse<'a> for Token<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        if matches(ws(tag("(")), &s) {
            let (s, paren) = context("Token", Surrounded::parse)(s)?;
            Ok((s, Self::Paren(paren)))
        } else if matches(ws(tag("{")), &s) {
            let (s, brace) = context("Token", Surrounded::parse)(s)?;
            Ok((s, Self::Brace(brace)))
        } else if matches(ws(tag("[")), &s) {
            let (s, brack) = context("Token", Surrounded::parse)(s)?;
            Ok((s, Self::Brack(brack)))
        } else if matches(ws(one_of("0123456789")), &s) {
            let (s, numlit) = context("Token", NumLit::parse)(s)?;
            Ok((s, Self::NumLit(numlit)))
        } else if matches(ws(one_of("\"`'")), &s) {
            let (s, strlit) = context("Token", StrLit::parse)(s)?;
            Ok((s, Self::StrLit(strlit)))
        } else if matches(Punct::parse, &s) {
            let (s, punct) = context("Token", Punct::parse)(s)?;
            Ok((s, Self::Punct(punct)))
        } else {
            let (s, ident) = context("Token", Ident::parse)(s)?;
            Ok((s, Self::Ident(ident)))
        }
    }
}

#[derive(Debug)]
pub struct Surrounded<'a, const L: &'static str, const R: &'static str> {
    l: Span<'a>,
    tt: TokenTree<'a>,
    r: Span<'a>,
}

impl<'a, const L: &'static str, const R: &'static str> Parse<'a> for Surrounded<'a, L, R> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        let (s, (l, tt, r)) = context("Surrounded", tuple((ws(tag(L)), TokenTree::parse, ws(tag(R)))))(s)?;

        Ok((s, Self { l, tt, r }))
    }
}

#[derive(Debug)]
pub struct TokenTree<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> Parse<'a> for TokenTree<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        let (s, tokens) = context("TokenTree", many0(Token::parse))(s)?;

        Ok((s, Self { tokens }))
    }
}
