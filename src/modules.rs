use crate::utils::get_current_folder;
use anyhow::Context;
use futures::future::join_all;
use rune::{
    Any, Context as Ctx, Diagnostics, Module, Source, Sources, Vm, compile,
    macros::{FormatArgs, MacroContext, TokenStream, quote},
    parse::Parser,
    runtime::VmResult,
    termcolor::{ColorChoice, StandardStream},
};
use std::{ops::DerefMut, path::PathBuf, sync::Arc};
use tokio::{sync::Mutex, task::spawn_blocking};
use tracing::instrument;

#[derive(Any, Debug)]
pub struct AppContext {}

#[instrument(skip_all)]
pub async fn load_modules() -> anyhow::Result<Arc<Mutex<AppContext>>> {
    let base = get_current_folder()
        .context("Unable to get current app folder")?
        .join("modules");

    let ctx = Arc::new(Mutex::new(AppContext {}));

    let modules = std::fs::read_dir(&base)
        .with_context(|| format!("Unable to read folder of modules: {}", base.display()))?
        .filter_map(|e| e.map(|e| e.path()).ok())
        .filter(|e| e.is_file())
        .map(|e| (ctx.clone(), e))
        .map(|e| spawn_blocking(move || load_module(e.0, e.1)))
        .collect::<Vec<_>>();

    let modules = join_all(modules)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    for module in modules {
        module?;
    }

    Ok(ctx)
}

#[instrument(skip(app_context))]
fn load_module(app_context: Arc<Mutex<AppContext>>, path: PathBuf) -> rune::support::Result<()> {
    let module = get_module()?;
    let mut context = get_default_ctx()?;
    context.install(module)?;

    let runtime = Arc::new(context.runtime()?);

    let mut sources = Sources::new();
    sources.insert(Source::from_path(&path)?)?;

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let unit = result?;
    let unit = Arc::new(unit);
    let mut vm = Vm::new(runtime, unit);

    let mut app_context = app_context.blocking_lock();
    let _output = vm.call(["register"], (app_context.deref_mut(),))?;

    tracing::debug!("Module is loaded");

    Ok(())
}

fn get_module() -> rune::support::Result<Module> {
    let mut module = Module::new();
    module.ty::<AppContext>()?;

    module.function_meta(debug_impl)?;
    module.macro_meta(debug_macro)?;

    Ok(module)
}

#[rune::macro_(path = debug)]
fn debug_macro(
    cx: &mut MacroContext<'_, '_, '_>,
    stream: &TokenStream,
) -> compile::Result<TokenStream> {
    let mut p = Parser::from_token_stream(stream, cx.input_span());
    let args = p.parse_all::<FormatArgs>()?;
    let expanded = args.expand(cx)?;
    Ok(quote!(debug(#expanded)).into_token_stream(cx)?)
}

#[rune::function(path = debug)]
fn debug_impl(message: &str) -> VmResult<()> {
    tracing::debug!(message);
    VmResult::Ok(())
}

fn get_default_ctx() -> rune::support::Result<Ctx> {
    let mut context = Ctx::new();

    context.install(rune::modules::iter::module()?)?;
    context.install(rune::modules::core::module()?)?;
    context.install(rune::modules::cmp::module()?)?;
    context.install(rune::modules::any::module()?)?;
    context.install(rune::modules::clone::module()?)?;
    context.install(rune::modules::num::module()?)?;
    context.install(rune::modules::hash::module()?)?;

    context.install(rune::modules::string::module()?)?;
    context.install(rune::modules::bytes::module()?)?;

    context.install(rune::modules::collections::module()?)?;
    context.install(rune::modules::char::module()?)?;
    context.install(rune::modules::f64::module()?)?;
    context.install(rune::modules::tuple::module()?)?;
    context.install(rune::modules::fmt::module()?)?;
    context.install(rune::modules::future::module()?)?;
    context.install(rune::modules::i64::module()?)?;
    context.install(rune::modules::u64::module()?)?;
    context.install(rune::modules::macros::module()?)?;
    context.install(rune::modules::macros::builtin::module()?)?;
    context.install(rune::modules::mem::module()?)?;
    context.install(rune::modules::object::module()?)?;
    context.install(rune::modules::ops::module()?)?;
    context.install(rune::modules::ops::generator::module()?)?;
    context.install(rune::modules::option::module()?)?;
    context.install(rune::modules::result::module()?)?;
    context.install(rune::modules::stream::module()?)?;
    context.install(rune::modules::test::module()?)?;
    context.install(rune::modules::vec::module()?)?;
    context.install(rune::modules::slice::module()?)?;

    Ok(context)
}
