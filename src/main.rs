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
extern crate serde_json;
extern crate ssh2;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate serenity;
#[macro_use] pub mod macros;

/*#[macro_use]*/ extern crate diesel_codegen;
#[macro_use] extern crate diesel;

extern crate curl;

pub mod data;
pub mod common;
pub mod conf;
pub mod types;
pub mod db;
pub mod collections;
pub mod commands;
mod handler;
mod sasaki;

fn main() {
  let mut conf = conf::parse_config();
  if let Err(err) = sasaki::run(&mut conf) {
    panic!("sasaiki failed with: {:?}", err)
  }
}
