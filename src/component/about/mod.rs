use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn About() -> Element {
  let mut count: Signal<isize> = use_signal(|| 0);
  rsx! {
    div {
      class: "box",
    h1 {
    "About"
    }
    p {
    "It also works on a sub-page."
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
