use rune::compile;
use rune::macros::{quote, FormatArgs, MacroContext, TokenStream};
use rune::parse::Parser;
use rune::runtime::VmResult;

#[rune::macro_(path = debug)]
pub fn debug_macro(
    cx: &mut MacroContext<'_, '_, '_>,
    stream: &TokenStream,
) -> compile::Result<TokenStream> {
    let mut p = Parser::from_token_stream(stream, cx.input_span());
    let args = p.parse_all::<FormatArgs>()?;
    let expanded = args.expand(cx)?;
    Ok(quote!(debug(#expanded)).into_token_stream(cx)?)
}

#[rune::function(path = debug)]
pub fn debug_impl(message: &str) -> VmResult<()> {
    tracing::debug!(message);
    VmResult::Ok(())
}