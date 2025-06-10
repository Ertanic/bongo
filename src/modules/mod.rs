mod macros;
mod utils;

use self::macros::*;
use crate::modules::utils::get_default_ctx;
use crate::utils::get_current_folder;
use anyhow::Context;
use axum::Router;
use futures::future::join_all;
use rune::{
    Any, Diagnostics, Module, Source, Sources, Vm, compile,
    macros::{FormatArgs, MacroContext, TokenStream, quote},
    parse::Parser,
    runtime::VmResult,
    termcolor::{ColorChoice, StandardStream},
};
use std::{ops::Deref, path::PathBuf, sync::Arc};
use tokio::{sync::Mutex, task::spawn_blocking};
use tracing::instrument;

#[derive(Any, Debug)]
pub struct AppContext {
    pub routes: Arc<Mutex<Vec<Router>>>,
}

#[instrument(skip_all)]
pub async fn load_modules() -> anyhow::Result<Arc<AppContext>> {
    let base = get_current_folder()
        .context("Unable to get current app folder")?
        .join("modules");

    let ctx = Arc::new(AppContext {
        routes: Arc::new(Mutex::new(Vec::new())),
    });

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
fn load_module(app_context: Arc<AppContext>, path: PathBuf) -> rune::support::Result<()> {
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

    let _output = vm.call(["register"], (app_context.deref(),))?;

    tracing::debug!("Module is loaded");

    Ok(())
}

fn get_module() -> rune::support::Result<Module> {
    let mut module = Module::new();
    module.ty::<AppContext>()?;

    module.function_meta(error_impl)?;
    module.macro_meta(error_macro)?;
    module.function_meta(warn_impl)?;
    module.macro_meta(warn_macro)?;
    module.function_meta(info_impl)?;
    module.macro_meta(info_macro)?;
    module.function_meta(debug_impl)?;
    module.macro_meta(debug_macro)?;
    module.function_meta(trace_impl)?;
    module.macro_meta(trace_macro)?;

    Ok(module)
}

#[derive(Any)]
pub struct Route(Router);
