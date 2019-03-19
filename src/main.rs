#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate chrono;
extern crate typemap;
extern crate argparse;
extern crate ini;
extern crate env_logger;
extern crate rand;
extern crate regex;

#[macro_use] extern crate log;
#[macro_use] extern crate serenity;
#[macro_use] pub mod macros;

extern crate curl;

pub mod conf;
pub mod types;
mod handler;
pub mod collections;
pub mod commands;
mod sasaki;

fn main() {
  let mut conf = conf::parse_config();
  if let Err(err) = sasaki::run(&mut conf) {
    panic!("sasaiki failed with: {:?}", err)
  }
}
