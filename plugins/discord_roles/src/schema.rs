// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_sqlx::*;

schema! {
            pub enum DiscordRolesGroups {
              #[big_integer().primary_key()]
    GroupId,
    #[big_integer().not_null()]
    GuildId,
    #[string_len(100).not_null()]
    GroupName,
    #[boolean().not_null()]
    Persistent
  }

  #[foreign_key(fk!(DiscordRolesGroups, GroupId, SetNull, Cascade))]
  pub enum DiscordRolesRoles {
    #[big_integer().primary_key()]
    RoleId,
    #[big_integer()]
    GroupId
  }

  // Roles are unique to each guild, so there is no need for member/guild relations
  #[primary_key(pk!(RoleId, UserId))
  .foreign_key(fk!(DiscordRolesRoles, RoleId, Cascade, Cascade))]
  pub enum DiscordRolesPersistence {
    #[big_integer().not_null()]
    RoleId,
    #[big_integer().not_null()]
    UserId
  }
}
