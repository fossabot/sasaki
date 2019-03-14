use types::SasakiOptions;
use handler::Handler;
use commands;

use argparse::{ArgumentParser, StoreTrue};
use argparse::action::{IFlagAction, ParseResult};

use env_logger::Env;

use serenity::prelude::Mutex;
use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::{
  framework::StandardFramework,
  http
};

use typemap::Key;
use std::sync::Arc;
use std::collections::HashSet;

struct VoiceManager;

impl Key for VoiceManager {
  type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct Version();

impl IFlagAction for Version {
  fn parse_flag(&self) -> ParseResult {
    set!( version = env!("CARGO_PKG_VERSION").to_string()
        , pname = "Sasaki"
        , version_string = format!("{} {}", pname, version) );
    println!("{}", version_string);
    return ParseResult::Exit;
  }
}

pub fn run(opts : &mut SasakiOptions) -> Result<(), serenity::Error> {
  { // this block limits scope of borrows by ap.refer() method
    let mut ap = ArgumentParser::new();
    let pname = "Sasaki";
    ap.set_description(pname);
    ap.refer(&mut opts.verbose)
      .add_option(&["-V", "--verbose"], StoreTrue,
      "Verbose output");
    ap.add_option(&["--version"], Version(), "Show version");
    ap.parse_args_or_exit();
  }

  let env = Env::default()
    .filter_or("MY_LOG_LEVEL", "info") // trace
    .write_style_or("MY_LOG_STYLE", "always");

  env_logger::init_from_env(env);

  if opts.verbose {
    info!("Sasaki {} I'm waking up", env!("CARGO_PKG_VERSION").to_string());
  }

  let mut client = serenity::Client::new
    (&opts.discord, Handler).expect("Error creating serenity client");

  {
    let mut data = client.data.lock();
    data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
  }

  let owners = match http::get_current_application_info() {
    Ok(info) => {
      let mut set = HashSet::new();
      set.insert(info.owner.id);
      set
    },
    Err(why) => panic!("Couldn't get application info: {:?}", why),
  };

  client.with_framework(StandardFramework::new()
    .configure(|c| c
      .owners(owners)
      .on_mention(true)
      .prefix("*"))
    .command("ping", |c| c
      .cmd(commands::meta::ping)
      .owners_only(true))
    .command("quit", |c| c
      .cmd(commands::owner::quit)
      .owners_only(true))
    .command("partners", |c| c
      .cmd(commands::meta::partners)
      .allowed_roles(vec!["wheel"]))
    );

  client.start()
}

#[cfg(test)]
mod tests {
  // TODO: write some tests
}
