use nom::{IResult, error::ParseError};
use nom_locate::LocatedSpan;
use nom_supreme::{error::ErrorTree, final_parser::final_parser};

use self::{extra::trailing_ws, token::TokenTree};

pub mod extra;
pub mod numlit;
pub mod strlit;
pub mod token;

pub type Span<'a> = LocatedSpan<&'a str>;

pub trait Parse<'a>: Sized {
    type Error: ParseError<Span<'a>> = ErrorTree<Span<'a>>;
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error>;
}

pub fn parse(input: &str) -> Result<TokenTree<'_>, ErrorTree<Span<'_>>> {
    let s = Span::new(input);
    final_parser(trailing_ws(TokenTree::parse))(s)
}
