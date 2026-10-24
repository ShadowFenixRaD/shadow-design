use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::nav::{ComponentItem, SearchPalette, CATEGORIES, COMPONENTS};
use crate::{Route, HEADER_SVG, VERSION};

const SIDEBAR_SCROLL_JS: &str = r#"
requestAnimationFrame(() => {
  const box = document.querySelector('.sc-sidebar');
  const el = box && box.querySelector('.sd-sidebar-link--active');
  if (box && el) { box.scrollTop = Math.max(0, el.offsetTop - box.clientHeight / 2); }
});
"#;

#[component]
pub fn Layout() -> Element {
    let theme = use_theme();
    let mut sidebar_collapsed = use_signal(|| false);
    let mut drawer_open = use_signal(|| false);
    let mut search_open = use_signal(|| false);
    let nav = navigator();
    let route = use_route::<Route>();

    use_hotkey("Ctrl+K", move || search_open.set(true));

    // On every navigation (and on first load): close the mobile drawer and
    // bring the active sidebar item into view.
    use_effect(use_reactive((&route,), move |(_r,)| {
        drawer_open.set(false);
        let _ = document::eval(SIDEBAR_SCROLL_JS);
    }));

    rsx! {
        div { class: "{theme().root}",
            SearchPalette { is_open: search_open }

            header { class: "sc-topbar z-layer-header",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    class: "sc-hide-below-md",
                    aria_label: Some("Comprimi o espandi la barra laterale".to_string()),
                    onclick: move |_| sidebar_collapsed.toggle(),
                    Icon { kind: IconKind::Menu, size: 18 }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    class: "sc-only-below-md",
                    aria_label: Some("Apri la navigazione".to_string()),
                    onclick: move |_| drawer_open.set(true),
                    Icon { kind: IconKind::Menu, size: 18 }
                }

                Link { to: Route::Home {}, class: "sc-brand",
                    img { src: HEADER_SVG, class: "sc-brand__logo", alt: "Shadowfenix" }
                    span { class: "sc-brand__text",
                        span { class: "sc-brand__name", "Shadow Design" }
                        span { class: "sc-brand__tag", "by shadowfenix" }
                    }
                }
                Badge {
                    label: format!("v{VERSION}"),
                    variant: BadgeVariant::Default,
                    pill: true,
                    class: "sc-hide-below-lg".to_string(),
                }

                div { class: "sc-topbar__spacer" }

                button {
                    r#type: "button",
                    class: "sc-search",
                    aria_label: "Cerca componenti",
                    onclick: move |_| search_open.set(true),
                    Icon { kind: IconKind::Search, size: 14 }
                    span { class: "sc-search__label", "Cerca" }
                    kbd { class: "sc-kbd sc-hide-below-md", "Ctrl K" }
                }

                div { class: "sc-hide-below-xl",
                    ThemeToolbar { on_builder: move |_| { nav.push(Route::ThemeBuilderPage {}); } }
                }
            }

            div { class: "sc-toolrow",
                ThemeToolbar { on_builder: move |_| { nav.push(Route::ThemeBuilderPage {}); } }
            }

            Drawer {
                is_open: drawer_open,
                title: Some("Navigazione".to_string()),
                position: DrawerPosition::Left,
                size: DrawerSize::Sm,
                div { class: "sc-sidebar-drawer",
                    SidebarNav { collapsed: false }
                }
            }

            div { class: "sc-shell",
                aside {
                    class: if sidebar_collapsed() { "sc-sidebar sc-sidebar--collapsed" } else { "sc-sidebar" },
                    aria_label: "Navigazione componenti",
                    SidebarNav { collapsed: sidebar_collapsed() }
                }

                main { class: "{theme().content} sc-main",
                    div { class: "sc-page", Outlet::<Route> {} }
                }
            }
        }
    }
}

