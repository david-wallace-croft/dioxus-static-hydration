use dioxus_static_hydration::launch;
#[cfg(feature = "hydrate")]
use dioxus_static_hydration::route::Route;

// Generate all routes and output them to the docs path
#[cfg(feature = "prerender")]
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

#[cfg(feature = "hydrate")]
fn main() {
  dioxus_web::launch_with_props(
    dioxus_fullstack::router::RouteWithCfg::<Route>,
    dioxus_fullstack::prelude::get_root_props_from_document()
      .expect("Failed to get root props from document"),
    dioxus_web::Config::default().hydrate(true),
  );
}

#[cfg(not(feature = "hydrate"))]
fn main() {
  launch();
}
