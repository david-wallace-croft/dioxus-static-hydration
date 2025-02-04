use crate::route::Route;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Nav() -> Element {
  rsx! {
    div {
      class: "box",
    span {
      class: "drum",
    "CroftSoft Dioxus Static Hydration",
    }
    nav {
    ul {
    li {
    Link {
      to: Route::Home {},
    "Home"
    }
    }
    li {
    Link {
      to: Route::About {},
    "About"
    }
    }
    li {
    a {
      href: "/merged/",
    "Merged"
    }
    }
    li {
    Link {
      to: Route::Colophon {},
    "Colophon"
    }
    }
    }
    }
    }
  }
}
