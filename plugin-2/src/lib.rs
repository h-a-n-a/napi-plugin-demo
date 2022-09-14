use napi::bindgen_prelude::*;
use napi_derive::napi;

use core_lib::Val;

pub fn create_external<T>(value: T) -> External<T> {
  External::new(value)
}

#[napi(ts_return_type = "ExternalObject<any>")]
pub fn create_val() -> External<Val> {
  create_external(Val(2))
}
