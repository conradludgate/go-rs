use nom::{
    bytes::complete::is_a,
    character::complete::{digit1, hex_digit1, oct_digit0},
    combinator::recognize,
    sequence::pair,
    IResult,
};
use nom_supreme::tag::complete::tag;

use crate::{
    extra::{matches, ws},
    Parse, Span,
};

#[derive(Debug)]
pub enum NumLit<'a> {
    Hex(HexNumLit<'a>),
    Bin(BinNumLit<'a>),
    Oct(OctNumLit<'a>),
    Dec(DecNumLit<'a>),
}

impl<'a> Parse<'a> for NumLit<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        if matches(ws(tag("0x")), &s) {
            let (s, hex) = HexNumLit::parse(s)?;
            Ok((s, Self::Hex(hex)))
        } else if matches(ws(tag("0b")), &s) {
            let (s, bin) = BinNumLit::parse(s)?;
            Ok((s, Self::Bin(bin)))
        } else if matches(ws(tag("0")), &s) {
            let (s, oct) = OctNumLit::parse(s)?;
            Ok((s, Self::Oct(oct)))
        } else {
            let (s, dec) = DecNumLit::parse(s)?;
            Ok((s, Self::Dec(dec)))
        }
    }
}

#[derive(Debug)]
pub struct HexNumLit<'a> {
    hex: Span<'a>,
}

impl<'a> Parse<'a> for HexNumLit<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        let (s, hex) = ws(recognize(pair(tag("0x"), hex_digit1)))(s)?;

        Ok((s, Self { hex }))
    }
}

#[derive(Debug)]
pub struct BinNumLit<'a> {
    bin: Span<'a>,
}

impl<'a> Parse<'a> for BinNumLit<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        let (s, bin) = ws(recognize(pair(tag("0b"), is_a("01"))))(s)?;

        Ok((s, Self { bin }))
    }
}

#[derive(Debug)]
pub struct OctNumLit<'a> {
    oct: Span<'a>,
}

impl<'a> Parse<'a> for OctNumLit<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        let (s, oct) = ws(recognize(pair(tag("0"), oct_digit0)))(s)?;

        Ok((s, Self { oct }))
    }
}

#[derive(Debug)]
pub struct DecNumLit<'a> {
    dec: Span<'a>,
}

impl<'a> Parse<'a> for DecNumLit<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error> {
        let (s, dec) = ws(digit1)(s)?;

        Ok((s, Self { dec }))
    }
}
