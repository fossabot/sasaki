use common::msg::{direct_message, channel_message};
use db;

command!(lookup(_ctx, msg, _args) {
  let db_data = db::lookup();
  channel_message(&msg, db_data.as_str());
});

command!(register(_ctx, msg, _args) {
  if msg.mentions.len() == 0 {
    direct_message(msg, "Must mention user");
    return Ok(());
  }
  for u in &msg.mentions {
    let u_id : i64 = u.id.as_u64().clone() as i64;
    if let Some(guild) = msg.guild_id {
      let guild_id : i64 = guild.as_u64().clone() as i64;
      if let Ok(member) = guild.member(msg.author.id) {
        let new_role : i64 =
          if member.roles.len() > 0 {
            member.roles[0].as_u64().clone() as i64
          } else {
            0
          };
        db::register(u_id, guild_id, new_role);
      }
    }
  }
});

command!(todo(_ctx, msg, _args) {
  let db_data = db::todo();
  channel_message(&msg, db_data.as_str());
});
