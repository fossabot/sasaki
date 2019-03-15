use collections::overwatch::{OVERWATCH, OVERWATCH_REPLIES};

use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use serenity::{
  model::{ event::ResumedEvent, gateway::Ready
         , channel::Message
         , event::MessageUpdateEvent },
  prelude::*,
};

use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;
use regex::Regex;

use std::sync::atomic::{AtomicBool, Ordering};

static CAGE_KREY : AtomicBool = AtomicBool::new(false);

pub struct Handler;

impl EventHandler for Handler {
  fn ready(&self, _ : Context, ready : Ready) {
    info!("Connected as {}", ready.user.name);
  }
  fn resume(&self, _ : Context, _ : ResumedEvent) {
    info!("Resumed");
  }
  fn guild_member_addition(&self, _: Context, guild_id: GuildId, mut member: Member) {
    use serenity::CACHE;
    let cache = CACHE.read();
    if let Some(guild) = cache.guild(guild_id) {
      let guild = guild.read();
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
                .title("has joined");
              if let Some(ref joined_at) = member.joined_at {
                e = e.timestamp(joined_at);
              } e
          })) {
            error!("Failed to log new user {:?}", why);
          }
        }
      }
      if member.user_id() == 476270148739661835 {
        if let Some(role) = guild.role_by_name("krey") {
          if let Err(why) = member.add_role(role) {
            error!("Failed to assign krey role to krey {:?}", why);
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
    if let Err(why) = new_data.channel_id.say("n o  e d i t i n g") {
      error!("Error sending overwatch reply: {:?}", why);
    }
  }
  fn message(&self, _ : Context, mut msg : Message) {
    if msg.is_own() {
      if msg.content == "pong" {
        if let Err(why) = msg.edit(|m| m.content("🅱enis!")) {
          error!("Failed to Benis {:?}", why);
        }
      }
      return
    }
    if msg.author.bot {
      if CAGE_KREY.load(Ordering::Relaxed) && msg.content.contains("n o   r e m o v i n g") {
        if let Err(why) = msg.delete() {
          error!("Error deleting no removing {:?}", why);
        }
      } else {
        // 1 of 3 will be replaced
        let rnd = rand::thread_rng().gen_range(0, 3);
        if rnd == 1 || msg.content == "pong" {
          if let Err(why) = msg.delete() {
            error!("Error deleting ekks {:?}", why);
          }
          if let Err(why) = msg.channel_id.say(msg.content) {
            error!("Error ekking {:?}", why);
          }
        }
      }
      return
    }
    if msg.content == "cage krey" {
      CAGE_KREY.store(true, Ordering::Relaxed);
      if let Err(msg_why) = msg.author.dm(|m| m.content(
        "Krey caged! You have cage access now too, to release use 'release krey' command")) {
        error!("Failed to cage krey: {:?}", msg_why);
      }
      if let Some(guild) = msg.guild_id {
        if let Ok(mut member) = guild.member(msg.author.id) {
          if let Ok(partial_guild) = guild.to_partial_guild() {
            if let Some(role) = partial_guild.role_by_name("krey") {
              if let Err(msg_why) = member.add_role(role) {
                error!("Failed to add user to cage: {:?}", msg_why);
              }
            }
          }
        }
      }
    }
    if msg.content == "release krey" {
      if msg.author.id != 476270148739661835 {
        CAGE_KREY.store(false, Ordering::Relaxed);
        if let Err(msg_why) = msg.author.dm(|m| m.content(
          "Krey is released! use 'cage krey' command to cage again")) {
          error!("Failed to release krey: {:?}", msg_why);
        }
        if let Some(guild) = msg.guild_id {
          if let Ok(mut member) = guild.member(msg.author.id) {
            if let Ok(partial_guild) = guild.to_partial_guild() {
              if let Some(role) = partial_guild.role_by_name("krey") {
                if let Err(msg_why) = member.remove_role(role) {
                  error!("Failed to remove user from cage: {:?}", msg_why);
                }
              }
            }
          }
        }
      } else {
        if let Err(msg_why) = &msg.author.dm(|m| m.content(
          "Sorry but you can't release yourself! You should be able to ask for it in the cage")) {
          error!("Failed to dm to krey: {:?}", msg_why);
        }
      }
    } else
    if msg.author.id == 476270148739661835 && CAGE_KREY.load(Ordering::Relaxed) {
      if msg.channel_id != 553855059767853066 {
        if let Err(why) = msg.delete() {
          error!("Error deleting krey msg {:?}", why);
        }
        if let Err(msg_why) = msg.author.dm(|m| m.content(
          "Sorry but you can't write outside the cage, you're caged!")) {
          error!("Failed to dm to krey: {:?}", msg_why);
        }
      }
    } else {
      if let Some(find_char_in_words) = OVERWATCH.into_iter().find(|&c| {
        let regex = format!(r"(^|\W)((?i){}(?-i))($|\W)", c);
        let is_overwatch = Regex::new(regex.as_str()).unwrap();
        is_overwatch.is_match(msg.content.as_str()) }) {
        let mut rng = thread_rng();
        set! { ov_reply = OVERWATCH_REPLIES.choose(&mut rng).unwrap()
             , reply = format!("{} {}", ov_reply, find_char_in_words) };
        if let Err(why) = msg.channel_id.say(reply) {
          error!("Error sending overwatch reply: {:?}", why);
        }
      }
    }
  }
}
