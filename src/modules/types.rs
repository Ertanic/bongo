use rune::Any;

#[derive(Any)]
pub enum WebResult {
    #[rune(constructor)]
    Body(#[rune(get)] String),
    None,
}
