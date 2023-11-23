// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_core::*;

#[tokio::main]
async fn main() -> R {
  if !std::env::var("RUST_LOG").is_ok_and(|f| !f.is_empty()) {
    std::env::set_var("RUST_LOG", "warn");
  }
  pretty_env_logger::init();
  let mut fw = NekoFramework::new();
  fw.run().await?;
  Ok(())
}
