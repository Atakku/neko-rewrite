// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use crate::*;
use std::any::Any;

pub trait Module: Any {
  fn on_init(&mut self, fw: &mut NekoFramework) -> R;
}

#[macro_export]
macro_rules! module {
  (
    $(#[$m:meta])*
    $module:ident;
    $($tail:tt)*
  ) => {
    $(#[$m])*
    #[derive(Default)]
    pub struct $module;
    module!(@internal $module, $($tail)*);
  };
  (
    $(#[$m:meta])*
    $module:ident {
      $($mpv:vis $mpn:ident: $mpt:ty $(= $mpd:expr)?),*$(,)?
    }
    $($tail:tt)*
  ) => {
    $(#[$m])*
    pub struct $module {
      $($mpv $mpn: $mpt),*
    }
    impl Default for $module {
      fn default() -> Self {
        Self {
          $($mpn: this_or_that!(($($mpd)?) or (Default::default()))),*
        }
      }
    }
    module!(@internal $module, $($tail)*);
  };
  (@internal $module:ident,
    $(impl on_init($init_fw:tt) $init_block:block)?
    $(impl on_runtime($rt_self:tt) $rt_block:block)?
    $($(fn $fn_name:ident$(<$fn_gnn:tt:$fn_gnt:tt>)?($($fn_param:tt)*) $(-> $fn_ty:ty)? $fn_block:block)+)?
  ) => {
    impl $crate::Module for $module {
      fn on_init(&mut self, this_or_that!(($($init_fw)?) or (fw)): &mut $crate::NekoFramework) -> $crate::R {
        $($init_block)?
        let fw = this_or_that!(($($init_fw)?) or (fw));
        $(module!(@internal rt, fw, |$rt_self| $rt_block))?;
        Ok(())
      }
    }

    $(
      impl $module {
        $(pub fn $fn_name$(<$fn_gnn : $fn_gnt>)?($($fn_param)*) $(-> $fn_ty)? $fn_block)*
      }
    )?
  };
  (@internal rt, $fw:ident, |$m:tt| $block:block) => {
    $fw.runtime.push(|fw| {
      let $m = fw.take::<Self>()?;
      Ok(Box::pin(async move $block))
    });
  }
}

#[macro_export]
macro_rules! future {
  ($block:block) => {
    Ok(Some($crate::tokio::spawn(async move $block)))
  }
}
