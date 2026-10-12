use dioxus::prelude::*;
use shadow_design::prelude::*;

mod components;
mod pages;

use components::Layout;
use pages::{
    AlertsPage, AttachmentCardsPage, AvatarsPage, BadgesPage, ButtonsPage, CardsPage, ChatPage,
    CheckboxPage, DividersPage, DrawerPage, DropdownPage, Home, InputsPage, ModalsPage,
    NavbarDemoPage, NotificationsPage, ProgressPage, SegmentedControlsPage, SkeletonsPage,
    SwitchPage, TabsPage, ThemeBuilderPage, TooltipPage, TreesPage, TypographyPage,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/theme-builder")]
    ThemeBuilderPage {},
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
    #[route("/avatars")]
    AvatarsPage {},
    #[route("/chat")]
    ChatPage {},
    #[route("/progress")]
    ProgressPage {},
    #[route("/attachment-cards")]
    AttachmentCardsPage {},
    #[route("/segmented-controls")]
    SegmentedControlsPage {},
    #[route("/dividers")]
    DividersPage {},
    #[route("/trees")]
    TreesPage {},
    #[route("/skeletons")]
    SkeletonsPage {},
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const STYLES: Asset = shadow_design::STYLES;
pub const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut theme = use_signal(Theme::default);
    use_context_provider(|| theme);

    // Resistenza ai riavvii e ai refresh: ripristino automatico da localStorage
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                r#"
                try {
                    let saved = localStorage.getItem("shadow_design_custom_theme");
                    dioxus.send(saved ? saved : "");
                } catch(e) {
                    dioxus.send("");
                }
                "#,
            );
            if let Ok(json_str) = eval.recv::<String>().await {
                if !json_str.is_empty() {
                    if let Ok(loaded) = serde_json::from_str::<Theme>(&json_str) {
                        *theme.write() = loaded;
                    }
                }
            }
        });
    });

    // Iniezione reattiva delle variabili CSS Custom Properties senza mutazione di document::Style
    use_effect(move || {
        let css = theme().to_css_variables();
        spawn(async move {
            let js = format!(
                r#"
                let el = document.getElementById("sd-dynamic-theme-vars");
                if (!el) {{
                    el = document.createElement("style");
                    el.id = "sd-dynamic-theme-vars";
                    document.head.appendChild(el);
                }}
                el.textContent = `{css}`;
                "#
            );
            let _ = document::eval(&js).await;
        });
    });

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
