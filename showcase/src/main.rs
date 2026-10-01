use dioxus::prelude::*;
use shadow_design::prelude::*;

mod components;
mod pages;

use components::Layout;
use pages::{
    AlertsPage, BadgesPage, ButtonsPage, CardsPage, CheckboxPage, DrawerPage, DropdownPage, Home,
    InputsPage, ModalsPage, NavbarDemoPage, NotificationsPage, SwitchPage, TabsPage, TooltipPage,
    TypographyPage,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/typography")]
    TypographyPage {},
    #[route("/buttons")]
    ButtonsPage {},
    #[route("/cards")]
    CardsPage {},
    #[route("/inputs")]
    InputsPage {},
    #[route("/checkbox")]
    CheckboxPage {},
    #[route("/badges")]
    BadgesPage {},
    #[route("/alerts")]
    AlertsPage {},
    #[route("/modals")]
    ModalsPage {},
    #[route("/navbar")]
    NavbarDemoPage {},
    #[route("/notifications")]
    NotificationsPage {},
    #[route("/tabs")]
    TabsPage {},
    #[route("/switch")]
    SwitchPage {},
    #[route("/tooltip")]
    TooltipPage {},
    #[route("/drawer")]
    DrawerPage {},
    #[route("/dropdown")]
    DropdownPage {},
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const STYLES: Asset = shadow_design::STYLES;
pub const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link { rel: "stylesheet", href: STYLES }
        NotificationProvider {
            Router::<Route> {}
        }
    }
}
