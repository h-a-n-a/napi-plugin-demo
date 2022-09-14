use napi::bindgen_prelude::*;
use napi_derive::napi;

use core_lib::{Plugin, Val};

#[napi(ts_args_type = "values: ExternalObject<any>[]")]
pub fn folder(values: Vec<External<Box<dyn Plugin>>>) -> u32 {
  values
    .iter()
    .fold(Val(0), |prev, plugin| plugin.run(prev))
    .0
}
