use ::dioxus::logger;
use ::dioxus::prelude::*;
use ::dioxus_static_hydration::route::Route;
use ::tracing::{Level, info};

#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
  Ok(
    Route::static_routes()
      .into_iter()
      .map(|route| route.to_string())
      .collect::<Vec<_>>(),
  )
}

fn main() {
  logger::init(Level::INFO).expect("Failed to initialize logger");

  info!(
    "CroftSoft Dioxus Static Hydration v{}",
    env!("CARGO_PKG_VERSION")
  );

  LaunchBuilder::new()
    .with_cfg(server_only! {
      ServeConfig::builder()
        // turn on incremental site generation with the .incremental() method
        .incremental(IncrementalRendererConfig::new())
        .build()
        .unwrap()
    })
    .launch(|| {
      rsx! {
        Router::<Route> {}
      }
    })
}
