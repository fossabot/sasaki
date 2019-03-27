use common::msg::{direct_message, reply};
use conf;

use serenity::{
  model::{ misc::Mentionable
         , id::GuildId, id::ChannelId },
  client::{ CACHE, bridge::voice::ClientVoiceManager },
  voice,
  prelude::*
};

use std::sync::Arc;
use typemap::Key;

pub struct VoiceManager;

impl Key for VoiceManager {
  type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub fn rejoin_voice_channel(ctx : &Context) {
  let conf = conf::parse_config();
  if conf.rejoin {
    set!{ last_guild_u64 = conf.last_guild.parse::<u64>().unwrap_or(0)
        , last_channel_u64 = conf.last_channel.parse::<u64>().unwrap_or(0) };
    if last_guild_u64 != 0 && last_channel_u64 != 0 {
      set!{ last_guild_conf = GuildId( last_guild_u64 )
          , last_channel_conf = ChannelId( last_channel_u64 ) };
      let manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
      let mut manager = manager_lock.lock();
      if manager.join(last_guild_conf, last_channel_conf).is_some() {
        info!("Rejoined voice channel: {}", last_channel_conf);
        if conf.last_stream != "" {
          if let Some(handler) = manager.get_mut(last_guild_conf) {
            let source = match voice::ytdl(&conf.last_stream) {
              Ok(source) => source,
              Err(why) => {
                error!("Err starting source: {:?}", why);
                return ();
              }
            };
            handler.play(source);
          }
        }
      } else {
        error!("Failed to rejoin voice channel: {}", last_channel_conf);
      }
    }
  }
}

command!(join(ctx, msg) {
  let guild = match msg.guild() {
    Some(guild) => guild,
    None => {
      direct_message(msg, "Groups and DMs not supported");
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
      direct_message(msg, "Not in a voice channel");
      return Ok(());
    }
  };
  let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
  let mut manager = manager_lock.lock();
  if manager.join(guild_id, connect_to).is_some() {
    let mut conf = conf::parse_config();
    let last_guild_conf = GuildId( conf.last_guild.parse::<u64>().unwrap_or(0) );
    let last_channel_conf = ChannelId( conf.last_channel.parse::<u64>().unwrap_or(0) );
    if last_guild_conf != guild_id || last_channel_conf != connect_to || conf.rejoin == false {
      conf.rejoin = true;
      conf.last_guild = format!("{}", guild_id);
      conf.last_channel = format!("{}", connect_to);
      conf::write_config(&conf);
    }
    if let Err(why) = msg.channel_id.say(&format!("I've joined {}", connect_to.mention())) {
      error!("failed to say joined {:?}", why);
    }
  } else {
    direct_message(msg, "Error joining the channel");
  }
});

command!(rejoin(ctx, msg) {
  let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
    Some(channel) => channel.read().guild_id,
    None => {
      direct_message(msg, "Groups and DMs not supported");
      return Ok(());
    },
  };
  let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
  let mut manager = manager_lock.lock();
  let has_handler = manager.get(guild_id).is_some();
  if has_handler {
    manager.remove(guild_id);
  }
  let guild = match msg.guild() {
    Some(guild) => guild,
    None => {
      direct_message(msg, "Groups and DMs not supported");
      return Ok(());
    }
  };
  let channel_id = guild
    .read()
    .voice_states.get(&msg.author.id)
    .and_then(|voice_state| voice_state.channel_id);
  let connect_to = match channel_id {
    Some(channel) => channel,
    None => {
      let _ = msg.channel_id.say("You're not in a voice channel");
      return Ok(());
    }
  };
  let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
  let mut manager = manager_lock.lock();
  if manager.join(guild_id, connect_to).is_none() {
    reply(&msg, "failed to rejoin voice channel");
  }
});

command!(leave(ctx, msg) {
  let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
    Some(channel) => channel.read().guild_id,
    None => {
      direct_message(msg, "Groups and DMs not supported");
      return Ok(());
    },
  };
  let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
  let mut manager = manager_lock.lock();
  let has_handler = manager.get(guild_id).is_some();
  if has_handler {
    manager.remove(guild_id);
    let _ = msg.channel_id.say("I left voice channel");
    let mut conf = conf::parse_config();
    if conf.rejoin {
      conf.rejoin = false;
      conf::write_config(&conf);
    }
  } else {
    reply(&msg, "I'm not in a voice channel");
  }
});

command!(play(ctx, msg, args) {
  let url = match args.single::<String>() {
    Ok(url) => url,
    Err(_) => {
      direct_message(msg, "Must provide a URL to a video or audio");
      return Ok(());
    }
  };
  if !url.starts_with("http") {
    direct_message(msg, "Must provide a valid URL");
    return Ok(());
  }
  let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
    Some(channel) => channel.read().guild_id,
    None => {
      direct_message(msg, "Error finding channel info");
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
        direct_message(msg, "Error sourcing ffmpeg");
        return Ok(());
      }
    };
    handler.play(source);
    let mut conf = conf::parse_config();
    let last_stream_conf = conf.last_stream;
    if last_stream_conf != url {
      conf.last_stream = url;
      conf::write_config(&conf);
    }
    let _ = msg.channel_id.say("Playing stream!");
  } else {
    direct_message(msg, "Not in a voice channel to play in");
  }
});
