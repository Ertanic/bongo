use rune::Context;

pub fn get_default_ctx() -> rune::support::Result<Context> {
    let mut context = Context::new();

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