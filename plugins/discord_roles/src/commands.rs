// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_poise::*;

command_group!(roles, "role_create");

commands! {
  fn role_create(ctx, role_name: String) -> BasicCommand {

    Ok(())
  }

  fn group_create(ctx, group_name: String) -> BasicCommand {

    Ok(())
  }
}
