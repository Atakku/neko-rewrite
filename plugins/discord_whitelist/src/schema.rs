// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_poise::*;
use neko_sqlx::*;

autocomplete!(discord_guilds, DiscordCacheGuilds, GuildId, GuildName);

schema! {
  pub enum DiscordCacheGuilds {
    GuildId.big_integer().primary_key(),
    GuildName.string_len(100).not_null(),
    GuildIcon.string_len(34),
    GuildSplash.string_len(34),
    GuildOwnerId.big_integer().not_null(),
    GuildVanityUrlCode.string(),
    GuildBanner.string_len(34)
  }

  pub enum DiscordCacheUsers {
    UserId.big_integer().primary_key(),
    UserUsername.string_len(32).not_null(),
    UserGlobalName.string_len(32).not_null(),
    UserAvatar.string_len(34),
    UserBanner.string_len(34),
    UserAccentColor.integer(),
    UserLocale.string_len(5),
    UserAvatarDecoration.string_len(34),
  }

  pub enum DiscordCacheMembers {
    UserId.big_integer().not_null(),
    GuildId.big_integer().not_null(),
    MemberNick.string_len(32),
    MemberAvatar.string_len(34);

    Self.primary_key(pk!(UserId, GuildId))
      .foreign_key(fk!(DiscordCacheGuilds, GuildId, Cascade, Cascade))
      .foreign_key(fk!(DiscordCacheUsers, UserId, Cascade, Cascade))
  }

  pub enum DiscordCacheRoles {
    RoleId.big_integer().primary_key(),
    GuildId.big_integer().not_null(),
    RoleName.string_len(100).not_null(),
    RoleColor.integer(),
    RoleHoist.boolean().not_null(),
    RolePosition.integer().not_null(),
    RolePermissions.big_integer().not_null();

    Self.foreign_key(fk!(DiscordCacheGuilds, GuildId, Cascade, Cascade))
  }

  pub enum DiscordCacheWhitelist {
    GuildId.big_integer().primary_key()
  }
}
