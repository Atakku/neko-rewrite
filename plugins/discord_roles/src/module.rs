// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_core::*;
use neko_poise::*;
use neko_sqlx::*;

pub mod commands;
pub mod handler;
pub mod queries;
pub mod schema;

module! {
  /// Rolepickers, role creation, role persitance
  DiscordRoles;

  impl on_init(fw) {
    init_tables!(fw);
    let poise = fw.req::<Poise>()?;
    poise.add_event_handler(handler::event_handler);
  }
}
