// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#[macro_export]
macro_rules! get_env {
  ($env:literal) => {
    std::env::var($env).expect(concat!($env, " is not present").into())
  };
  ($env:literal, $default:literal) => {
    std::env::var($env).unwrap_or($default.into())
  };
  ($env:literal, $default:literal, $ty:ty) => {
    std::env::var($env)
      .map_err(|i| i.to_string())
      .and_then(|a| a.parse::<$ty>().map_err(|i| i.to_string()))
      .unwrap_or($default)
  };
}

#[macro_export]
macro_rules! this_or_that {
  (($($this:tt)+) or ($($that:tt)+)) => {$($this)+};
  (() or ($($that:tt)+)) => {$($that)+};
}

#[macro_export]
macro_rules! once_cell {
  (@define, $name:ident: $ty:ty) => {
    static $name: $crate::tokio::sync::OnceCell<$ty> = $crate::tokio::sync::OnceCell::const_new();
  };
  ($fun:ident, $name:ident: $ty:ty) => {
    once_cell!(@define, $name: $ty);

    pub fn $fun() -> &'static $ty {
      $name.get().expect(concat!(stringify!($fun), " has not yet been initialized"))
    }
  };
  ($fun:ident, $name:ident: $ty:ty, $block:block) => {
    once_cell!(@define, $name: $ty);

    pub async fn $fun() -> &'static $ty {
      $name.get_or_init(|| async $block).await
    }
  };
}

#[macro_export]
macro_rules! export {
  ($(pub use $module:ident::{$($ident:ident),*};)*) => {$(
    #[macro_use]
    pub mod $module;

    #[allow(unused_imports)]
    pub use $module::{$($ident),*};
  )*};
}
