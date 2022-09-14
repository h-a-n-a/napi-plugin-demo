use napi::bindgen_prelude::*;
use napi_derive::napi;

use core_lib::Val;

#[napi(ts_args_type = "values: ExternalObject<any>[]")]
pub fn folder(values: Vec<External<Val>>) -> u32 {
  values.iter().fold(0, |mut prev, val| {
    prev += val.0;
    prev
  })
}
