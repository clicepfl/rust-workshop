#![allow(dead_code, unused_variables, unreachable_code, clippy::all)]

fn main() {
  // Explicit panic
  panic!("crash and burn");

  // Panic on out of bounds access
  let v = vec![1, 2, 3];
  v[99];

  // cargo run --example 50_panic
  // RUST_BACKTRACE=1 cargo run --example 50_panic
}