// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use crate::{hasher::IdHasher, *};
use futures::future::{join_all, LocalBoxFuture};
use std::{
  any::{type_name, Any, TypeId},
  collections::HashMap,
  hash::BuildHasherDefault,
};
use tokio::task::JoinHandle;

pub type ModuleMap = HashMap<TypeId, Box<dyn Any>, BuildHasherDefault<IdHasher>>;
type OptFuture = Option<JoinHandle<R>>;
pub type RtClosure = fn(&mut NekoFramework) -> Res<LocalBoxFuture<'static, Res<OptFuture>>>;

#[derive(Default)]
pub struct NekoFramework {
  pub modules: ModuleMap,
  pub runtime: Vec<RtClosure>,
}

impl NekoFramework {
  pub fn new() -> Self {
    Default::default()
  }

  pub async fn run(mut self) -> R {
    let mut handles = vec![];
    // Run all async mains and collect any handles
    for run in &mut std::mem::replace(&mut self.runtime, vec![]) {
      if let Some(handle) = run(&mut self)?.await? {
        handles.push(handle);
      }
    }
    // Await any returned handles
    for res in join_all(handles).await {
      res??
    }
    Ok(())
  }

  pub fn has<T: Module>(&mut self) -> bool {
    self.modules.get(&TypeId::of::<T>()).is_some()
  }

  /// Load a supplied module
  pub fn init<T: Module>(&mut self, mut module: T) -> Res<&mut Self> {
    log::info!("Initializing {}", std::any::type_name::<T>());
    module.on_init(self)?;
    self.modules.insert(TypeId::of::<T>(), Box::new(module));
    Ok(self)
  }

  /// Check if module is already loaded, and if not, load a default impl
  pub fn req<T: Module + Default>(&mut self) -> Res<&mut T> {
    if !self.has::<T>() {
      self.init(T::default())?;
    }

    Ok(
      self
        .modules
        .get_mut(&TypeId::of::<T>())
        .and_then(|b| b.downcast_mut::<T>())
        .ok_or(format!(
          "Required module {} is not loaded",
          type_name::<T>()
        ))?,
    )
  }

  pub fn take<T: Module>(&mut self) -> Res<T> {
    Ok(
      self
        .modules
        .remove(&TypeId::of::<T>())
        .and_then(|b| b.downcast::<T>().ok())
        .map(|b| *b)
        .ok_or(format!(
          "Required module {} is not loaded",
          type_name::<T>()
        ))?,
    )
  }
}
