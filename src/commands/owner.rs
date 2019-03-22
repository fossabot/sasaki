use data;

use std::sync::atomic::{Ordering};
use std::net::{TcpStream};

command!(ssh(_ctx, msg, args) {
  if data::SSH_MODE.load(Ordering::Relaxed) {
    data::SSH_MODE.store(false, Ordering::Relaxed);
    info!("Leaving SSH mode!");
    let _ = msg.reply("Leaving SSH mode!");
    if let Ok(mut sess) = data::SSH_SESSION.lock() {
      let longdescription: String = ::std::iter::repeat('A').take(10).collect();
      if let Err(why) = sess.disconnect(None, &longdescription, None) {
        error!("Failed to disconnect from ssh: {}", why);
      }
    }
  } else {
    if args.len() > 2 {
      let tcp = TcpStream::connect(args.single::<String>().unwrap()).unwrap();
      if let Ok(mut sess) = data::SSH_SESSION.lock() {
        sess.handshake(&tcp).unwrap();
        let name = args.single::<String>().unwrap();
        sess.userauth_agent(&name).unwrap();
        if sess.authenticated() {
          data::SSH_MODE.store(true, Ordering::Relaxed);
          info!("Entering SSH mode!");
          let _ = msg.reply("Entering SSH mode!");
        } else {
          let _ = msg.reply("Failed to enter SSH mode (wrong address)");
        }
      }
    }
  }
});

command!(shell(_ctx, msg) {
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
