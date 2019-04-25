use crate::{
  common::{
    msg::{ channel_message, reply },
    log::{ log }
  },
  commands::voice,
  data::{ DATA, DataField, SHELL_MODE, SSH_MODE, SSH_SESSION },
  db,
  conf,
  collections::{
    overwatch::{ OVERWATCH, OVERWATCH_REPLIES },
    highlighting::HIGHLIGHTING
  }
};

use std::sync::atomic::{ Ordering };
use std::io::prelude::*;

use serenity::{
  model::{ event::ResumedEvent, gateway::Ready, guild::Member
         , channel::Message, channel::Reaction
         , id::GuildId
         , event::MessageUpdateEvent },
  prelude::*,
};

use rand::{
  Rng,
  thread_rng,
  seq::SliceRandom
};

use regex::Regex;

pub struct Handler;

impl EventHandler for Handler {
  fn ready(&self, ctx : Context, ready : Ready) {
    info!("Connected as {}", ready.user.name);
    voice::rejoin_voice_channel(&ctx);
  }
  fn resume(&self, _ : Context, _ : ResumedEvent) {
    info!("Resumed");
  }
  fn reaction_add(&self, _ctx: Context, add_reaction: Reaction) {
    let conf = conf::parse_config();
    if let Ok(roles_msg1) = conf.roles_msg1.parse::<u64>() {
      if roles_msg1 != 0 {
        if add_reaction.message_id == roles_msg1 {
          if let Ok(msg) = add_reaction.message() {
            if let Some(channel) = msg.channel() {
              let g = channel.guild().unwrap();
              let guild_id = g.read().guild_id;
              if let Ok(guild) = guild_id.to_partial_guild() {
                if let Some(role) = guild.role_by_name("gay") {
                  if let Ok(mut member) = guild.member(add_reaction.user_id) {
                    if let Err(why) = member.add_role(role) {
                      error!("Failed to assign gay role {:?}", why);
                    } else {
                      let user_i64 = add_reaction.user_id.as_u64().clone() as i64;
                      let guild_i64 = guild_id.as_u64().clone() as i64;
                      let mut roles_vector : Vec<i64> = Vec::new();
                      for role in &member.roles {
                        roles_vector.push(
                          role.as_u64().clone() as i64);
                      }
                      db::update_member(user_i64, guild_i64, &roles_vector);
                      log(&guild_id, &format!("{} is gay now", member));
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  fn reaction_remove(&self, _ctx: Context, add_reaction: Reaction) {
    let conf = conf::parse_config();
    if let Ok(roles_msg1) = conf.roles_msg1.parse::<u64>() {
      if roles_msg1 != 0 {
        if add_reaction.message_id == roles_msg1 {
          if let Ok(msg) = add_reaction.message() {
            if let Some(channel) = msg.channel() {
              let g = channel.guild().unwrap();
              let guild_id = g.read().guild_id;
              if let Ok(guild) = guild_id.to_partial_guild() {
                if let Some(role) = guild.role_by_name("gay") {
                  if let Ok(mut member) = guild.member(add_reaction.user_id) {
                    if let Err(why) = member.remove_role(role) {
                      error!("Failed to assign gay role {:?}", why);
                    } else {
                      let user_i64 = add_reaction.user_id.as_u64().clone() as i64;
                      let guild_i64 = guild_id.as_u64().clone() as i64;
                      let mut roles_vector : Vec<i64> = Vec::new();
                      for role in &member.roles {
                        roles_vector.push(
                          role.as_u64().clone() as i64);
                      }
                      db::update_member(user_i64, guild_i64, &roles_vector);
                      log(&guild_id, &format!("{} is not gay anymore", member));
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  fn guild_member_addition(&self, _: Context, guild_id: GuildId, mut member: Member) {
    use serenity::CACHE;
    let cache = CACHE.read();
    if let Some(guild) = cache.guild(guild_id) {
      let guild = guild.read();
      let roles = db::reset_roles(member.user_id(), guild_id);
      for role in roles {
        if let Err(why) = member.add_role(role) {
          error!("Failed to reset role for user {:?}", why);
        }
      }
      if let Ok(channels) = guild.channels() {
        let log_channel = channels.iter().find(|&(c, _)|
          if let Some(name) = c.name() {
            name == "log"
          } else {
            false
          });
        if let Some((_, channel)) = log_channel {
          let user = member.user.read();
          if let Err(why) = channel.send_message(|m| m
            .embed(|e| {
              let mut e = e
                .author(|a| a.icon_url(&user.face()).name(&user.name))
                .title("has joined!");
              if let Some(ref joined_at) = member.joined_at {
                e = e.timestamp(joined_at);
              } e
          })) {
            error!("Failed to log new user {:?}", why);
          }
        }
      }
    }
  }
  fn message_update(&self, _ctx: Context, new_data: MessageUpdateEvent) {
    // they do it too!
    if let Some(author) = new_data.author {
      if author.bot {
        return;
      }
    }
    // wait for new serenity release
    /* that was just a test!
    channel_message("n o  e d i t i n g");
    */
  }
  fn message(&self, _ : Context, mut msg : Message) {
    if msg.is_own() {
      if msg.content.to_lowercase() == "pong" {
        if let Err(why) = msg.edit(|m| m.content("🅱enis!")) {
          error!("Failed to Benis {:?}", why);
        }
      }
      return;
    } else if msg.author.bot {
      // only on own server
      if let Some(guild_id) = msg.guild_id {
        let conf = conf::parse_config();
        if let Ok(guild_u64) = conf.guild.parse::<u64>() {
          if &guild_u64 == guild_id.as_u64() {
            // 1 of 3 will be replaced
            let rnd = rand::thread_rng().gen_range(0, 3);
            if rnd == 1 || msg.content == "pong" {
              if let Err(why) = msg.delete() {
                error!("Error deleting ekks {:?}", why);
              }
              channel_message(&msg, msg.content.as_str());
            }
          }
        }
      }
    } else {
      if SHELL_MODE.load(Ordering::Relaxed) && msg.content.starts_with("~") {
        if let Ok(data) = DATA.lock() {
          if let Some(owner) = data.get(&DataField::Owner) {
            let conf = conf::parse_config();
            if let Ok(owner_u64) = conf.owner.parse::<u64>() {
              let cmd = &msg.content[1..];
              let syntax = if cmd.starts_with("cat ") {
                if let Some((_, found_syntax)) = HIGHLIGHTING.into_iter().find(|(r, _)| {
                  match Regex::new(*r) {
                    Ok(is_an_syntax) => {
                      is_an_syntax.is_match(cmd) },
                    Err(error) => {
                      error!("wrong regex scheme! {} {:?}", r, error);
                      false
                    }
                  }
                }) { *found_syntax } else {""}
                } else if cmd.starts_with("git diff") { "diff"
                } else {""};
              if msg.author.id.as_u64() == owner && &owner_u64 == owner {
                if msg.is_private() {
                  let cmd = &msg.content[1..];
                  let (_code, stdout, _stderr) = bash!("{}", cmd);
                  let formatted_out = format!("```{}\n{}\n```\n", syntax, stdout);
                  if let Err(why) = msg.author.dm(|m| m.content(formatted_out)) {
                    error!("Error sending dm: {:?}", why);
                  }
                } else if let Some(guild_id) = msg.guild_id {
                  if let Ok(guild_u64) = conf.guild.parse::<u64>() {
                    if &guild_u64 == guild_id.as_u64() {
                      let (_code, stdout, stderr) = bash!("{}", cmd);
                      let formatted_out = format!("```{}\n{}\n```\n", syntax, stdout);
                      channel_message(&msg, formatted_out.as_str());
                      if !stderr.is_empty() {
                        let formatted_err = format!("```error\n{}\n```\n", stderr);
                        channel_message(&msg, formatted_err.as_str());
                      }
                    }
                  }
                }
              } else {
                reply(&msg, "NO SHELL MODE ACCESS!");
              }
            }
          }
        }
      } else if SSH_MODE.load(Ordering::Relaxed) && msg.content.starts_with("~") {
        if let Ok(data) = DATA.lock() {
          if let Some(owner) = data.get(&DataField::Owner) {
            let conf = conf::parse_config();
            if let Ok(owner_u64) = conf.owner.parse::<u64>() {
              if msg.author.id.as_u64() == owner && &owner_u64 == owner {
                if msg.is_private() {
                  if let Ok(ifsess) = SSH_SESSION.lock() {
                    if let Some(ref sess) = *ifsess {
                      let cmd = &msg.content[1..];
                      match sess.channel_session() {
                        Ok(mut channel) => {
                          channel.exec(cmd).unwrap();
                          let mut s = String::new();
                          channel.read_to_string(&mut s).unwrap();
                          let formatted_out = format!("```\n{}\n```\n", s);
                          if let Err(why) = msg.author.dm(|m| m.content(formatted_out)) {
                            error!("Error sending dm: {:?}", why);
                          }
                          let _ = channel.wait_close();
                        },
                        Err(err) => {
                          if let Err(why) = msg.author.dm(|m| m.content(err)) {
                            error!("Error sending dm: {:?}", why);
                          }
                        }
                      };
                    }
                  }
                }
              }
            }
          }
        }
      } else if let Some(find_char_in_words) = OVERWATCH.into_iter().find(|&c| {
        let regex = format!(r"(^|\W)((?i){}(?-i))($|\W)", c);
        let is_overwatch = Regex::new(regex.as_str()).unwrap();
        is_overwatch.is_match(msg.content.as_str()) }) {
        let mut rng = thread_rng();
        set! { ov_reply = OVERWATCH_REPLIES.choose(&mut rng).unwrap()
             , reply = format!("{} {}", ov_reply, find_char_in_words) };
        if let Err(why) = msg.channel_id.say(reply) {
          error!("Error sending overwatch reply: {:?}", why);
        }
      } else {
        let regex_no_u = Regex::new(r"(^|\W)((?i)no u(?-i))($|\W)").unwrap();
        if regex_no_u.is_match(msg.content.as_str()) {
          let rnd = rand::thread_rng().gen_range(0, 2);
          if rnd == 1 {
            if let Err(why) = msg.channel_id.say("No u") {
              error!("Error sending no u reply: {:?}", why);
            }
          }
        }
      }
    }
  }
}
