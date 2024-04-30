// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use super::schema::*;
use neko_core::*;
use neko_poise::{poise::serenity_prelude::Role, *};
use neko_sqlx::*;
use poise::serenity_prelude::{Context, GuildId, Member, User, UserId};
use sea_query::{Expr, OnConflict, Query};

const CHUNK_SIZE: usize = 10000;

pub async fn check_guild_whitelist(id: GuildId) -> Res<bool> {
  use DiscordCacheWhitelist::*;
  let mut qb = Query::select();
  qb.from(Table);
  qb.column(GuildId);
  qb.and_where(Expr::col((Table, GuildId)).eq(id.0 as i64));
  Ok(sql!(FetchOpt, &qb, (i64,))?.is_some())
}

pub async fn update_guild(ctx: &Context, id: GuildId) -> R {
  log::trace!("update_guild({id}): Requesting guild information");
  let g = ctx.http.get_guild(id.0).await?;
  log::trace!("update_guild({id}): Upserting guild into the db");
  use DiscordCacheGuilds::*;
  query!(
    Upsert,
    Table,
    [GuildId],
    [GuildName, GuildIcon, GuildSplash, GuildOwnerId, GuildVanityUrlCode, GuildBanner],
    [params!(g.id.0, g.name, g.icon, g.splash, g.owner_id.0, g.vanity_url_code, g.banner)]
  );
  Ok(())
}

pub async fn update_roles(roles: Vec<&Role>) -> R {
  use DiscordCacheRoles::*;
  log::trace!("Updating {} roles", roles.len());
  for chunk in roles.chunks(CHUNK_SIZE) {
    query!(
      Upsert,
      Table,
      [RoleId],
      [GuildId, RoleName, RoleColor, RoleHoist, RolePosition, RolePermissions],
      chunk.into_iter().map(|r| params!(
        r.id.0,
        r.guild_id.0,
        r.name.clone(),
        r.colour.0,
        r.hoist,
        r.position,
        r.permissions.bits()
      ))
    );
  }
  Ok(())
}

pub async fn update_users(users: Vec<User>) -> R {
  use DiscordCacheUsers::*;
  log::trace!("Updating {} users", users.len());
  for chunk in users.chunks(CHUNK_SIZE) {
    query!(
      Upsert,
      Table,
      [UserId],
      [
        UserUsername,
        UserGlobalName,
        UserAvatar,
        UserBanner,
        UserAccentColor,
        UserLocale,
        UserAvatarDecoration
      ],
      chunk.into_iter().map(|u| params!(
        u.id.0,
        u.name.clone(),
        u.name.clone(),
        u.avatar.clone(),
        u.banner.clone(),
        0,
        "",
        ""
      ))
    );
  }
  Ok(())
}

pub async fn update_members(members: Vec<Member>) -> R {
  use DiscordCacheMembers::*;
  log::trace!("Updating {} members", members.len());
  for chunk in members.chunks(CHUNK_SIZE) {
    query!(
      Upsert,
      Table,
      [GuildId, UserId],
      [MemberNick, MemberAvatar],
      chunk.into_iter().map(|m| params!(
        m.guild_id.0,
        m.user.id.0,
        m.nick.clone(),
        m.avatar.clone()
      ))
    );
  }
  Ok(())
}

pub async fn remove_guild(id: GuildId) -> R {
  use DiscordCacheGuilds::*;
  log::trace!("remove_guild({id}): Removing guild from the db");
  query!(Delete, Table, GuildId, id.0);
  Ok(())
}

pub async fn prune_all_guilds() -> R {
  use DiscordCacheGuilds::*;
  log::trace!("Pruning all guilds");
  let mut qb = Query::delete();
  qb.from_table(Table);
  sql!(Execute, &qb)?;
  Ok(())
}

pub async fn remove_member(g: GuildId, u: UserId) -> R {
  use DiscordCacheMembers::*;
  log::trace!("remove_member({g}, {u}): Removing member from the db");
  query!(Delete, Table, GuildId, g.0, UserId, u.0);
  Ok(())
}
