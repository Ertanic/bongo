use rune::compile;
use rune::macros::{quote, FormatArgs, MacroContext, TokenStream};
use rune::parse::Parser;
use rune::runtime::VmResult;

#[rune::macro_(path = error)]
pub fn error_macro(
    cx: &mut MacroContext<'_, '_, '_>,
    stream: &TokenStream,
) -> compile::Result<TokenStream> {
    let mut p = Parser::from_token_stream(stream, cx.input_span());
    let args = p.parse_all::<FormatArgs>()?;
    let expanded = args.expand(cx)?;
    Ok(quote!(error(#expanded)).into_token_stream(cx)?)
}

#[rune::function(path = error)]
pub fn error_impl(message: &str) -> VmResult<()> {
    tracing::error!(message);
    VmResult::Ok(())
}

#[rune::macro_(path = warn)]
pub fn warn_macro(
    cx: &mut MacroContext<'_, '_, '_>,
    stream: &TokenStream,
) -> compile::Result<TokenStream> {
    let mut p = Parser::from_token_stream(stream, cx.input_span());
    let args = p.parse_all::<FormatArgs>()?;
    let expanded = args.expand(cx)?;
    Ok(quote!(warn(#expanded)).into_token_stream(cx)?)
}

#[rune::function(path = warn)]
pub fn warn_impl(message: &str) -> VmResult<()> {
    tracing::warn!(message);
    VmResult::Ok(())
}

#[rune::macro_(path = info)]
pub fn info_macro(
    cx: &mut MacroContext<'_, '_, '_>,
    stream: &TokenStream,
) -> compile::Result<TokenStream> {
    let mut p = Parser::from_token_stream(stream, cx.input_span());
    let args = p.parse_all::<FormatArgs>()?;
    let expanded = args.expand(cx)?;
    Ok(quote!(info(#expanded)).into_token_stream(cx)?)
}

#[rune::function(path = info)]
pub fn info_impl(message: &str) -> VmResult<()> {
    tracing::info!(message);
    VmResult::Ok(())
}

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

#[rune::macro_(path = trace)]
pub fn trace_macro(
    cx: &mut MacroContext<'_, '_, '_>,
    stream: &TokenStream,
) -> compile::Result<TokenStream> {
    let mut p = Parser::from_token_stream(stream, cx.input_span());
    let args = p.parse_all::<FormatArgs>()?;
    let expanded = args.expand(cx)?;
    Ok(quote!(trace(#expanded)).into_token_stream(cx)?)
}

#[rune::function(path = trace)]
pub fn trace_impl(message: &str) -> VmResult<()> {
    tracing::trace!(message);
    VmResult::Ok(())
}