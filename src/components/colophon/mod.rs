use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Colophon() -> Element {
  rsx! {
    div {
      class: "box",
    h1 { "Colophon" }
    ul {
    p {
    "This website was created using the Rust library ",
    a {
      href: "https://dioxuslabs.com/",
      target: "_blank",
    "Dioxus",
    },
    "."
    }
    p {
    "The open source repository for this website is hosted on GitHub:",
    br { },
    a {
      href: "https://github.com/david-wallace-croft/dioxus-static-hydration",
      target: "_blank",
    "https://github.com/david-wallace-croft/dioxus-static-hydration"
    }
    }
    }
    }
  }
}
