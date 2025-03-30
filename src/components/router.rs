use dioxus::prelude::*;

use crate::components::{DogView, NavBar, Favorites};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,
}