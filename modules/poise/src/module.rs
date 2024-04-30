// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use crate::*;
use poise::serenity_prelude::GatewayIntents;

module! {
  Poise {
    token: String = get_env!("DISCORD_TOKEN"),
    intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES,
    commands: Vec<PoiseCommand>,
    event_handlers: Vec<PoiseEventHandler>,
  }

  impl on_init(fw) {
    fw.req::<Fluent>()?;
  }

  impl on_runtime(poise) {
    future!({
      PoiseFw::builder()
        .token(poise.token)
        .intents(poise.intents)
        .options(FrameworkOptions {
          commands: localized_commands(poise.commands, loc()),
          event_handler: |c, e, _f, ehs| {
            Box::pin(async move {
              for res in join_all(ehs.iter().map(|eh|(eh)(c,e))).await {
                res?;
              }
              Ok(())
            })
          },
          ..Default::default()
        })
        .setup(move |_c, _r, _f| {
          Box::pin(async move { Ok(poise.event_handlers) })
        })
        .run()
        .await?;
      Ok(())
    })
  }

  fn add_command(&mut self, cmd: PoiseCommand) {
    self.commands.push(cmd);
  }

  fn add_commands(&mut self, mut cmd: Vec<PoiseCommand>) {
    self.commands.append(&mut cmd);
  }

  fn add_event_handler(&mut self, eh: PoiseEventHandler) {
    self.event_handlers.push(eh);
  }

  fn add_intent(&mut self, intent: GatewayIntents) {
    self.intents.insert(intent);
  }
}
