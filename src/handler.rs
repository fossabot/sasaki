use collections::overwatch::{OVERWATCH, OVERWATCH_REPLIES};

use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use serenity::{
  model::{ event::ResumedEvent, gateway::Ready
         , channel::Message },
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
    if member.user_id() == 476270148739661835 {
      if let Ok(guild) = guild_id.to_partial_guild() {
        if let Some(role) = guild.role_by_name("krey") {
          if let Err(why) = member.add_role(role) {
            error!("Failed to assign krey role to krey {:?}", why);
          }
        }
      }
    }
  }
  fn message(&self, _ : Context, msg : Message) {
    if msg.is_own() {
      return
    }
    if msg.author.bot {
      let rnd = rand::thread_rng().gen_range(0, 2);
      if rnd == 1 {
        if let Err(why) = msg.delete() {
          error!("Error deleting ekks {:?}", why);
        }
        if let Err(why) = msg.channel_id.say(msg.content) {
          error!("Error ekking {:?}", why);
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
    if CAGE_KREY.load(Ordering::Relaxed) {
      if msg.author.id == 476270148739661835 {
        if msg.channel_id != 553855059767853066 {
          if let Err(why) = msg.delete() {
            error!("Error deleting krey msg {:?}", why);
          }
          if let Err(msg_why) = msg.author.dm(|m| m.content(
            "Sorry but you can't write outside the cage, you're caged!")) {
            error!("Failed to dm to krey: {:?}", msg_why);
          }
        }
      }
    } else
    {
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
