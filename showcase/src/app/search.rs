//! Search palette (Ctrl+K): stories, foundation pages and theme actions (REVAMP §2.2).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::app::theme_menu::{glow_from_key, radius_from_key, PRESETS};
use crate::pages::FOUNDATION;
use crate::story::registry;
use crate::Route;

const THEME_CATEGORY: &str = "Tema";

fn actions() -> Vec<CommandAction> {
    let mut out: Vec<CommandAction> = FOUNDATION
        .iter()
        .enumerate()
        .map(|(i, p)| {
            CommandAction::new(format!("page:{i}"), p.name)
                .with_description(p.blurb)
                .with_category("Fondamenta")
                .with_icon(p.icon)
        })
        .collect();

    out.extend(registry::all().iter().map(|s| {
        CommandAction::new(format!("story:{}", s.slug), s.name)
            .with_description(s.tagline)
            .with_category(s.category.label())
            .with_icon(s.icon)
            .with_keywords(s.keywords.iter().chain(s.features.iter()).copied())
    }));

    for (color, name) in PRESETS {
        out.push(
            CommandAction::new(format!("theme:preset:{}", color.name().to_lowercase()), format!("Accento {name}"))
                .with_description("Applica il preset a tutto lo showcase")
                .with_category(THEME_CATEGORY)
                .with_icon(IconKind::Palette)
                .with_keywords(["tema", "theme", "colore", "accent", "preset"]),
        );
    }
    for (key, label) in [("sharp", "Sharp"), ("medium", "Medium"), ("soft", "Soft")] {
        out.push(
            CommandAction::new(format!("theme:radius:{key}"), format!("Raggio {label}"))
                .with_description("Scala dei raggi --sd-radius-*")
                .with_category(THEME_CATEGORY)
                .with_icon(IconKind::Box)
                .with_keywords(["radius", "raggio", "angoli", "tema"]),
        );
    }
    for (key, label) in [("off", "Off"), ("subtle", "Subtle"), ("expressive", "Expressive")] {
        out.push(
            CommandAction::new(format!("theme:glow:{key}"), format!("Glow {label}"))
                .with_description("Intensità --sd-glow-strength")
                .with_category(THEME_CATEGORY)
                .with_icon(IconKind::Sparkles)
                .with_keywords(["glow", "bagliore", "tema"]),
        );
    }
    out
}

#[component]
pub fn SearchPalette(is_open: Signal<bool>) -> Element {
    let navigator = use_navigator();
    let mut theme = use_theme();
    let mut toast = use_notification();
    // The registry is static: build the action list once.
    let all = use_hook(actions);

    rsx! {
        CommandPalette {
            opened: is_open,
            actions: all,
            placeholder: "Cerca componenti, pagine o azioni del tema…".to_string(),
            on_select: move |action: CommandAction| {
                let id = action.id.as_str();
                if let Some(slug) = id.strip_prefix("story:") {
                    if let Some(s) = registry::find(slug) {
                        navigator.push(Route::story(s.slug));
                    }
                } else if let Some(i) = id.strip_prefix("page:").and_then(|i| i.parse::<usize>().ok()) {
                    if let Some(p) = FOUNDATION.get(i) {
                        navigator.push(p.route.clone());
                    }
                } else if let Some(name) = id.strip_prefix("theme:preset:") {
                    if let Some((color, label)) = PRESETS.into_iter().find(|(c, _)| c.name().to_lowercase() == name) {
                        theme.write().apply_primary_color(color);
                        toast.success(format!("Accento {label} applicato"));
                    }
                } else if let Some(key) = id.strip_prefix("theme:radius:") {
                    theme.write().apply_radius_scale(radius_from_key(key));
                    toast.success(format!("Raggio {key} applicato"));
                } else if let Some(key) = id.strip_prefix("theme:glow:") {
                    theme.write().apply_glow_mode(glow_from_key(key));
                    toast.success(format!("Glow {key} applicato"));
                }
            },
        }
    }
}
