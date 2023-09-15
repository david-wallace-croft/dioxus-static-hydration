use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  let mut count = use_state(cx, || 0);
  render! {
    div {
      class: "app-home box",
    h1 {
      "CroftSoft Dioxus Static Hydration Prototype"
    }
    div {
      p { "High-Five counter: {count}" }
      button { onclick: move |_| count += 1, "Up high!" }
      button { onclick: move |_| count -= 1, "Down low!" }
    }
  }
  }
}
