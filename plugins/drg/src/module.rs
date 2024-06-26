// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_core::*;
use neko_poise::*;
use neko_reqwest::*;

pub mod reqwest;

module! {
  DeepRockGalactic;

  impl on_init(fw) {
    fw.req::<Reqwest>()?;
    let poise = fw.req::<Poise>()?;
    //poise.add_command(drg());
  }
}

fn mutator_icon(mutator: String) -> String {
  match mutator.as_str() {
    "Critical Weakness" => "<:mutator_critical_weakness:1152800323967135856>",
    "Double XP" => "<:mutator_double_xp:1152800395341598810>",
    "Gold Rush" => "<:mutator_gold_rush:1152800396918661130>",
    "Golden Bugs" => "<:mutator_golden_bugs:1152800399150035095>",
    "Low Gravity" => "<:mutator_low_gravity:1152800400651591760>",
    "Mineral Mania" => "<:mutator_mineral_mania:1152800403407257601>",
    "Rich Atmosphere" => "<:mutator_rich_atmosphere:1152800405005275186>",
    "Volatile Guts" => "<:mutator_volatile_guts:1152800406691381340> ",
    _ => "",
  }
  .into()
}

fn warning_icon(warning: String) -> String {
  match warning.as_str() {
    "Cave Leech Cluster" => "<:warning_cave_leech_cluster:1152827797476233266>",
    "Elite Threat" => "<:warning_elite_threat:1152827795039342622>",
    "Exploder Infestation" => "<:warning_exploder_infestation:1152827792636002375>",
    "Haunted Cave" => "<:warning_haunted_cave:1152827790761136198>",
    "Lethal Enemies" => "<:warning_lethal_enemies:1152827787493769286>",
    "Low Oxygen" => "<:warning_low_oxygen:1152827817722118276>",
    "Mactera Plague" => "<:warning_mactera_plague:1152827814538645618>",
    "Parasites" => "<:warning_parasites:1152827811933978756>",
    "Regenerative Bugs" => "<:warning_regenerative_bugs:1152827806976311437>",
    "Rival Presence" => "<:warning_rival_presence:1152827803121762356>",
    "Shield Disruption" => "<:warning_shield_disruption:1152827801330798592>",
    "Swarmageddon" => "<:warning_swarmageddon:1152827799715975198>",
    _ => "<:warning_placeholder:1152827809983631390>",
  }
  .into()
}
//impl std::fmt::Display for Variant {
//  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//    let stages = &self
//      .stages
//      .iter()
//      .map(|s| {
//        let warning = s
//          .warning
//          .as_ref()
//          .and_then(|i| Some(warning_icon(i.into())))
//          .unwrap_or("".into());
//        let mutator = s
//          .mutator
//          .as_ref()
//          .and_then(|i| Some(mutator_icon(i.into())))
//          .unwrap_or("".into());
//        format!(
//          "Stage {}: {warning}{mutator}\n- {}\n- {}",
//          s.id, s.primary, s.secondary
//        )
//      })
//      .collect::<Vec<String>>()
//      .join("\n");
//    write!(f, "Seed: `{}`\n{stages}", self.seed)
//  }
//}
