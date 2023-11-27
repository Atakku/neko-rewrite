// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_core::*;
use reqwest::Client;

pub mod macros;

pub use reqwest;

once_cell!(get_reqwest, CLIENT: Client);

module! {
  Reqwest {
    user_agent: String = get_env!("USER_AGENT", "neko.rs"),
  }

  impl on_runtime(reqwest) {
    CLIENT.set(Client::builder().user_agent(reqwest.user_agent).build()?)?;
    Ok(None)
  }
}
