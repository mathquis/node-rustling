#[macro_use]
extern crate serde;

use neon::prelude::*;
use rustling_ontology::*;
use json::*;
use std::str::FromStr;

mod json;

pub struct RustlingParser {
  parser: Parser
}

impl Finalize for RustlingParser {}

fn create_parser(mut cx: FunctionContext) -> JsResult<JsBox<RustlingParser>> {
  let parser_lang = cx.argument::<JsString>(0)?.value(&mut cx);
  let language = Lang::from_str(&parser_lang).unwrap();
  let _parser = match build_parser(language) {
    Ok(_parser) => _parser,
    Err(_e) => panic!("{}", _e),
  };
  Ok(cx.boxed(RustlingParser {
    parser: _parser
  }))
}

fn parse(mut cx: FunctionContext) -> JsResult<JsValue> {
  let rustling = cx.argument::<JsBox<RustlingParser>>(0)?;
  let query = cx.argument::<JsString>(1)?.value(&mut cx);

  // Optional ordered list of `OutputKind` to parse in `query`
  let kinds = match cx.argument_opt(2) {
    Some(args) => args.downcast::<JsArray, _>(&mut cx)
      .or_throw(&mut cx)?
      .to_vec(&mut cx)
      .map(|values| {
         values
            .iter()
            .map(|s| {
              OutputKind::from_str(
                &s.downcast::<JsString, _>(&mut cx)
                  .unwrap()
                  .value(&mut cx)
              )
              .unwrap()
            })
            .collect::<Vec<_>>()
      })
      .unwrap(),
    None => vec![],
  };

  let context = ResolverContext::default();
  let entities = {
    if kinds.len() > 0 {
        rustling.parser.parse_with_kind_order(&*query, &context, &kinds).unwrap()
    } else {
        rustling.parser.parse(&*query, &context).unwrap()
    }
  };

  let entities_array = entities
    .iter()
    .map(|entity| SlotValue::from(entity.value.clone()))
    .collect::<Vec<_>>();

  let result = neon_serde::to_value(&mut cx, &entities_array)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

  Ok(result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("createParser", create_parser)?;
  cx.export_function("parse", parse)?;
  Ok(())
}