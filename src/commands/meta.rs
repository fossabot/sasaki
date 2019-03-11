use rand::Rng;

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
    if split.len() > 3 {
      set! { partner_name         = split[0]
           , partner_description  = split[1]
           , partner_owner        = split[2]
           , partner_invite       = split[3] };
      set! { red    = rand::thread_rng().gen_range(0, 255)
           , green  = rand::thread_rng().gen_range(0, 255)
           , blue   = rand::thread_rng().gen_range(0, 255) };
      let thumbnail = if split.len() > 4 { split[4] } else { "https://discordapp.com/assets/2c21aeda16de354ba5334551a883b481.png" };
      if let Err(why) = msg.channel_id.send_message(|m| m
        //.content(partner_invite)
        .embed(|e| e
          .title(partner_name)
          .thumbnail(thumbnail)
          .description(partner_description)
          .fields(vec![
            ("Owner", partner_owner, true),
            ("Invite", partner_invite, true)
            ])
          .colour((red, green, blue)))) {
        error!("Error posting partner: {:?}", why);
      }
    }
  }
});