/// Colour swatches, radius scale and glow mode. Fixed-width segments, so the
/// toolbar never shifts when a value changes.
#[component]
fn ThemeToolbar(on_builder: EventHandler<()>) -> Element {
    let mut theme = use_theme();
    let primary = theme().primary_color;

    let mut radius = use_signal(|| radius_key(theme.peek().radius_scale).to_string());
    let mut glow = use_signal(|| glow_key(theme.peek().glow_mode).to_string());

    // Keep the segments in sync when the theme changes elsewhere (Theme Builder).
    use_effect(move || {
        let t = theme();
        let r = radius_key(t.radius_scale);
        if radius.peek().as_str() != r {
            radius.set(r.to_string());
        }
        let g = glow_key(t.glow_mode);
        if glow.peek().as_str() != g {
            glow.set(g.to_string());
        }
    });

    rsx! {
        div { class: "sc-toolbar",
            div { class: "sc-toolbar__swatches", role: "group", aria_label: "Colore primario",
                for (color , title) in [
                    (PrimaryColor::Violet, "Violet Ultraviolet"),
                    (PrimaryColor::Obsidian, "Obsidian Titanium"),
                    (PrimaryColor::Crimson, "Crimson Phoenix"),
                    (PrimaryColor::Cyan, "Cyber Cyan"),
                    (PrimaryColor::Emerald, "Emerald Matrix"),
                ] {
                    ColorSwatchButton {
                        key: "{title}",
                        color: color.clone(),
                        is_active: primary == color,
                        title,
                        onclick: {
                            let color = color.clone();
                            move |_| theme.write().apply_primary_color(color.clone())
                        },
                    }
                }
            }

            div { class: "sc-toolbar__seg sc-toolbar__seg--radius",
                SegmentedControl {
                    size: SegmentedControlSize::Sm,
                    full_width: true,
                    options: vec![
                        SegmentOption::new("sharp", "Sharp"),
                        SegmentOption::new("medium", "Medium"),
                        SegmentOption::new("soft", "Soft"),
                    ],
                    value: radius,
                    on_change: move |v: String| {
                        let scale = match v.as_str() {
                            "sharp" => RadiusScale::Sharp,
                            "soft" => RadiusScale::Soft,
                            _ => RadiusScale::Medium,
                        };
                        theme.write().apply_radius_scale(scale);
                    },
                }
            }

            div { class: "sc-toolbar__seg sc-toolbar__seg--glow",
                SegmentedControl {
                    size: SegmentedControlSize::Sm,
                    full_width: true,
                    options: vec![
                        SegmentOption::new("off", "Off"),
                        SegmentOption::new("subtle", "Subtle"),
                        SegmentOption::new("expressive", "Expressive"),
                    ],
                    value: glow,
                    on_change: move |v: String| {
                        let mode = match v.as_str() {
                            "off" => GlowMode::Off,
                            "expressive" => GlowMode::Expressive,
                            _ => GlowMode::Subtle,
                        };
                        theme.write().apply_glow_mode(mode);
                    },
                }
            }

            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Sm,
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Palette, size: 14 }
                }),
                onclick: move |_| on_builder.call(()),
                "Theme Builder"
            }
        }
    }
}

fn radius_key(r: RadiusScale) -> &'static str {
    match r {
        RadiusScale::Sharp => "sharp",
        RadiusScale::Medium => "medium",
        RadiusScale::Soft => "soft",
    }
}

fn glow_key(g: GlowMode) -> &'static str {
    match g {
        GlowMode::Off => "off",
        GlowMode::Subtle => "subtle",
        GlowMode::Expressive => "expressive",
    }
}

/// Category list generated from the unified registry, plus the theme card.
#[component]
fn SidebarNav(collapsed: bool) -> Element {
    let theme = use_theme();
    let primary = theme().primary_color;
    let radius = theme().radius_scale;
    let glow = theme().glow_mode;

    rsx! {
        for cat in CATEGORIES.iter() {
            {
                let items: Vec<&'static ComponentItem> = COMPONENTS.iter().filter(|c| c.class == cat.name).collect();
                rsx! {
                    if !items.is_empty() {
                        nav { class: "sc-nav", key: "{cat.name}", aria_label: "{cat.name}",
                            if collapsed {
                                div { class: "sc-nav__sep" }
                            } else {
                                p { class: "sc-nav__label",
                                    Icon { kind: cat.icon, size: 12 }
                                    "{cat.name}"
                                }
                            }
                            for comp in items {
                                SidebarLink {
                                    key: "{comp.name}",
                                    to: comp.route.clone(),
                                    icon: comp.icon,
                                    label: comp.name,
                                    collapsed,
                                }
                            }
                        }
                    }
                }
            }
        }

        if !collapsed {
            div { class: "sc-theme-card",
                div { class: "sc-theme-card__head",
                    span { "Tema attivo" }
                    Badge { label: "v{VERSION}", variant: BadgeVariant::Default, pill: true }
                }
                div { class: "sc-theme-card__row", span { "Preset" } strong { "{theme().name}" } }
                div { class: "sc-theme-card__row", span { "Primary" } strong { "{primary.name()}" } }
                div { class: "sc-theme-card__row", span { "Radius" } strong { "{radius.name()}" } }
                div { class: "sc-theme-card__row", span { "Glow" } strong { "{glow:?}" } }
            }
        }
    }
}

#[component]
fn ColorSwatchButton(
    color: PrimaryColor,
    is_active: bool,
    title: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let hex = color.hex().to_string();
    rsx! {
        button {
            r#type: "button",
            class: if is_active { "sc-swatch sc-swatch--active" } else { "sc-swatch" },
            style: "background-color: {hex};",
            title,
            aria_label: title,
            aria_pressed: "{is_active}",
            onclick: move |e| onclick.call(e),
        }
    }
}

#[component]
pub fn SidebarLink(
    to: Route,
    icon: IconKind,
    label: &'static str,
    #[props(default = false)] collapsed: bool,
) -> Element {
    let theme = use_theme();
    rsx! {
        Link {
            to,
            class: "{theme().sidebar_link}",
            active_class: "{theme().sidebar_link_active}",
            title: "{label}",
            Icon { kind: icon, size: 16 }
            if !collapsed {
                span { class: "truncate", "{label}" }
            }
        }
    }
}
