use serenity::client::{CACHE};
use serenity::model::channel::Message;
use serenity::model::misc::Mentionable;

use serenity::voice;
use serenity::Result as SerenityResult;
use serenity::prelude::Mutex;
use serenity::client::bridge::voice::ClientVoiceManager;

use std::sync::Arc;
use typemap::Key;

pub struct VoiceManager;

impl Key for VoiceManager {
  type Value = Arc<Mutex<ClientVoiceManager>>;
}

fn check_msg(result: SerenityResult<Message>) {
  if let Err(why) = result {
    error!("Error: {:?}", why);
  }
}

command!(join(ctx, msg) {
  let guild = match msg.guild() {
    Some(guild) => guild,
    None => {
      check_msg(msg.author.dm(|m| m.content("Groups and DMs not supported")));
      return Ok(());
    }
  };
  let guild_id = guild.read().id;
  let channel_id = guild
    .read()
    .voice_states.get(&msg.author.id)
    .and_then(|voice_state| voice_state.channel_id);
  let connect_to = match channel_id {
    Some(channel) => channel,
    None => {
      check_msg(msg.author.dm(|m| m.content("Not in a voice channel")));
      return Ok(());
    }
  };
  let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
  let mut manager = manager_lock.lock();
  if manager.join(guild_id, connect_to).is_some() {
    check_msg(msg.author.dm(|m| m.content(&format!("Joined {}", connect_to.mention()))));
  } else {
    check_msg(msg.author.dm(|m| m.content("Error joining the channel")));
  }
});

command!(leave(ctx, msg) {
  let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
    Some(channel) => channel.read().guild_id,
    None => {
      check_msg(msg.author.dm(|m| m.content("Groups and DMs not supported")));
      return Ok(());
    },
  };
  let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
  let mut manager = manager_lock.lock();
  let has_handler = manager.get(guild_id).is_some();
  if has_handler {
    manager.remove(guild_id);
    check_msg(msg.author.dm(|m| m.content("Left voice channel")));
  } else {
    check_msg(msg.reply("I'm not in a voice channel"));
  }
});

command!(play(ctx, msg, args) {
  let url = match args.single::<String>() {
    Ok(url) => url,
    Err(_) => {
      check_msg(msg.author.dm(|m| m.content("Must provide a URL to a video or audio")));
      return Ok(());
    }
  };
  if !url.starts_with("http") {
    check_msg(msg.author.dm(|m| m.content("Must provide a valid URL")));
    return Ok(());
  }
  let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
    Some(channel) => channel.read().guild_id,
    None => {
      check_msg(msg.author.dm(|m| m.content("Error finding channel info")));
      return Ok(());
    }
  };
  let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
  let mut manager = manager_lock.lock();
  if let Some(handler) = manager.get_mut(guild_id) {
    let source = match voice::ytdl(&url) {
      Ok(source) => source,
      Err(why) => {
        error!("Err starting source: {:?}", why);
        check_msg(msg.author.dm(|m| m.content("Error sourcing ffmpeg")));
        return Ok(());
      }
    };
    handler.play(source);
    check_msg(msg.author.dm(|m| m.content("Playing song")));
  } else {
    check_msg(msg.author.dm(|m| m.content("Not in a voice channel to play in")));
  }
});
