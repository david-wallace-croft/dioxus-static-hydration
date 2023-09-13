use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Footer(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    p {
      dangerous_inner_html: "&copy;",
      " 2023 "
      a {
        href: "https://www.croftsoft.com/",
        target: "_blank",
        "CroftSoft Inc"
      }
    }
  }
  }
}
