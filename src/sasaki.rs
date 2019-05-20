use crate::{
  types::SasakiOptions,
  handler::Handler,
  data::{ DATA, DataField },
  commands,
  commands::voice::VoiceManager,
  commands::meta::ShardManagerContainer
};

use argparse::{
  { ArgumentParser, StoreTrue },
  action::{IFlagAction, ParseResult}
};

use env_logger::Env;

use serenity::{
  framework::StandardFramework,
  http
};

use std::collections::HashSet;
use std::sync::Arc;

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
    data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
  }

  let owners = match http::get_current_application_info() {
    Ok(info) => {
      let mut set = HashSet::new();
      DATA.lock().unwrap().insert
        (DataField::Owner, info.owner.id.as_u64().clone());
      set.insert(info.owner.id);
      set
    },
    Err(why) => panic!("Couldn't get application info: {:?}", why),
  };

  client.with_framework(StandardFramework::new()
    .configure(|c| c
      .owners(owners)
      .on_mention(true)
      .prefix("`")
      .case_insensitivity(true))

    .cmd("help", commands::meta::help)
    .cmd("ping", commands::meta::ping)

    .group("voice commands", |g| g
      .cmd("join", commands::voice::join)
      .cmd("leave", commands::voice::leave)
      .cmd("play", commands::voice::play))
    .group("public", |g| g
      .cmd("todo", commands::cockroach::todo))

    .command("roles", |c| c
      .cmd(commands::owner::roles)
      .owners_only(true))
    .command("shell", |c| c
      .cmd(commands::owner::shell)
      .owners_only(true))
    .command("ssh", |c| c
      .cmd(commands::owner::ssh)
      .owners_only(true))
    .command("quit", |c| c
      .cmd(commands::owner::quit)
      .owners_only(true))
    .command("clear_channel", |c| c
      .cmd(commands::owner::clear)
      .owners_only(true))
    .command("partners", |c| c
      .cmd(commands::meta::partners)
      .owners_only(true))

    .command("lookup", |c| c
      .cmd(commands::cockroach::lookup)
      .owners_only(true)
      .allowed_roles(vec!["wheel"]))
    .command("register", |c| c
      .cmd(commands::cockroach::register)
      .owners_only(true))
    );

  client.start()
}

#[cfg(test)]
mod tests {
  // TODO: write some tests
}
