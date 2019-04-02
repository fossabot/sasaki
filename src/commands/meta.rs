use rand::Rng;
use curl::easy::Easy;
use std::str;
use regex::Regex;
use std::sync::Arc;
use typemap::Key;

use serde_json::{Value};

use serenity::{
  client::bridge::gateway::{ShardId, ShardManager},
  prelude::*,
};

pub struct ShardManagerContainer;

impl Key for ShardManagerContainer {
  type Value = Arc<Mutex<ShardManager>>;
}

command!(help(_ctx, msg) {
  let version = format!("Sasaki {}", env!("CARGO_PKG_VERSION").to_string());
  if let Err(why) = msg.channel_id.send_message(|m| m
    .embed(|e| e
      .title("Sasaki")
      .url("https://github.com/cynede/sasaki")
      .image("https://69.media.tumblr.com/3b44eefaf1c3d4eb236a2ea1f7cf3fb2/tumblr_mjxxdijsyZ1reurhko1_540.png")
      .thumbnail("https://i.pinimg.com/originals/6d/0a/c9/6d0ac96e60b45dc429b224909086f993.jpg")
      .description("佐々木 優太")
      .fields(vec![
        ("Age", "15", true),
        ("Birthdate", "December 23", true)
        ])
      .fields(vec![
        ("Height", "152 cm", true),
        ("Version", version.as_str(), true)
        ])
      .field("user commands", "todo: read todo list
todo rm <number>: removes an row from todo list by number
todo <text>: add <text> to todo list", false)
      .field("music commands", "join: to music channel
leave: from music channel
play <url>: play an radio stream or youtube music", false)
      .field("wheel commands", "register @user: register an user or many users into CockroachDB cluster
partners: display formatted partners info", false)
      .footer(|f| f.text("proficient in martial arts, extremely cruel"))
      .colour((246, 111, 0)))) {
    error!("Error sending help message: {:?}", why);
  }
});

command!(ping(ctx, msg, _args) {
  let data = ctx.data.lock();
  let shard_manager = match data.get::<ShardManagerContainer>() {
    Some(v) => v,
    None => {
      if let Err(why) = msg.author.dm(|m| m.content("There was a problem getting the shard manager")) {
        error!("Error DMing user: {:?}", why);
      }
      return Ok(());
    },
  };
  set!( manager = shard_manager.lock()
      , runners = manager.runners.lock() );
  let runner = match runners.get(&ShardId(ctx.shard_id)) {
    Some(runner) => runner,
    None => {
      if let Err(why) = msg.author.dm(|m| m.content("No shard found")) {
        error!("Error DMing user: {:?}", why);
      }
      return Ok(());
    },
  };
  if let Err(why) = msg.reply(&format!("The shard ping is {:?}", runner.latency)) {
    error!("Error posting ping: {:?}", why);
  }
});

//TODO: rewrite using https://docs.rs/serenity/0.5.13/serenity/http/raw/index.html
command!(partners(_ctx, msg) {
  let lines : Vec<&str> = msg.content.lines().collect();
  if let Err(why) = msg.delete() {
    error!("Error deleting partners table {:?}", why);
  }
  for line in lines {
    let split : Vec<&str> = line.split('|').collect();
    if split.len() > 1 {
      set! { partner_description  = split[0]
           , partner_invite       = split[1] };

      let mut easy = Easy::new();
      let app_invite = String::from("https://discordapp.com/invite/") + partner_invite;
      easy.url(app_invite.as_str()).expect("Failed to curl the invite");
      let mut dst = Vec::<u8>::new();
      {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
          dst.extend_from_slice(data);
          Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
      }
      let invite_content = str::from_utf8(&dst).unwrap();

      let invite_link = String::from("https://discord.gg/") + partner_invite;

      set! { mc_regex = Regex::new(r#"\| (.*) members"#).unwrap()
           , mc_caps = mc_regex.captures(invite_content) };
      let members =
        match mc_caps {
          Some(x) => if x.len() > 0 { x.get(1).map_or("", |m| m.as_str()) } else { "" },
          None    => {
            set! { mc_regex2 = Regex::new(r#"with (.*) other members"#).unwrap()
                 , mc_caps2 = mc_regex2.captures(invite_content) };
            match mc_caps2 {
              Some(xx) => if xx.len() > 0 { xx.get(1).map_or("", |m| m.as_str()) } else { "" },
              None    => {
                "-"
              }
           }
          }
        };

      set! { red    = rand::thread_rng().gen_range(0, 255)
           , green  = rand::thread_rng().gen_range(0, 255)
           , blue   = rand::thread_rng().gen_range(0, 255) };

      let mut easy_v7 = Easy::new();
      let app_invite_v7 = String::from("https://discordapp.com/api/v7/invites/") + partner_invite;
      easy_v7.url(app_invite_v7.as_str()).expect("Failed to curl the invite v7");
      let mut dst_v7 = Vec::<u8>::new();
      {
        let mut transfer = easy_v7.transfer();
        transfer.write_function(|data| {
          dst_v7.extend_from_slice(data);
          Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
      }
      let invite_content_v7 = str::from_utf8(&dst_v7).unwrap();
      let v : Value = serde_json::from_str(invite_content_v7).unwrap();

      let inviter_val = &v["inviter"];
      let inviter : Value = serde_json::from_value(inviter_val.clone()).unwrap();

      let guild_val = &v["guild"];
      let guild : Value = serde_json::from_value(guild_val.clone()).unwrap();

      let username_val = &inviter["username"];
      let username = username_val.as_str().unwrap_or("-");

      let user_id = &inviter["id"].as_str().unwrap_or("");
      let avatar_id = &inviter["avatar"].as_str().unwrap_or("");
      let avatar_link = format!("https://cdn.discordapp.com/avatars/{}/{}.png", user_id, avatar_id);

      let title = &guild["name"].as_str().unwrap_or("-");

      let guild_id = &guild["id"].as_str().unwrap_or("");
      let icon_hash = &guild["icon"].as_str().unwrap_or("");
      let guild_icon = format!("https://cdn.discordapp.com/icons/{}/{}.png", guild_id, icon_hash);

      if let Err(why) = if split.len() > 2 {
        let image = split[2];
        msg.channel_id.send_message(|m| m
        .embed(|e| e
          .author(|a| {a.name(username).icon_url(avatar_link.as_str())})
          .title(title)
          .thumbnail(guild_icon)
          .image(image)
          .url(invite_link.as_str())
          .description(partner_description)
          .fields(vec![
            ("Members", members, true),
            ("Invite link", invite_link.as_str(), true)
            ])
          .colour((red, green, blue)))) } else {
            msg.channel_id.send_message(|m| m
            .embed(|e| e
              .author(|a| {a.name(username).icon_url(avatar_link.as_str())})
              .title(title)
              .thumbnail(guild_icon)
              .url(invite_link.as_str())
              .description(partner_description)
              .fields(vec![
                ("Members", members, true),
                ("Invite link", invite_link.as_str(), true)
                ])
              .colour((red, green, blue))))
          } {
        error!("Error posting partner: {:?}", why);
      }
    }
  }
});
