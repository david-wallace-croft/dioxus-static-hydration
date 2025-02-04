use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Home() -> Element {
  let mut count: Signal<isize> = use_signal(|| 0);

  rsx! {
    div {
      class: "app-home box",
    h1 {
    "CroftSoft Dioxus Static Hydration"
    }
    div {
    p {
    "High-Five counter: {count}"
    }
    button {
      onclick: move |_| count += 1, "Up high!"
    }
    button {
      onclick: move |_| count -= 1, "Down low!"
    }
    }
    }
  }
}
