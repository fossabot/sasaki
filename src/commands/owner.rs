use data;

use std::sync::atomic::{Ordering};

command!(shell(_ctx, msg, _args) {
  if data::SHELL_MODE.load(Ordering::Relaxed) {
    data::SHELL_MODE.store(false, Ordering::Relaxed);
    info!("Leaving shell mode!");
    let _ = msg.reply("Leaving shell mode!");
  } else {
    data::SHELL_MODE.store(true, Ordering::Relaxed);
    info!("Entering shell mode!");
    let _ = msg.reply("Entering shell mode!");
  }
});

command!(quit(ctx, msg, _args) {
  ctx.quit();
  let _ = msg.reply("context shutting down!");
});
