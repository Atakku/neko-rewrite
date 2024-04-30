// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#[macro_export]
macro_rules! handler {
  ($name:ident, |$ctx:tt, $event:tt| $block:block) => {
    pub(crate) fn $name<'a>($ctx: &'a $crate::poise::serenity_prelude::Context, $event: &'a $crate::poise::Event<'a>) -> $crate::poise::BoxFuture<'a, R> {
      Box::pin(async move $block)
    }
  };
}

#[macro_export]
macro_rules! commands {
  (@internal $(#[$m:meta])* fn $name:ident($ctx:ident$(,$($tt:tt)*)?) $block:block) => {
    $(#[$m])*
    pub async fn $name($ctx: $crate::Ctx<'_>, $($($tt)*)?) -> neko_core::R $block
  };
  (@internal $(#[$m:meta])* fn $name:ident($ctx:ident$(,$($tt:tt)*)?) $(-> BasicCommand)? $block:block) => {
    commands!(@internal $(#[$m])* #[poise::command(prefix_command, slash_command)] fn $name($ctx$(,$($tt)*)?) $block);
  };
  (@internal $(#[$m:meta])* fn $name:ident($ctx:ident$(,$($tt:tt)*)?) $(-> OwnerCommand)? $block:block) => {
    commands!(@internal $(#[$m])* #[poise::command(prefix_command, owners_only)] fn $name($ctx$(,$($tt)*)?) $block);
  };
  ($($(#[$m:meta])* fn $name:ident($ctx:ident$(,$($tt:tt)*)?) $(-> $type:ident)? $block:block)*) => {
    $(commands!(@internal $(#[$m])* fn $name($ctx$(,$($tt)*)?) $(-> $type)? $block);)*
  };
}

#[macro_export]
macro_rules! autocomplete {
  ($fn_name:ident, $path:path, $id:ident, $name:ident) => {
    pub async fn $fn_name<'a>(
      _: $crate::Ctx<'_>,
      search: &'a str,
    ) -> Vec<$crate::poise::AutocompleteChoice<String>> {
      use neko_sqlx::sea_query::{Alias, Expr, Func, Order};
      use $path::*;
      let mut qb = neko_sqlx::sea_query::SelectStatement::new();
      qb.from(Table);
      qb.columns([$id, $name]);
      qb.and_where(
        Expr::expr(Func::lower(Expr::col($name)))
          .like(format!("%{}%", search.to_lowercase()))
          .or(Expr::col($id).cast_as(Alias::new("TEXT")).like(format!("%{search}%"))),
      );
      qb.order_by($name, Order::Asc);
      qb.limit(25);
      use $crate::unicode_truncate::UnicodeTruncateStr;
      sql!(FetchAll, &qb, (i64, String))
        .unwrap_or(vec![])
        .into_iter()
        .map(|g| $crate::poise::AutocompleteChoice {
          value: g.0.to_string(),
          name: g.1.unicode_truncate(100).0.into(),
        })
        .collect()
    }
  };
}

#[macro_export]
macro_rules! command_group {
  ($name:ident, $($sub:literal),*) => {
    #[poise::command(prefix_command, slash_command, subcommand_required, subcommands($($sub),*))]
    pub async fn $name(_: neko_poise::Ctx<'_>) -> neko_core::R {Ok(())}
  };
}
