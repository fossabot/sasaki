use crate::{
  common::msg::{ direct_message, channel_message, reply },
  db
};

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

command!(todo(_ctx, msg, args) {
  if args.len() > 0 {
    let mut ifrm = args.single::<String>().unwrap();
    if ifrm == "rm" {
      if args.len() > 1 {
        let mut id_string = args.single::<String>().unwrap();
        if let Ok(id) = id_string.parse::<usize>() {
          db::todo_rm(msg.author.id.as_u64().clone() as i64, id);
        } else {
          reply(&msg, "id to remove should be a number");
        }
      } else {
        reply(&msg, "specify id to remove, please");
      }
    } else {
      let text = msg.content.clone();
      let first_space = text.find(' ').unwrap();
      let start_from =
        if let Some(first_newline) = text.find('\n') {
          if first_space < first_newline { first_space }
          else { first_newline }
        } else { first_space } + 1;
      let todo_text = &text[start_from..];
      db::todo_add(msg.author.id.as_u64().clone() as i64, String::from(todo_text));
    }
  } else {
    let db_data = db::todo(msg.author.id.as_u64().clone() as i64);
    channel_message(&msg, db_data.as_str());
  }
});
