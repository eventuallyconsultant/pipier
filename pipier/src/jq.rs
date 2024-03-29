// helping examples :
// see here : https://github.com/01mf02/jaq/blob/main/jaq-interpret/tests/common/mod.rs
// and here : https://github.com/01mf02/jaq/blob/c776647e66e3c481a505bd34e333678acb0141d8/jaq/src/main.rs#L402

use jaq_interpret::{Ctx, FilterT, RcIter, Val};
use serde_json::Value;

pub type Filter = jaq_syn::Main;

#[derive(thiserror::Error, Debug)]
pub enum JqError {
  #[error("Interpret error: {0}")]
  InterpretError(String),
  #[error("Parse error: {0:?}")]
  ParseError(String),
  #[error("Unable to parse")]
  UnableToParse,
}

impl From<Vec<jaq_parse::Error>> for JqError {
  fn from(value: Vec<jaq_parse::Error>) -> Self {
    let string = value.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
    JqError::ParseError(string)
  }
}

impl From<jaq_interpret::Error> for JqError {
  fn from(value: jaq_interpret::Error) -> Self {
    JqError::InterpretError(value.to_string())
  }
}

pub fn jq(json: Value, query: &str) -> Result<Value, JqError> {
  let filter = make_jq_filter(query)?;
  let vec = jq_from_filter(json, filter)?;
  Ok(if vec.len() == 1 { vec.into_iter().next().unwrap_or_default() } else { Value::Array(vec) })
}

pub fn jq_from_filter(json: Value, filter: Filter) -> Result<Vec<Value>, JqError> {
  let mut ctx = make_default_context();
  let filter = ctx.compile(filter);

  let jq_val: Val = json.into();

  let null = Box::new(core::iter::once(Ok(Val::Null))) as Box<dyn Iterator<Item = _>>;
  let null = RcIter::new(null);
  let null_ctx = Ctx::new(vec![], &null);

  let results = filter.run((null_ctx.clone(), jq_val)).map(|x| x.map(Into::into)).collect::<Result<Vec<_>, _>>()?;

  Ok(results)
}

pub fn make_jq_filter(query: &str) -> Result<Filter, JqError> {
  let (main, errs) = jaq_parse::parse(query, jaq_parse::main());
  if !errs.is_empty() {
    return Err(errs.into());
  }
  let Some(main) = main else {
    return Err(JqError::UnableToParse);
  };
  Ok(main)
}

fn make_default_context() -> jaq_interpret::ParseCtx {
  let mut ctx = jaq_interpret::ParseCtx::new(Vec::new());
  ctx.insert_natives(jaq_core::core());
  ctx.insert_defs(jaq_std::std());
  ctx
}
