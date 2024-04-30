// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_poise::*;
use neko_sqlx::*;

autocomplete!(discord_guilds, DiscordCacheGuilds, GuildId, GuildName);

schema! {
  pub enum DiscordCacheGuilds {
    #[big_integer()]
         #[primary_key()]
    GuildId,
    #[string_len(100)]
    #[not_null()]
    GuildName,
    #[string_len(34)]
    GuildIcon,
    #[string_len(34)]
    GuildSplash,
    #[big_integer()]
    #[not_null()]
    GuildOwnerId,
    #[string()]
    GuildVanityUrlCode,
    #[string_len(34)]
    GuildBanner
  }

  pub enum DiscordCacheUsers {
    #[big_integer()]
    #[primary_key()]
    UserId,
    #[string_len(32)]
    #[not_null()]
    UserUsername,
    #[string_len(32)]
    #[not_null()]
    UserGlobalName,
    #[string_len(34)]
    UserAvatar,
    #[string_len(34)]
    UserBanner,
    #[integer()]
    UserAccentColor,
    #[string_len(5)]
    UserLocale,
    #[string_len(34)]
    UserAvatarDecoration,
  }

  #[test="primary_key(pk!(UserId, GuildId)).foreign_key(fk!(DiscordCacheGuilds, GuildId, Cascade, Cascade)).foreign_key(fk!(DiscordCacheUsers, UserId, Cascade, Cascade))"]
  pub enum DiscordCacheMembers {
    #[big_integer().not_null()]
    UserId,
    #[big_integer().not_null()]
    GuildId,
    #[string_len(32)]
    MemberNick,
    #[string_len(34)]
    MemberAvatar
  }

  #[foreign_key(fk!(DiscordCacheGuilds, GuildId, Cascade, Cascade))]
  pub enum DiscordCacheRoles {
    #[big_integer().primary_key()]
    RoleId ,
    #[big_integer().not_null()]
    GuildId ,
    #[string_len(100).not_null()]
    RoleName ,
    #[integer()]
    RoleColor,
    #[boolean().not_null()]
    RoleHoist ,
    #[integer().not_null()]
    RolePosition ,
    #[big_integer().not_null()]
    RolePermissions
  }

  pub enum DiscordCacheWhitelist {
    #[big_integer().primary_key()]
    GuildId
  }
}
