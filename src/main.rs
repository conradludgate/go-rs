#![feature(adt_const_params)]
#![feature(associated_type_defaults)]

use miette::{Context, IntoDiagnostic, Result};
use nom::{
    bytes::complete::{take_while1},
    character::{
        complete::{multispace0, satisfy},
    },
    combinator::{peek},
    error::{
        context, ParseError, VerboseError,
    },
    sequence::{preceded},
    Finish, IResult, Parser,
};
use nom_locate::{position, LocatedSpan};
use nom_supreme::{error::ErrorTree, tag::{TagError, complete::tag}};

// #[derive(Error, Debug, Diagnostic)]
// #[error("oops!")]
// #[diagnostic(code(oops::my::bad), help("try doing it better next time?"))]
// struct MyBad {
//     #[source_code]
//     src: NamedSource,
//     #[label = "This is bad"]
//     snip1: SourceSpan,
//     #[label = "This is worse"]
//     snip2: SourceSpan,
// }

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: ParseError<Span<'a>>>(
    inner: F,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
where
    F: Parser<Span<'a>, O, E>,
{
    preceded(multispace0, inner)
}

type Span<'a> = LocatedSpan<&'a str>;

trait Parse<'a>: Sized {
    type Error: ParseError<Span<'a>> = ErrorTree<Span<'a>>;
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, Self::Error>;
}

#[derive(Debug)]
struct Token<'a, const TOKEN: &'static str> {
    token: Span<'a>,
}

impl<'a, const TOKEN: &'static str> Parse<'a> for Token<'a, TOKEN> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, ErrorTree<Span<'a>>> {
        let (s, token) = context(TOKEN, ws(tag(TOKEN)))(s)?;

        Ok((s, Self { token }))
    }
}

#[derive(Debug)]
struct Ident<'a> {
    ident: Span<'a>,
}

impl<'a> Parse<'a> for Ident<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, ErrorTree<Span<'a>>> {
        let (s, _) = multispace0(s)?;

        let (s, _) = context(
            "IDENT",
            peek(ws(satisfy(|c: char| c.is_alphabetic() || c == '_'))),
        )(s)?;
        let (s, ident) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(s)?;

        Ok((s, Self { ident }))
    }
}

#[derive(Debug)]
struct Package<'a> {
    position: Span<'a>,
    token: Token<'a, "package">,
    ident: Ident<'a>,
}

impl<'a> Parse<'a> for Package<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, ErrorTree<Span<'a>>> {
        let (s, position) = position(s)?;
        let (s, token) = Token::parse(s)?;
        let (s, ident) = Ident::parse(s)?;

        Ok((
            s,
            Self {
                position,
                token,
                ident,
            },
        ))
    }
}

#[derive(Debug)]
struct GoFile<'a> {
    position: Span<'a>,
    package: Package<'a>,
}

impl<'a> Parse<'a> for GoFile<'a> {
    fn parse(s: Span<'a>) -> IResult<Span<'a>, Self, ErrorTree<Span<'a>>> {
        let (s, position) = position(s)?;
        let (s, package) = Package::parse(s)?;

        Ok((s, Self { position, package }))
    }
}

fn main() -> Result<()> {
    let name = "examples/example.go";
    let source = std::fs::read_to_string(name)
        .into_diagnostic()
        .context("could not read file contents")?;

    let s = Span::new(&source);
    let go_file = match GoFile::parse(s).finish() {
        Ok((_, go_file)) => go_file,
        Err(err) => {
            panic!("{:#?}", err)
        }
    };
    println!("{:?}", go_file);

    // Err(MyBad {
    //     src: NamedSource::new(name, source),
    //     snip1: SourceSpan::new(5.into(), 4.into()),
    //     snip2: SourceSpan::new(12.into(), 4.into()),
    // })?

    Ok(())
}
