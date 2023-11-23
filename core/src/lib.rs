// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

pub mod macros;

mod hasher;

pub type Err = Box<dyn std::error::Error + Send + Sync>;
pub type Res<T> = Result<T, Err>;
pub type R = Res<()>;

pub use futures;
pub use log;
pub use tokio;

export! {
  pub use framework::{NekoFramework};
  pub use module::{Module};
}
