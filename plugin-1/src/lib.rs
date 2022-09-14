use napi::bindgen_prelude::*;
use napi_derive::napi;

use core_lib::{Plugin, Val};

pub fn create_external<T>(value: T) -> External<T> {
  External::new(value)
}

struct Plugin1 {}

impl Plugin for Plugin1 {
  fn run(&self, mut val: Val) -> Val {
    val.0 += 1;
    val
  }
}

#[napi(ts_return_type = "ExternalObject<any>")]
pub fn create_plugin() -> External<Box<dyn Plugin>> {
  create_external(Box::new(Plugin1 {}))
}
