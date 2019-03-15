use rand::Rng;
use curl::easy::Easy;
use std::str;
use regex::Regex;

command!(help(_ctx, msg) {
  let version = format!("Sasaki {}", env!("CARGO_PKG_VERSION").to_string());
  if let Err(why) = msg.channel_id.send_message(|m| m
    .embed(|e| e
      .title("My name")
      .description("佐々木 優太")
      .fields(vec![
        ("Age", "15", true),
        ("Birthdate", "December 23", true)
        ])
      .fields(vec![
        ("Height", "152 cm", true),
        ("Version", version.as_str(), true)
        ])
      .field("cage krey", "cages krey (can be used by everyone but no bots)", false)
      .field("release krey", "releases krey from cage (can be used by everyone except krey)", false)
      .field("play <url>", "play an radio stream or youtube music", false)
      .footer(|f| f.text("proficient in martial arts, extremely cruel"))
      .colour((246, 111, 0)))) {
    error!("Error sending help message: {:?}", why);
  }
});

command!(partners(_ctx, msg) {
  let lines : Vec<&str> = msg.content.lines().collect();
  for line in lines {
    let split : Vec<&str> = line.split('|').collect();
    if split.len() > 2 {
      let mut partner_description  = split[0];
      set! { partner_owner        = split[1]
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

      let invite_regex = Regex::new(r#"meta name="twitter:image" content="(.*)\?size="#).unwrap();
      let caps = invite_regex.captures(invite_content).unwrap();
      let thumbnail = if caps.len() > 0 { caps.get(1).map_or("", |m| m.as_str()) }
        else { "https://discordapp.com/assets/2c21aeda16de354ba5334551a883b481.png" };

      let invite_link = String::from("https://discord.gg/") + partner_invite;

      set! { desc_regex = Regex::new(r#"og:description" content="(.*) \|"#).unwrap()
           , desc_caps = desc_regex.captures(invite_content) };
      let description =
        match desc_caps {
          Some(x) => if x.len() > 0 { x.get(1).map_or("", |m| m.as_str()) } else { "" },
          None    => {
            let temp = partner_description;
            partner_description = "-";
            temp
          }
       };

      set! { mc_regex = Regex::new(r#"\| (.*) members"#).unwrap()
           , mc_caps = mc_regex.captures(invite_content) };
      let members =
        match mc_caps {
          Some(x) => if x.len() > 0 { x.get(1).map_or("", |m| m.as_str()) } else { "" },
          None    => {
            set! { mc_regex2 = Regex::new(r#"with (.*) other members"#).unwrap()
                 , mc_caps2 = mc_regex2.captures(invite_content) };
            match mc_caps2 {
              Some(xx) => if xx.len() > 0 { xx.get(1).map_or("", |m| m.as_str()) } else { "" },
              None    => {
                "-"
              }
           }
          }
        };

      set! { red    = rand::thread_rng().gen_range(0, 255)
           , green  = rand::thread_rng().gen_range(0, 255)
           , blue   = rand::thread_rng().gen_range(0, 255) };
      if let Err(why) = msg.channel_id.send_message(|m| m
        //.content(partner_invite)
        .embed(|e| e
          .title(title_fixed)
          .thumbnail(thumbnail)
          .description(description)
          .fields(vec![
            ("Owner", partner_owner, true),
            ("Invite", invite_link.as_str(), true)
            ])
          .fields(vec![
            ("Members", members, true),
            ("Note", partner_description, true)
            ])
          .colour((red, green, blue)))) {
        error!("Error posting partner: {:?}", why);
      }
    }
  }
});
