use crate::common::msg::{ split_code, split_message, MESSAGE_LIMIT };

use serenity::{
  builder::CreateMessage,
  model::{ id::GuildId, channel::GuildChannel },
};

pub fn log_any<F: FnOnce(CreateMessage) -> CreateMessage>(guild_id: &GuildId, f: F) {
  if let Ok(channels) = guild_id.channels() {
    let log_channel = channels.iter().find(|&(c, _)|
      if let Some(name) = c.name() {
        name == "log"
      } else {
        false
      });
    if let Some((_, channel)) = log_channel {
      if let Err(why) = channel.send_message(f) {
        error!("Failed to log new user {:?}", why);
      }
    }
  }
}

fn serenity_channel_message_single(chan : &GuildChannel, text: &str) {
  if let Err(why) = chan.say(text) {
    error!("Error sending log message: {:?}", why);
  }
}
fn serenity_channel_message_multi(chan : &GuildChannel, texts : Vec<&str>) {
  for text in texts {
    serenity_channel_message_single(chan, text);
  }
}
fn serenity_channel_message_multi2(chan : &GuildChannel, texts : Vec<String>) {
  for text in texts {
    serenity_channel_message_single(chan, text.as_str());
  }
}
fn channel_message(chan : &GuildChannel, text: &str) {
  if text.len() > MESSAGE_LIMIT {
    if text.starts_with("```") {
      serenity_channel_message_multi2(chan, split_code(text));
    } else {
      serenity_channel_message_multi(chan, split_message(text));
    }
  } else {
    serenity_channel_message_single(chan, text);
  }
}

pub fn log(guild_id: &GuildId, text: &str) {
  if let Ok(channels) = guild_id.channels() {
    let log_channel = channels.iter().find(|&(c, _)|
      if let Some(name) = c.name() {
        name == "log"
      } else {
        false
      });
    if let Some((_, channel)) = log_channel {
      channel_message(channel, text);
    }
  }
}
