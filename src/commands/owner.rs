use data;

use std::sync::atomic::{Ordering};
use std::net::{TcpStream};
use ssh2;

command!(ssh(_ctx, msg, args) {
  if data::SSH_MODE.load(Ordering::Relaxed) {
    data::SSH_MODE.store(false, Ordering::Relaxed);
    info!("Leaving SSH mode!");
    let _ = msg.reply("Leaving SSH mode!");
    if let Ok(mut ifsess) = data::SSH_SESSION.lock() {
      if let Some(ref sess) = *ifsess {
        let longdescription: String = ::std::iter::repeat('A').take(10).collect();
        if let Err(why) = sess.disconnect(None, &longdescription, None) {
          error!("Failed to disconnect from ssh: {}", why);
        }
      }
    }
  } else {
    if args.len() >= 2 {
      let mut address = args.single::<String>().unwrap();
      if !address.contains(':') {
        address += ":22";
      }
      if let Ok(mut tcp_stream) = data::SSH_TCP_STREAM.lock() {
        match TcpStream::connect(&address) {
          Ok(tcp) => {
            if let Ok(mut ifsess) = data::SSH_SESSION.lock() {
              if ifsess.is_none() {
                *ifsess = ssh2::Session::new();
              }
              if ifsess.is_none() {
                error!("failed to create new ssh session");
                let _ = msg.reply("failed to create new ssh session");
                return Ok(());
              }
            }
            if let Some(mut sess) = data::SSH_SESSION.lock().unwrap().as_mut() {
              match sess.handshake(&tcp) {
                Ok(_) => {
                  let name = args.single::<String>().unwrap();
                  if args.len() >= 3 {
                    let password = args.single::<String>().unwrap();
                    match sess.userauth_password(name.as_str(), password.as_str()) {
                      Ok(_) => {
                        if sess.authenticated() {
                          data::SSH_MODE.store(true, Ordering::Relaxed);
                          info!("Entering SSH mode! (not it's not ideal and a bit buggy atm)");
                          let _ = msg.reply("Entering SSH mode!");
                        } else {
                          let _ = msg.reply("Failed to enter SSH mode");
                        }
                      },
                      Err(err) => {
                        let msgToReply = format!("Authentification of user {} failed : {} (pwd: {}))", name, err, password);
                        let _ = msg.reply(msgToReply.as_str());
                      }
                    };
                  } else {
                    match sess.userauth_agent(name.as_str()) {
                      Ok(_) => {
                        if sess.authenticated() {
                          data::SSH_MODE.store(true, Ordering::Relaxed);
                          info!("Entering SSH mode!");
                          let _ = msg.reply("Entering SSH mode!");
                        } else {
                          let _ = msg.reply("Failed to enter SSH mode");
                        }
                      },
                      Err(err) => {
                        let msgToReply = format!("Authentification of user {} failed : {})", name, err);
                        let _ = msg.reply(msgToReply.as_str());
                      }
                    };
                  }
                },
                Err(err) => {
                  let msgToReply = format!("Handshake failed : {}", err);
                  let _ = msg.reply(msgToReply.as_str());
                }
              };
              *tcp_stream = Some(tcp);
            }
          }
          Err(err) => {
            let msgToReply = format!("Failed to connect {} : {}", address, err);
            let _ = msg.reply(msgToReply.as_str());
          }
        };
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
  let _ = msg.reply("Shutting down!");
  let _ = bash!("killall sasaki");
});
