command!(ping(_ctx, msg) {
  let _ = msg.channel_id.say("Go away!");
});
