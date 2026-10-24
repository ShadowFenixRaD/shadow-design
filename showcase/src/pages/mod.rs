//! Non-component pages (REVAMP §2.4) and the foundation table used by sidebar, search and home.
mod home;
mod icons;
mod not_found;
mod theme_builder;
mod tokens;
mod typography;

pub use home::Home;
pub use icons::IconsPage;
pub use not_found::NotFound;
pub use theme_builder::ThemeBuilderPage;
pub use tokens::TokensPage;
pub use typography::TypographyPage;

use crate::Route;
use shadow_design::prelude::IconKind;

pub struct FoundationPage {
    pub name: &'static str,
    pub route: Route,
    pub icon: IconKind,
    pub blurb: &'static str,
}

pub const FOUNDATION: &[FoundationPage] = &[
    FoundationPage { name: "Panoramica", route: Route::Home {}, icon: IconKind::Sparkles, blurb: "Scene reali, preset e installazione." },
    FoundationPage {
        name: "Theme Builder",
        route: Route::ThemeBuilderPage {},
        icon: IconKind::Palette,
        blurb: "Progetta il tema e copia il codice Rust.",
    },
    FoundationPage { name: "Tokens", route: Route::TokensPage {}, icon: IconKind::Layers, blurb: "Ogni variabile --sd-* resa dal vivo." },
    FoundationPage { name: "Typography", route: Route::TypographyPage {}, icon: IconKind::Code, blurb: "Kanit, Onest, Source Code Pro." },
    FoundationPage { name: "Icons", route: Route::IconsPage {}, icon: IconKind::Search, blurb: "Icone SVG native, stroke 1.5." },
];
