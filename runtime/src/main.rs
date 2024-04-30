// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_core::*;
use neko_cron::job;

#[tokio::main]
async fn main() -> R {
  if !std::env::var("RUST_LOG").is_ok_and(|f| !f.is_empty()) {
    std::env::set_var("RUST_LOG", "warn");
  }
  pretty_env_logger::init();
  let mut fw = NekoFramework::new();
  job!(fw, "*/1 * * * * *", {
    log::warn!("hi i run every second");
  });
  fw.init(neko_discord_cache::DiscordCache)?;
  //fw.init(neko_discord_roles::DiscordRoles)?;
  //fw.init(FemboyTV)?;
  //fw.init(DiscordCache)?;
  //fw.init(Steam)?;
  //fw.init(DeepRockGalactic)?;
  //fw.init(Gwaaa {})?;
  fw.run().await?;
  Ok(())
}
