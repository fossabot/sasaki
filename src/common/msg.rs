use serenity::{
  model::{ channel::Message }
};

pub static MESSAGE_LIMIT: usize = 2000;

fn serenity_direct_message_single(msg : &Message, text: &str) {
  if let Err(why) = msg.author.dm(|m| m.content(text)) {
    error!("Error DMing user: {:?}", why);
  }
}

fn serenity_reply_single(msg : &Message, text: &str) {
  if let Err(why) = msg.reply(text) {
    error!("Error replieng to user: {:?}", why);
  }
}

fn serenity_channel_message_single(msg : &Message, text: &str) {
  if let Err(why) = msg.channel_id.say(text) {
    error!("Error sending message to channel: {:?}", why);
  }
}

fn serenity_direct_message_multi(msg : &Message, texts : Vec<&str>) {
  for text in texts {
    serenity_direct_message_single(msg, text);
  }
}
fn serenity_direct_message_multi2(msg : &Message, texts : Vec<String>) {
  for text in texts {
    serenity_direct_message_single(msg, text.as_str());
  }
}

fn serenity_reply_multi(msg : &Message, texts : Vec<&str>) {
  for text in texts {
    serenity_reply_single(msg, text);
  }
}
fn serenity_reply_multi2(msg : &Message, texts : Vec<String>) {
  for text in texts {
    serenity_reply_single(msg, text.as_str());
  }
}

fn serenity_channel_message_multi(msg : &Message, texts : Vec<&str>) {
  for text in texts {
    serenity_channel_message_single(msg, text);
  }
}
fn serenity_channel_message_multi2(msg : &Message, texts : Vec<String>) {
  for text in texts {
    serenity_channel_message_single(msg, text.as_str());
  }
}

pub fn split_code(text: &str) -> Vec<String> {
  let first_space = text.find(' ').unwrap();
  let first_newline = text.find('\n').unwrap();
  let start_from = if first_space < first_newline { first_space }
                   else { first_newline };
  let starting_pattern = &text[..start_from];
  let whole_new_text = &text[start_from..text.len()-4];
  let peaces = whole_new_text.as_bytes()
    .chunks(MESSAGE_LIMIT - 200)
    .map(|s| unsafe { ::std::str::from_utf8_unchecked(s).replace("```", "'''") });
  peaces.map(|s| format!("{}\n{}\n```", starting_pattern, s)).collect()
}

pub fn split_message(text: &str) -> Vec<&str> {
  text.as_bytes()
    .chunks(MESSAGE_LIMIT)
    .map(|s| unsafe { ::std::str::from_utf8_unchecked(s) })
    .collect::<Vec<&str>>()
}

pub fn direct_message(msg : &Message, text: &str) {
  if text.len() > MESSAGE_LIMIT {
    if text.starts_with("```") {
      serenity_direct_message_multi2(msg, split_code(text));
    } else {
      serenity_direct_message_multi(msg, split_message(text));
    }
  } else {
    serenity_direct_message_single(msg, text);
  }
}

pub fn reply(msg : &Message, text: &str) {
  if text.len() > MESSAGE_LIMIT {
    if text.starts_with("```") {
      serenity_reply_multi2(msg, split_code(text));
    } else {
      serenity_reply_multi(msg, split_message(text));
    }
  } else {
    serenity_reply_single(msg, text);
  }
}

pub fn channel_message(msg : &Message, text: &str) {
  if text.len() > MESSAGE_LIMIT {
    if text.starts_with("```") {
      serenity_channel_message_multi2(msg, split_code(text));
    } else {
      serenity_channel_message_multi(msg, split_message(text));
    }
  } else {
    serenity_channel_message_single(msg, text);
  }
}
