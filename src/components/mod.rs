use dioxus::prelude::*;

mod favorites;
mod nav;
mod view;
mod router;

pub use favorites::*;
pub use nav::*;
pub use view::*;
pub use router::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
      document::Stylesheet { href: MAIN_CSS }
      Router::<router::Route> {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
      div { id: "title",
        h1 { "HotDog  🌭" }
      }
    }
}
