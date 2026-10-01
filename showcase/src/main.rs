use dioxus::prelude::*;

mod components;
mod pages;

use components::Layout;
use pages::{
    AlertsPage, BadgesPage, ButtonsPage, CardsPage, Home, InputsPage, ModalsPage, NavbarDemoPage,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/buttons")]
    ButtonsPage {},
    #[route("/cards")]
    CardsPage {},
    #[route("/inputs")]
    InputsPage {},
    #[route("/badges")]
    BadgesPage {},
    #[route("/alerts")]
    AlertsPage {},
    #[route("/modals")]
    ModalsPage {},
    #[route("/navbar")]
    NavbarDemoPage {},
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const HEADER_SVG: Asset = asset!("/assets/header.svg");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
