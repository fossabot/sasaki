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
        let mut roles_vector : Vec<i64> = Vec::new();
        for role in member.roles {
          roles_vector.push(
            role.as_u64().clone() as i64);
        }
        db::register(u_id, guild_id, &roles_vector);
      }
    }
  }
});

command!(todo(_ctx, msg, _args) {
  let db_data = db::todo();
  channel_message(&msg, db_data.as_str());
});
