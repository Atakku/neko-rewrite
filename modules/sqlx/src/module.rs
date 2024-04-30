// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_core::*;
use sea_query::TableCreateStatement;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod macros;

pub use sea_query;
pub use sea_query_binder;
pub use sqlx;

once_cell!(get_pool, POOL: PgPool);

module! {
  Postgres {
    db_url: String = get_env!("DATABASE_URL"),
    options: PgPoolOptions,
    tables: Vec<TableCreateStatement>
  }

  impl on_runtime(postgres) {
    POOL.set(postgres.options.connect(&postgres.db_url).await?)?;
    for init in postgres.tables {
      let sql = init.build(sea_query::PostgresQueryBuilder);
      sqlx::query(&sql).execute(get_pool()).await?;
    }
    Ok(None)
  }

  fn init_table<T: Table>(&mut self) {
    self.tables.push(T::create())
  }

  fn init_tables(&mut self, tables: &mut Vec<TableCreateStatement>) {
    self.tables.append(tables)
  }
}

pub trait Table {
  fn create() -> TableCreateStatement;
}
