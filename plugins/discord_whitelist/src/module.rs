// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_core::*;
use neko_poise::*;
use neko_sqlx::*;
use poise::serenity_prelude::GatewayIntents;

//pub mod handler;
//pub mod queries;
//pub mod schema;

module! {
  /// Whitelisted in-database cache of all known Discord guilds/users/members for usage in database queries
  DiscordCache;

  impl on_init(fw) {
    init_tables!(fw);
    let poise = fw.req::<Poise>()?;
    //poise.add_event_handler(handler::event_handler);
    poise.add_intent(GatewayIntents::GUILDS);
    poise.add_intent(GatewayIntents::GUILD_MEMBERS);
  }
}
