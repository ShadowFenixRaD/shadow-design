use dioxus::prelude::*;
use shadow_design::prelude::*;

mod components;
mod pages;

use components::Layout;
use pages::{
    AlertsPage, AnimationsPage, AttachmentCardsPage, AvatarsPage, BadgesPage, ButtonsPage, CardsPage, ChatPage,
    CheckboxPage, DividersPage, DrawerPage, DropdownPage, Home, IconsPage, InputsPage, ModalsPage,
    NavbarDemoPage, NotificationsPage, ProgressPage, SegmentedControlsPage, SelectPage,
    SkeletonsPage, SwitchPage, TabsPage, TextareaPage, ThemeBuilderPage, TooltipPage, TreesPage,
    TypographyPage, BreadcrumbsPage,
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
    #[route("/select")]
    SelectPage {},
    #[route("/textarea")]
    TextareaPage {},
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
    #[route("/breadcrumbs")]
    BreadcrumbsPage {},
    #[route("/icons")]
    IconsPage {},
    #[route("/animations")]
    AnimationsPage {},
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
    let mut is_restored = use_signal(|| false);
    use_context_provider(|| theme);

    // Resistenza ai riavvii e ai refresh: ripristino automatico da localStorage (eseguito solo una volta all'avvio)
    use_hook(|| {
        spawn(async move {
            let eval = document::eval(
                r#"
                try {
                    return (typeof window !== "undefined" && window.localStorage)
                        ? (window.localStorage.getItem("shadow_design_custom_theme") || "")
                        : "";
                } catch(e) {
                    return "";
                }
                "#,
            );
            if let Ok(val) = eval.await {
                if let Some(json_str) = val.as_str() {
                    if !json_str.is_empty() {
                        if let Ok(loaded) = serde_json::from_str::<Theme>(json_str) {
                            *theme.write() = loaded;
                        }
                    }
                }
            }
            is_restored.set(true);
        });
    });

    // Persistenza automatica di qualsiasi modifica al tema su localStorage (SC-14)
    use_effect(move || {
        if !is_restored() {
            return;
        }
        let current_theme = theme();
        if let Ok(json) = serde_json::to_string(&current_theme) {
            spawn(async move {
                let js_str = serde_json::to_string(&json).unwrap_or_default();
                let eval = document::eval(&format!(
                    r#"
                    try {{
                        if (typeof window !== "undefined" && window.localStorage) {{
                            window.localStorage.setItem("shadow_design_custom_theme", {js_str});
                        }}
                    }} catch(e) {{
                        console.warn("Storage save error:", e);
                    }}
                    "#
                ));
                let _ = eval.await;
            });
        }
    });

    // Iniezione reattiva delle variabili CSS Custom Properties senza mutazione di document::Style
    use_effect(move || {
        let css = theme().to_css_variables();
        spawn(async move {
            let eval = document::eval(&format!(
                r#"
                try {{
                    var el = document.getElementById("sd-dynamic-theme-vars");
                    if (!el) {{
                        el = document.createElement("style");
                        el.id = "sd-dynamic-theme-vars";
                        var target = document.head || document.documentElement;
                        if (target) {{
                            target.appendChild(el);
                        }}
                    }}
                    if (el) {{
                        el.textContent = `{css}`;
                    }}
                }} catch(e) {{
                    console.warn("Theme vars injection warning:", e);
                }}
                return true;
                "#
            ));
            let _ = eval.await;
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
