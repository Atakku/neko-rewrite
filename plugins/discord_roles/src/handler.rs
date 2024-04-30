// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_core::*;
use neko_poise::*;

handler!(event_handler, |_, event| {
  use poise::Event::*;
  match event {
    GuildCreate { guild, is_new: _ } => {
      println!("ROLE START");
      println!("{}", guild.name);
      // Check all members in guild
      println!("ROLE STOP");
    }
    GuildMemberAddition { new_member: m } => {
      if !m.user.bot {
        // Check persistant roles for this member
      }
    }
    _ => {}
  }
  Ok(())
});
