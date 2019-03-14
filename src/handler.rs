use collections::overwatch::{OVERWATCH, OVERWATCH_REPLIES};

use serenity::{
  model::{ event::ResumedEvent, gateway::Ready
         , channel::Message },
  prelude::*,
};

use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;
use regex::Regex;

pub struct Handler;

//to dm: msg.author.dm(|m| m.content("Hello!"))

impl EventHandler for Handler {
  fn ready(&self, _ : Context, ready : Ready) {
    info!("Connected as {}", ready.user.name);
  }
  fn resume(&self, _ : Context, _ : ResumedEvent) {
    info!("Resumed");
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
