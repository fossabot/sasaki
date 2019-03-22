use std::sync::atomic::AtomicBool;
use std::collections::HashMap;
use std::sync::Mutex;

pub static SHELL_MODE : AtomicBool = AtomicBool::new(false);

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
