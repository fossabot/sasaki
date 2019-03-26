use std::sync::atomic::AtomicBool;
use std::collections::HashMap;
use std::sync::Mutex;
use std::net::TcpStream;
use std::cell::RefCell;

use ssh2::{ Channel, Session };

pub static SHELL_MODE : AtomicBool = AtomicBool::new(false);
pub static SSH_MODE : AtomicBool = AtomicBool::new(false);

pub struct Sess<'s> {
  pub session: &'s Session,
  pub channel: Channel<'s>,
  pub channel_id: i32,
}

impl<'z> ::std::ops::Deref for Sess<'z> {
	type Target = i32;
	fn deref(&self) -> &i32 { &self.channel_id }
}

impl<'z> ::std::ops::DerefMut for Sess<'z> {
	fn deref_mut(&mut self) -> &mut i32 { &mut self.channel_id }
}

rental! {
  pub mod rentals {
    use super::*;
    #[rental_mut]
    pub struct SSHSession {
      session_box: Box<Session>,
      sess: Sess<'session_box>,
    }
  }
}

#[derive(PartialEq, Eq, Hash)]
pub enum DataField {
  Owner,
  Guild
}

lazy_static! {
  pub static ref DATA: Mutex<HashMap<DataField, u64>> = {
    Mutex::new(HashMap::new())
  };
}

lazy_static! {
  pub static ref SSH_TCP_STREAM : Mutex<Option<TcpStream>> = {
    Mutex::new(None)
  };
}

thread_local! {
  pub static SSH_RENTAL: RefCell<Option<self::rentals::SSHSession>> = {
    RefCell::new(None)
  };
}
