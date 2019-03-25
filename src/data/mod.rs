use std::sync::atomic::AtomicBool;
use std::collections::HashMap;
use std::sync::Mutex;
use std::net::TcpStream;

use ssh2;

pub static SHELL_MODE : AtomicBool = AtomicBool::new(false);
pub static SSH_MODE : AtomicBool = AtomicBool::new(false);

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
  pub static ref SSH_SESSION : Mutex<Option<ssh2::Session>> = {
    Mutex::new(None)
  };
  pub static ref SSH_TCP_STREAM : Mutex<Option<TcpStream>> = {
    Mutex::new(None)
  };
}
