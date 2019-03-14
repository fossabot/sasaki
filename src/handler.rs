use collections::overwatch::{OVERWATCH, OVERWATCH_REPLIES};

use serenity::{
  model::{ event::ResumedEvent, gateway::Ready
         , channel::Message },
  prelude::*,
};

use rand::thread_rng;
use rand::seq::SliceRandom;

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
    if msg.author.bot {
      if msg.content == "EKK EKK" {
        if let Err(why) = msg.delete() {
          error!("Error deleting ekks {:?}", why);
        }
        if let Err(why) = msg.channel_id.say("EKK EKK EKK") {
          error!("Error ekking {:?}", why);
        }
      }
      return
    }
    {
      set! { lower = msg.content.to_lowercase()
           , lower_words = lower.split_whitespace() };
      if let Some(find_char_in_words) = lower_words.into_iter().find(
                |&w| OVERWATCH.into_iter().find(|&c| c == &w).is_some()) {
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
