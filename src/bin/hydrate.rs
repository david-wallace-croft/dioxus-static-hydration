use dioxus_static_hydration::route::Route;

fn main() {
  dioxus_web::launch_with_props(
    dioxus_fullstack::router::RouteWithCfg::<Route>,
    dioxus_fullstack::prelude::get_root_props_from_document()
      .expect("Failed to get root props from document"),
    dioxus_web::Config::default().hydrate(true),
  );
}
