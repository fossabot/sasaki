use serenity::{
  model::{event::ResumedEvent, gateway::Ready},
  prelude::*,
};

pub struct Handler;

impl EventHandler for Handler {
  fn ready(&self, _: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);
  }

  fn resume(&self, _: Context, _: ResumedEvent) {
    info!("Resumed");
  }
}
