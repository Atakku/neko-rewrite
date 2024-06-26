// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use futures::future::join_all;
use neko_core::*;
use neko_fluent::{loc, localize, Fluent, FluentBundle, FluentBundles};
use poise::{
  serenity_prelude::Context as SCtx, BoxFuture, Command, Context, Event, Framework,
  FrameworkContext, FrameworkOptions,
};

pub type PoiseEventHandler = for<'a> fn(&'a SCtx, &'a Event<'a>) -> BoxFuture<'a, R>;
pub type PoiseFw = Framework<Vec<PoiseEventHandler>, Err>;
pub type FwCtx<'a> = FrameworkContext<'a, Vec<PoiseEventHandler>, Err>;
pub type Ctx<'a> = Context<'a, Vec<PoiseEventHandler>, Err>;
pub type PoiseCommand = Command<Vec<PoiseEventHandler>, Err>;

pub use poise;
pub use unicode_truncate;

pub mod macros;
pub mod module;
pub use module::Poise;

//TODO map this
const LOCALES: [&str; 31] = [
  "id", "da", "de", "en-GB", "en-US", "es-ES", "fr", "hr", "it", "lt", "hu", "nl", "no", "pl",
  "pt-BR", "ro", "fi", "sv-SE", "vi", "tr", "cs", "el", "bg", "ru", "uk", "hi", "th", "zh-CN",
  "ja", "zh-TW", "ko",
];

fn localized_commands(mut commands: Vec<PoiseCommand>, fb: &FluentBundles) -> Vec<PoiseCommand> {
  if let Some(bun) = fb.bundles.get(&fb.default) {
    for loc in LOCALES {
      log::trace!("Defaulting locale '{loc}' from '{}'", fb.default);
      for cmd in &mut commands {
        log::trace!("Defaulting locale '{loc}' for '{}' from '{}'", cmd.name, fb.default);
        localize_cmd(cmd, loc, bun, None, loc == fb.default)
      }
    }
  } else {
    log::warn!("Default locale '{}' was not found", fb.default);
  }
  for (loc, bun) in
    fb.bundles.iter().filter(|(l, _)| LOCALES.contains(&l.as_str()) && *l != &fb.default)
  {
    log::info!("Applying locale '{loc}'");
    for cmd in &mut commands {
      log::trace!("Applying locale '{loc}' to '{}'", cmd.name);
      localize_cmd(cmd, loc, bun, None, true)
    }
  }
  commands
}

fn localize_cmd(
  cmd: &mut PoiseCommand,
  loc: &str,
  fb: &FluentBundle,
  parent_path: Option<&str>,
  log_missing: bool,
) {
  let path = format!("{}_{}", parent_path.unwrap_or("cmd"), cmd.name);
  // Skip trying to localize group commands
  if !cmd.subcommand_required {
    if let Some(name) = get_loc(loc, fb, &path, None, true, log_missing) {
      cmd.name_localizations.insert(loc.into(), name.into());
    }
    if let Some(desc) = get_loc(loc, fb, &path, Some("desc"), false, log_missing) {
      cmd.description_localizations.insert(loc.into(), desc.into());
    }
    for prm in &mut cmd.parameters {
      let prm_path = format!("prm_{}", &prm.name);
      if let Some(name) = get_loc(loc, fb, &path, Some(&prm_path), true, log_missing) {
        prm.name_localizations.insert(loc.into(), name.into());
      }
      if let Some(desc) =
        get_loc(loc, fb, &path, Some(&format!("{prm_path}_desc")), false, log_missing)
      {
        prm.description_localizations.insert(loc.into(), desc.into());
      }
      for cho in &mut prm.choices {
        let path = format!("cho_{}", &prm.name);
        if let Some(name) = get_loc(loc, fb, &path, Some(&cho.name), false, log_missing) {
          cho.localizations.insert(loc.into(), name.into());
        }
      }
    }
  }
  for sub in &mut cmd.subcommands {
    localize_cmd(sub, loc, fb, Some(&path), log_missing);
  }
}

fn get_loc<'a>(
  loc: &str,
  bun: &FluentBundle,
  path: &str,
  attr: Option<&str>,
  check_lowercase: bool,
  warn_missing: bool,
) -> Option<String> {
  let log_path = attr.and_then(|a| Some(format!("{path}.{a}"))).unwrap_or(path.into());
  if let Some(localized) = localize(bun, path, attr, None) {
    if !check_lowercase || localized.chars().all(char::is_lowercase) {
      return Some(localized);
    } else {
      log::error!("Locale '{loc}' contains uppercase characters in '{log_path}'")
    }
  } else if warn_missing {
    log::warn!("Locale '{loc}' is missing '{log_path}'")
  }
  return None;
}
