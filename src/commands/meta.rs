use rand::Rng;
use curl::easy::Easy;
use std::str;
use regex::Regex;

command!(ping(_ctx, msg) {
  let _ = msg.channel_id.say("Go away!");
});

command!(partners(_ctx, msg) {
  /*
  if let Err(msg_why) = msg.author.dm(|m| m.content("Posted!")) {
    error!("Failed to dm to partners request author: {:?}", msg_why);
  }
  */
  let lines : Vec<&str> = msg.content.lines().collect();
  for line in lines {
    let split : Vec<&str> = line.split('|').collect();
    if split.len() > 2 {
      set! { partner_description  = split[0]
           , partner_owner        = split[1]
           , partner_invite       = split[2] };
      let mut easy = Easy::new();
      let app_invite = String::from("https://discordapp.com/invite/") + partner_invite;
      easy.url(app_invite.as_str()).expect("Failed to curl the invite");
      let mut dst = Vec::<u8>::new();
      {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
          dst.extend_from_slice(data);
          Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
      }
      let invite_content = str::from_utf8(&dst).unwrap();

      let mut title;
      if split.len() > 3 {
        title = split[3];
      } else {
        let title_regex = Regex::new(r#"og:title" content="Join the (.*)!"#).unwrap();
        let title_caps = title_regex.captures(invite_content).unwrap();
        title = if title_caps.len() > 0 { title_caps.get(1).map_or("", |m| m.as_str()) }
          else { "" };
      }
      //TODO: replace all html codes...
      let title_fixed = title.replace("&#39;", "'");

      let invite_regex = Regex::new(r#"og:image" content="(.*)\?size="#).unwrap();
      let caps = invite_regex.captures(invite_content).unwrap();

      let thumbnail = if caps.len() > 0 { caps.get(1).map_or("", |m| m.as_str()) }
        else { "https://discordapp.com/assets/2c21aeda16de354ba5334551a883b481.png" };

      let invite_link = String::from("https://discord.gg/") + partner_invite;

      set! { red    = rand::thread_rng().gen_range(0, 255)
           , green  = rand::thread_rng().gen_range(0, 255)
           , blue   = rand::thread_rng().gen_range(0, 255) };
      if let Err(why) = msg.channel_id.send_message(|m| m
        //.content(partner_invite)
        .embed(|e| e
          .title(title_fixed)
          .thumbnail(thumbnail)
          .description(partner_description)
          .fields(vec![
            ("Owner", partner_owner, true),
            ("Invite", invite_link.as_str(), true)
            ])
          .colour((red, green, blue)))) {
        error!("Error posting partner: {:?}", why);
      }
    }
  }
});
