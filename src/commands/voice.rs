use serenity::{
  model::{ channel::Message, misc::Mentionable },
  client::{ CACHE, bridge::voice::ClientVoiceManager },
  voice,
  prelude::Mutex
};

use std::sync::Arc;
use typemap::Key;

pub struct VoiceManager;

impl Key for VoiceManager {
  type Value = Arc<Mutex<ClientVoiceManager>>;
}

fn dm(msg : &Message, text: &str) {
  if let Err(why) = msg.author.dm(|m| m.content(text)) {
    error!("Error DMing user: {:?}", why);
  }
}

command!(join(ctx, msg) {
  let guild = match msg.guild() {
    Some(guild) => guild,
    None => {
      dm(msg, "Groups and DMs not supported");
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
      dm(msg, "Not in a voice channel");
      return Ok(());
    }
  };
  let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
  let mut manager = manager_lock.lock();
  if manager.join(guild_id, connect_to).is_some() {
    dm(msg, &format!("Joined {}", connect_to.mention()));
  } else {
    dm(msg, "Error joining the channel");
  }
});

command!(leave(ctx, msg) {
  let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
    Some(channel) => channel.read().guild_id,
    None => {
      dm(msg, "Groups and DMs not supported");
      return Ok(());
    },
  };
  let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
  let mut manager = manager_lock.lock();
  let has_handler = manager.get(guild_id).is_some();
  if has_handler {
    manager.remove(guild_id);
    dm(msg, "Left voice channel");
  } else {
    dm(msg, "I'm not in a voice channel");
  }
});

command!(play(ctx, msg, args) {
  let url = match args.single::<String>() {
    Ok(url) => url,
    Err(_) => {
      dm(msg, "Must provide a URL to a video or audio");
      return Ok(());
    }
  };
  if !url.starts_with("http") {
    dm(msg, "Must provide a valid URL");
    return Ok(());
  }
  let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
    Some(channel) => channel.read().guild_id,
    None => {
      dm(msg, "Error finding channel info");
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
        dm(msg, "Error sourcing ffmpeg");
        return Ok(());
      }
    };
    handler.play(source);
    dm(msg, "Playing song");
  } else {
    dm(msg, "Not in a voice channel to play in");
  }
});
