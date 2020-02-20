extern crate neon;
extern crate neon_serde;
extern crate rustling_ontology;
#[macro_use]
extern crate serde;

mod json;

use neon::prelude::*;
use rustling_ontology::*;
use json::*;

use std::str::FromStr;

pub struct RustlingParser {
  parser: Parser
}

impl RustlingParser {
  fn parse(&self, query: &str, kinds: &[OutputKind]) -> Vec<SlotValue> {
    let context = ResolverContext::default();
    let entities = {
      if kinds.len() > 0 {
          self.parser.parse_with_kind_order(&*query, &context, &kinds).unwrap()
      } else {
          self.parser.parse(&*query, &context).unwrap()
      }
    };

    entities
      .iter()
      .map(|entity| SlotValue::from(entity.value.clone()))
      .collect::<Vec<_>>()
  }
}

declare_types! {
  pub class JsRustlingParser for RustlingParser {
    init(mut cx) {
      let parser_lang = cx.argument::<JsString>(0)?.value();
      let language = Lang::from_str(&parser_lang).unwrap();
      let _parser = match build_parser(language) {
        Ok(_parser) => _parser,
        Err(_e) => panic!(format!("{}", _e)),
      };
      Ok(RustlingParser {
        parser: _parser
      })
    }

    method parse(mut cx) {
      let query = cx.argument::<JsString>(0)?.value();

      // Optional ordered list of `OutputKind` to parse in `query`
      let kinds = match cx.argument_opt(1) {
        Some(args) => args.downcast::<JsArray>()
          .or_throw(&mut cx)?
          .to_vec(&mut cx)
          .map(|values| {
             values
                .iter()
                .map(|s| {
                  OutputKind::from_str(
                    &s.downcast::<JsString>()
                      .unwrap()
                      .value()
                  )
                  .unwrap()
                })
                .collect::<Vec<_>>()
          })
          .unwrap(),
        None => vec![],
      };

      let this = cx.this();
      let entities_array = {
        let guard = cx.lock();
        let instance = this.borrow(&guard);
        instance.parse(&*query, &kinds)
      };

      let result = neon_serde::to_value(&mut cx, &entities_array)?;

      Ok(result)
    }
  }
}


register_module!(mut m, {
  m.export_class::<JsRustlingParser>("Parser")?;
  Ok(())
});