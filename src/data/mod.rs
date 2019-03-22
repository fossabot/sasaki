use std::sync::atomic::AtomicBool;
use std::collections::HashMap;
use std::sync::Mutex;

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
  pub static ref SSH_SESSION : Mutex<ssh2::Session> = {
    Mutex::new(ssh2::Session::new().unwrap())
  };
}
