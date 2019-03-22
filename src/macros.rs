#[macro_export]
macro_rules! set {
  ($init:ident = $val:expr, $($lhs:ident = $rhs:expr),*) => {
      let $init = $val;
    $(
      let $lhs = $rhs;
    )*
  };
}

//partially taken from https://github.com/Proksima/shells/blob/master/src/lib.rs
#[macro_export]
macro_rules! bash {
  ( $( $cmd:tt )* ) => {{
    let mut command = {
      let mut command = ::std::process::Command::new("bash");
      command.arg("-c").arg(&format!($( $cmd )*));
      command
    };
    match command.output() {
      Ok(output) => {
        (output.status.code().unwrap_or(if output.status.success() { 0 } else { 1 }),
         String::from_utf8_lossy(&output.stdout[..]).into_owned(),
         String::from_utf8_lossy(&output.stderr[..]).into_owned())
      },
      Err(e) => (126, String::new(), e.to_string()),
    }
  }};
}
