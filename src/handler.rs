use collections::overwatch::{OVERWATCH, OVERWATCH_REPLIES};

use serenity::{
  model::{ event::ResumedEvent, gateway::Ready
         , channel::Message },
  prelude::*,
};

use rand::Rng;

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
    //TODO: find better way to ignore myself
    if msg.author.name != "Sasaki" {
      if msg.content == "sasaki help" {
        if let Err(why) = msg.channel_id.send_message(|m| m
          .content("Sasaki help")
          .embed(|e| e
            .title("Sasaki")
            .description("Sasaki:")
            .fields(vec![
              ("field1", "Hello", true),
              ("field2", "I'm Sasaki", true)
              ])
            .field("field3", "Nice to meet you", false)
            .footer(|f| f.text("Sasaki."))
            .colour((246, 111, 0)))) {
          error!("Error sending help message: {:?}", why);
        }
      } else {
        letrec! { lower = msg.content.to_lowercase()
                , lower_words = lower.split_whitespace() };
        if let Some(find_char_in_words) = lower_words.into_iter().find(
                  |&w| OVERWATCH.into_iter().find(|&c| c == &w).is_some()) {
          letrec! { ov_reply = rand::thread_rng().choose(OVERWATCH_REPLIES).unwrap()
                  , reply = format!("{} {}", ov_reply, find_char_in_words) };
          if let Err(why) = msg.channel_id.say(reply) {
            error!("Error sending overwatch reply: {:?}", why);
          }
        }
      }
    }
  }
}
