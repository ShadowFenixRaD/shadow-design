use dioxus::prelude::*;
use shadow_design::prelude::*;
pub use shadow_design::VERSION;

mod app;
mod components;
mod pages;
mod scenes;
mod stories;
mod story;

use app::Layout;
use pages::{Home, IconsPage, NotFound, ThemeBuilderPage, TokensPage, TypographyPage};
use story::page::StoryPage;

/// Routes (ADR-0006 §2): five static pages, one dynamic story route for every component
/// (`/buttons`, `/date-picker`, … all resolve through the registry) and a catch-all.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/theme-builder")]
        ThemeBuilderPage {},
        #[route("/tokens")]
        TokensPage {},
        #[route("/typography")]
        TypographyPage {},
        #[route("/icons")]
        IconsPage {},
        #[route("/:slug")]
        StoryPage { slug: String },
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

impl Route {
    pub fn story(slug: &str) -> Self {
        Route::StoryPage { slug: slug.to_string() }
    }
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const STYLES: Asset = shadow_design::STYLES;
/// Showcase chrome (.sc-* classes), loaded after the library stylesheet.
pub const SHOWCASE_STYLES: Asset = asset!("/assets/showcase.scss");
pub const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let locale = use_signal(shadow_design::locale::Locale::italian);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link { rel: "stylesheet", href: STYLES }
        document::Link { rel: "stylesheet", href: SHOWCASE_STYLES }
        shadow_design::locale::LocaleProvider { locale: Some(locale),
            ThemeProvider { persist_key: "shadow_design_custom_theme".to_string(),
                NotificationProvider { Router::<Route> {} }
            }
        }
    }
}
