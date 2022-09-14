#![deny(clippy::all)]

pub struct Val(pub u32);

pub trait Plugin {
  fn run(&self, val: Val) -> Val;
}
