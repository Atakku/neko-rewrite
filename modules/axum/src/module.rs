// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use axum::{Router, Server};
use futures::future::BoxFuture;
use neko_core::*;
use std::net::Ipv4Addr;

pub type Route = fn(Router) -> BoxFuture<'static, Res<Router>>;

pub use axum;

module! {
  Axum {
    routes: Vec<Route>,
    port: u16 = get_env!("HTTP_PORT", 8080, u16),
  }

  impl on_runtime(axum) {
    let mut router = Router::new();
    for route in axum.routes {
      router = route(router).await?;
    }
    future!({
      Server::bind(&(Ipv4Addr::UNSPECIFIED, axum.port).into())
        .serve(router.into_make_service())
        .await?;
      Ok(())
    })
  }

  // TODO: resolve the issue with overlap
  fn add_route(&mut self, route: Route) {
    self.routes.push(route);
  }
}
