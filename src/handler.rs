use serenity::{
  model::{ event::ResumedEvent, gateway::Ready
         , channel::Message },
  prelude::*,
};

pub struct Handler;

impl EventHandler for Handler {
  fn ready(&self, _ : Context, ready : Ready) {
    info!("Connected as {}", ready.user.name);
  }
  fn resume(&self, _ : Context, _ : ResumedEvent) {
    info!("Resumed");
  }
  fn message(&self, _ : Context, msg : Message) {
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
      if msg.content.contains("tracer") {
        
      }
    }
  }
}
