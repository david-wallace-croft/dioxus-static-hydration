use dioxus::prelude::*;
use dioxus_fullstack::{launch, prelude::*};
use dioxus_router::prelude::*;
use dioxus_static_hydration::launch;
use dioxus_static_hydration::route::Route;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
  pre_cache_static_routes_with_props(
    &ServeConfigBuilder::new_with_router(
      dioxus_fullstack::router::FullstackRouterConfig::<Route>::default(),
    )
    .assets_path("dist")
    .incremental(IncrementalRendererConfig::default().static_dir("dist"))
    .build(),
  )
  .await
  .unwrap();
}
