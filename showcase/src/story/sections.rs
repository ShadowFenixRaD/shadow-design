//! Section vocabulary shared by every page: story template, home, tokens, builder.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use super::registry;
use super::schema::{Example, PropGroup, Recipe, Span};
use crate::Route;

/// Eyebrow + Kanit title + optional one-line description. The same head on every page.
#[component]
pub fn SectionHead(eyebrow: &'static str, title: &'static str, #[props(default)] desc: Option<&'static str>) -> Element {
    rsx! {
        div { class: "sc-section__head",
            span { class: "sc-eyebrow", "{eyebrow}" }
            h2 { class: "sc-section__title", "{title}" }
            if let Some(d) = desc {
                p { class: "sc-section__desc", "{d}" }
            }
        }
    }
}

#[component]
pub fn ExamplesSection(examples: Vec<Example>) -> Element {
    rsx! {
        section { class: "sc-section", id: "esempi",
            SectionHead {
                eyebrow: "Esempi",
                title: "Varianti e contesto",
                desc: Some("Ogni esempio è codice reale: aprilo con \"Codice\"."),
            }
            div { class: "sc-examples",
                for e in examples {
                    ExampleCard { key: "{e.id}", example: e }
                }
            }
        }
    }
}

#[component]
fn ExampleCard(example: Example) -> Element {
    let mut show_code = use_signal(|| false);
    let class = if example.span == Span::Full { "sc-example sc-example--full" } else { "sc-example" };
    rsx! {
        Card {
            class: class.to_string(),
            title: Some(example.title.to_string()),
            subtitle: Some(example.description.to_string()),
            buttons: Some(rsx! {
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    aria_pressed: Some(show_code()),
                    left_icon: Some(rsx! {
                        Icon { kind: IconKind::Code, size: 14 }
                    }),
                    onclick: move |_| show_code.toggle(),
                    "Codice"
                }
            }),
            div { class: "sc-example__stage", {(example.render)()} }
            Collapsible { open: show_code,
                CodeBlock { code: example.code.to_string(), language: "rsx", with_line_numbers: false }
            }
        }
    }
}

#[component]
pub fn NotesSection(recipe: Option<Recipe>, a11y: &'static [&'static str]) -> Element {
    rsx! {
        section { class: "sc-section", id: "ricetta",
            SectionHead { eyebrow: "Lit Glass", title: "Ricetta e accessibilità" }
            div { class: "sc-notes",
                if let Some(r) = recipe {
                    Card { title: Some("Ricetta".to_string()), subtitle: Some(r.recipe.to_string()),
                        dl { class: "sc-dl",
                            dt { "Profondità" }
                            dd { "{r.depth}" }
                            dt { "Token" }
                            dd { class: "sc-dl__chips",
                                for t in r.tokens {
                                    Code { key: "{t}", "{t}" }
                                }
                            }
                            dt { "Note" }
                            dd { "{r.notes}" }
                        }
                    }
                }
                if !a11y.is_empty() {
                    Card { title: Some("Accessibilità".to_string()),
                        List { spacing: ListSpacing::Sm,
                            for (i , item) in a11y.iter().enumerate() {
                                ListItem {
                                    key: "{i}",
                                    icon: Some(rsx! {
                                        Icon { kind: IconKind::CheckCircle, size: 14 }
                                    }),
                                    "{item}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PropsSection(groups: Vec<PropGroup>) -> Element {
    let active = use_signal(|| groups.first().map(|g| g.component.to_string()).unwrap_or_default());
    rsx! {
        section { class: "sc-section", id: "api",
            SectionHead { eyebrow: "API", title: "Props" }
            if groups.len() == 1 {
                PropsTable { group: groups[0].clone() }
            } else {
                Tabs {
                    TabList { aria_label: Some("Componenti documentati".to_string()),
                        for g in groups.iter() {
                            Tab { key: "{g.component}", value: g.component.to_string(), active, "{g.component}" }
                        }
                    }
                    for g in groups.iter() {
                        TabPanel { key: "{g.component}", value: g.component.to_string(), active,
                            PropsTable { group: g.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PropsTable(group: PropGroup) -> Element {
    rsx! {
        div { class: "sc-props",
            Table { hoverable: false, bordered: true,
                TableHeader {
                    TableRow {
                        TableCell { is_header: true, "Prop" }
                        TableCell { is_header: true, "Tipo" }
                        TableCell { is_header: true, "Default" }
                        TableCell { is_header: true, "Descrizione" }
                    }
                }
                TableBody {
                    for row in group.rows {
                        TableRow { key: "{row.name}",
                            TableCell { class: "sc-props__name", "{row.name}" }
                            TableCell { class: "sc-props__type", "{row.ty}" }
                            TableCell { class: "sc-props__default", "{row.default}" }
                            TableCell { class: "sc-props__desc", "{row.doc}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RelatedSection(related: &'static [&'static str]) -> Element {
    let navigator = use_navigator();
    rsx! {
        section { class: "sc-section", id: "correlati",
            SectionHead { eyebrow: "Correlati", title: "Vedi anche" }
            div { class: "sc-related",
                for slug in related {
                    if let Some(s) = registry::find(slug) {
                        NavLink {
                            key: "{s.slug}",
                            label: s.name.to_string(),
                            description: Some(s.tagline.to_string()),
                            icon: Some(SvgIcon::kind(s.icon)),
                            on_click: move |_| {
                                navigator.push(Route::story(s.slug));
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PrevNext(prev: Option<(&'static str, &'static str)>, next: Option<(&'static str, &'static str)>) -> Element {
    let navigator = use_navigator();
    rsx! {
        nav { class: "sc-prevnext", aria_label: "Storie adiacenti",
            if let Some((slug, name)) = prev {
                Button {
                    variant: ButtonVariant::Secondary,
                    left_icon: Some(rsx! {
                        Icon { kind: IconKind::ChevronLeft, size: 14 }
                    }),
                    onclick: move |_| {
                        navigator.push(Route::story(slug));
                    },
                    "{name}"
                }
            } else {
                span {}
            }
            if let Some((slug, name)) = next {
                Button {
                    variant: ButtonVariant::Secondary,
                    right_icon: Some(rsx! {
                        Icon { kind: IconKind::ChevronRight, size: 14 }
                    }),
                    onclick: move |_| {
                        navigator.push(Route::story(slug));
                    },
                    "{name}"
                }
            }
        }
    }
}

/// STATIC layout tile for Grid/Group/Stack/Masonry/Center/AspectRatio demos. Never looks clickable.
#[component]
pub fn DemoCell(
    #[props(default)] index: Option<usize>,
    #[props(default)] label: Option<String>,
    #[props(default)] tall: bool,
    #[props(default)] short: bool,
    #[props(default)] accent: bool,
) -> Element {
    let tall = if tall { "sc-demo-cell--tall" } else { "" };
    let short = if short { "sc-demo-cell--short" } else { "" };
    let accent = if accent { "sc-demo-cell--accent" } else { "" };
    rsx! {
        div { class: "sc-demo-cell {tall} {short} {accent}",
            if let Some(i) = index {
                span { class: "sc-demo-cell__index", "{i}" }
            }
            if let Some(l) = label {
                span { "{l}" }
            }
        }
    }
}
