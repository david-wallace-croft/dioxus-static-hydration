use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn About(cx: Scope) -> Element {
  let mut count = use_state(cx, || 0);
  render! {
    div {
      class: "box",
    h1 {
      "About"
    }
    p {
      "It also works on a sub-page."
    }
    div {
      p { "High-Five counter: {count}" }
      button { onclick: move |_| count += 1, "Up high!" }
      button { onclick: move |_| count -= 1, "Down low!" }
    }
    }
  }
}
