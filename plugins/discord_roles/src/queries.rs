// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use crate::schema::*;
use neko_core::*;
use neko_poise::{poise::serenity_prelude::Role, *};
use neko_sqlx::*;
use poise::serenity_prelude::{Context, GuildId, Member, User, UserId};
use sea_query::{Expr, OnConflict, Query};

//pub async fn update_roles(roles: Vec<&Role>) -> R {
//  use DiscordCacheRoles::*;
//  log::trace!("Updating {} roles", roles.len());
//  for chunk in roles.chunks(CHUNK_SIZE) {
//    query!(
//      Upsert,
//      Table,
//      [RoleId],
//      [GuildId, RoleName, RoleColor, RoleHoist, RolePosition, RolePermissions],
//      chunk.into_iter().map(|r| params!(
//        r.id.0,
//        r.guild_id.0,
//        r.name.clone(),
//        r.colour.0,
//        r.hoist,
//        r.position,
//        r.permissions.bits()
//      ))
//    );
//  }
//  Ok(())
//}
