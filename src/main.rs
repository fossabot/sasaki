#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(proc_macro_derive_resolution_fallback)]

// if I will need bd
//#[macro_use] extern crate diesel;

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

pub mod types;
mod handler;
pub mod collections;
pub mod commands;
mod sasaki;

const CONF_FILE_NAME: &'static str = "conf.ini";

use ini::Ini;

fn parse_config() -> types::SasakiOptions {
  let mut options: types::SasakiOptions = types::SasakiOptions {
    verbose: true,
    discord: String::from("")
  };
  let _and_then_there_is_useless_result =
    Ini::load_from_file(CONF_FILE_NAME)
      .and_then(|conf| Ok({
        options.verbose = conf["General"]["verbose"] == "true";
        options.discord = conf["Discord"]["token"].to_owned();
      }));
  options
}

fn main() {
  let mut conf = parse_config();
  if let Err(err) = sasaki::run(&mut conf) {
    panic!("sasaiki failed with: {:?}", err)
  }
}
