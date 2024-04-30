// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use crate::queries::*;
use futures::StreamExt;
use neko_core::*;
use neko_poise::*;

handler!(event_handler, |c, event| {
  use poise::Event::*;
  match event {
    Ready { data_about_bot: _ } => {
      // Removing guilds cascades to removing all guild members and roles
      prune_all_guilds().await?;
    }
    GuildCreate { guild, is_new: _ } => {
      println!("CACHE START CACHING");
      if check_guild_whitelist(guild.id).await? {
        update_guild(c, guild.id).await?;
        let raw_members: Vec<_> = guild.id.members_iter(c).collect().await;
        let members: Vec<_> =
          raw_members.into_iter().filter_map(Result::ok).filter(|m| !m.user.bot).collect();
        let users: Vec<_> = members.clone().into_iter().map(|m| m.user).collect();
        let roles: Vec<_> = guild.roles.values().collect();
        // Users are never pruned
        update_users(users).await?;
        // Members and roles are pruned via cascade on GuildDelete and Ready
        update_members(members).await?;
        update_roles(roles).await?;
      } else {
        log::warn!("Leaving non-whitelisted server '{}' ({})", guild.name, guild.id);
        guild.leave(&c).await?;
      }
      println!("CACHE DONE CACHING");
    }
    GuildUpdate { old_data_if_available: _, new_but_incomplete: g } => {
      if check_guild_whitelist(g.id).await? {
        update_guild(&c, g.id).await?;
      }
    }
    GuildDelete { incomplete: g, full: _ } => {
      if !g.unavailable {
        // Removing guild cascades to removing all guild members and roles
        remove_guild(g.id).await?;
      }
    }
    GuildMemberAddition { new_member: m } => {
      if !m.user.bot && check_guild_whitelist(m.guild_id).await? {
        update_users(vec![m.user.clone()]).await?;
        update_members(vec![m.clone()]).await?;
      }
    }
    GuildMemberUpdate { old_if_available: _, new: m } => {
      if !m.user.bot && check_guild_whitelist(m.guild_id).await? {
        update_users(vec![m.user.clone()]).await?;
        update_members(vec![m.clone()]).await?;
      }
    }
    GuildMemberRemoval { guild_id, user, member_data_if_available: _ } => {
      if !user.bot && check_guild_whitelist(*guild_id).await? {
        remove_member(*guild_id, user.id).await?;
      }
    }
    GuildRoleCreate { new: r } => if check_guild_whitelist(r.guild_id).await? {},
    GuildRoleUpdate { old_data_if_available: _, new: r } => {}
    GuildRoleDelete { guild_id, removed_role_id: id, removed_role_data_if_available: _ } => {
      if check_guild_whitelist(*guild_id).await? {}
    }
    _ => {}
  }
  Ok(())
});
