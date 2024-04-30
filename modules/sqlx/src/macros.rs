// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#[macro_export]
macro_rules! sql {
  (Prepare, $qb:expr) => {{
    (
      $crate::sea_query_binder::SqlxBinder::build_sqlx(
        $qb,
        $crate::sea_query::PostgresQueryBuilder,
      ),
      $crate::get_pool(),
    )
  }};
  (@internal, $qb:expr, $ty:ty, $fn:ident) => {{
    let ((q, v), p) = sql!(Prepare, $qb);
    $crate::sqlx::query_as_with::<_, $ty, _>(&q, v).$fn(p).await
  }};
  (@internal, $qb:expr, $fn:ident) => {{
    let ((q, v), p) = sql!(Prepare, $qb);
    $crate::sqlx::query_with(&q, v).$fn(p).await
  }};
  (FetchOne, $qb:expr, $ty:ty) => {{
    sql!(@internal, $qb, $ty, fetch_one)
  }};
  (FetchAll, $qb:expr, $ty:ty) => {{
    sql!(@internal, $qb, $ty, fetch_all)
  }};
  (FetchOpt, $qb:expr, $ty:ty) => {{
    sql!(@internal, $qb, $ty, fetch_optional)
  }};
  (Execute, $qb:expr) => {{
    sql!(@internal, $qb, execute)
  }};
}

/// Declare a new sea_query schema
#[macro_export]
macro_rules! schema {
  ($(
    //$(#[$meta:meta])*
    $($(#[$tp:ident($($tpp:expr),*)])+)?
    $vis:vis enum $ident:ident {
      $($(#[$param:ident($($tt:expr),*)])+ $field:ident),*$(,)?
    }
  )*) => {
    $(
      #[derive(sea_query::Iden)]
      //$(#[$meta])*
      #[allow(dead_code)]
      $vis enum $ident {
        Table, $($field),*
      }

      impl $crate::Table for $ident {
        fn create() -> $crate::sea_query::TableCreateStatement {
          $crate::sea_query::Table::create().table($ident::Table).if_not_exists()
          $(.col(&mut $crate::sea_query::ColumnDef::new($ident::$field)$(.$param($($tt),*))*))*
          $($(.$tp($($tpp),*))*)?
          .to_owned()
        }
      }
    )*

    pub fn create_tables() -> Vec<sea_query::TableCreateStatement> {
      use $crate::Table;
      vec![$($ident::create()),*]
    }
  };
}

#[macro_export]
macro_rules! fk {
  ($t1:ident, $f1:ident, $t2:ident, $f2:ident, $od:ident, $ou:ident) => {
    $crate::sea_query::ForeignKey::create()
      .from($t1::Table, $t1::$f1)
      .to($t2::Table, $t2::$f2)
      .on_delete($crate::sea_query::ForeignKeyAction::$od)
      .on_update($crate::sea_query::ForeignKeyAction::$ou)
  };
  ($t1:ident, $t2:ident, $f:ident, $od:ident, $ou:ident) => {
    fk!($t1, $f, $t2, $f, $od, $ou)
  };
  ($t:ident, $f:ident, $od:ident, $ou:ident) => {
    fk!(Self, $f, $t, $f, $od, $ou)
  };
}

#[macro_export]
macro_rules! pk {
  ($f1:ident, $f2:ident) => {
    $crate::sea_query::Index::create().col(Self::$f1).col(Self::$f2)
  };
}

#[macro_export]
macro_rules! uk {
  ($f1:ident, $f2:ident) => {
    $crate::sea_query::Index::create().unique().col(Self::$f1).col(Self::$f2)
  };
}

#[macro_export]
macro_rules! init_tables {
  ($fw:ident) => {
    $fw.req::<$crate::Postgres>()?.init_tables(&mut schema::create_tables());
  };
}

#[macro_export]
macro_rules! query {
  (Upsert, $table:ident, [$($pkey:ident),*], [$($columns:ident),*], $values:expr) => {
    let mut qb = $crate::sea_query::Query::insert();
    qb.into_table($table);
    qb.columns([$($pkey),*, $($columns),*]);
    qb.on_conflict(
      OnConflict::columns([$($pkey),*])
        .update_columns([$($columns),*])
        .to_owned(),
    );
    for v in $values {
      qb.values(v)?;
    }
    sql!(Execute, &qb)?;
  };
  (Delete, $table:ident, $($pkey:ident, $pval:expr),*) => {
    sql!(Execute, $crate::sea_query::Query::delete().from_table($table)
      $(.cond_where(Expr::col($pkey).eq($pval)))*)?
  }
}

#[macro_export]
macro_rules! params {
  ($($values:expr),*) => {
    [$($values.into()),*]
  }
}
