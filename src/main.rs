#![feature(adt_const_params)]
#![feature(associated_type_defaults)]

use miette::{Context, IntoDiagnostic, Result};
mod lexer;

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

fn main() -> Result<()> {
    let name = "examples/example.go";
    let source = std::fs::read_to_string(name)
        .into_diagnostic()
        .context("could not read file contents")?;

    let token_tree = match lexer::parse(&source) {
        Ok(token_tree) => token_tree,
        Err(err) => {
            panic!("{:#?}", err)
        }
    };
    println!("{:#?}", token_tree);

    // Err(MyBad {
    //     src: NamedSource::new(name, source),
    //     snip1: SourceSpan::new(5.into(), 4.into()),
    //     snip2: SourceSpan::new(12.into(), 4.into()),
    // })?

    Ok(())
}
