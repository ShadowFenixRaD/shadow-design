//! The one page template that renders every Story (ADR-0006 §1).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use super::controls::KnobPanel;
use super::knobs::{self, Knobs};
use super::registry;
use super::schema::{Playground, Stage, Status, Story};
use super::sections::{ExamplesSection, NotesSection, PrevNext, PropsSection, RelatedSection, SectionHead};
use crate::app::theme_menu::ThemeSwatches;

#[component]
pub fn StoryPage(slug: String) -> Element {
    match registry::find(&slug) {
        // `key` forces a fresh scope per story: knob signals never leak between stories.
        Some(story) => rsx! {
            StoryView { key: "{story.slug}", story: story.clone() }
        },
        None => rsx! {
            crate::pages::NotFound { segments: vec![slug] }
        },
    }
}

#[component]
fn StoryView(story: Story) -> Element {
    let (prev, next) = registry::neighbours(story.slug);
    let is_stub = story.status == Status::Stub;
    rsx! {
        document::Title { "{story.name} · Shadow Design" }
        article { class: "sc-story",
            StoryHead { story: story.clone() }
            if is_stub {
                Alert { variant: AlertVariant::Info, title: Some("Storia in migrazione".to_string()),
                    "Questa pagina viene riscritta sul nuovo template. Il componente è già disponibile nella libreria."
                }
            }
            if let Some(pg) = story.playground.clone() {
                LivePlayground { playground: pg }
            }
            if !story.examples.is_empty() {
                ExamplesSection { examples: story.examples.clone() }
            }
            if story.recipe.is_some() || !story.a11y.is_empty() {
                NotesSection { recipe: story.recipe, a11y: story.a11y }
            }
            if !story.props.is_empty() {
                PropsSection { groups: story.props.clone() }
            }
            if !story.related.is_empty() {
                RelatedSection { related: story.related }
            }
            PrevNext { prev: prev.map(|s| (s.slug, s.name)), next: next.map(|s| (s.slug, s.name)) }
        }
    }
}

#[component]
fn StoryHead(story: Story) -> Element {
    let cargo = story.cargo_line();
    rsx! {
        header { class: "sc-story__head",
            Breadcrumbs {
                items: Some(vec![
                    BreadcrumbData::new("Componenti"),
                    BreadcrumbData::new(story.category.label()),
                    BreadcrumbData::new(story.name),
                ]),
            }
            div { class: "sc-story__title-row",
                div { class: "sc-icon-tile",
                    Icon { kind: story.icon, size: 22 }
                }
                div { class: "sc-story__title-col",
                    h1 { class: "sc-title", "{story.name}" }
                    p { class: "sc-lede", "{story.tagline}" }
                }
                div { class: "sc-story__meta",
                    for f in story.features {
                        Badge { key: "{f}", label: f.to_string(), pill: true }
                    }
                    if story.status == Status::Beta {
                        Badge { label: "Beta".to_string(), variant: BadgeVariant::Warning, pill: true }
                    }
                }
            }
            CodeBlock {
                code: cargo,
                language: "toml",
                filename: Some("Cargo.toml".to_string()),
                with_line_numbers: false,
                class: "sc-story__cargo".to_string(),
            }
        }
    }
}

#[component]
fn LivePlayground(playground: Playground) -> Element {
    let knobs = use_signal(|| Knobs::from_controls(&playground.controls));
    let changed = use_signal(|| None::<&'static str>);
    let current = knobs();
    let generated = knobs::generate(&playground, &current, changed());
    let stage_class = match playground.stage {
        Stage::Center => "sc-stage",
        Stage::Start => "sc-stage sc-stage--start",
        Stage::Form => "sc-stage sc-stage--form",
        Stage::Fill => "sc-stage sc-stage--fill",
    };
    let file = format!("{}.rs", playground.component.to_lowercase());
    rsx! {
        section { class: "sc-section", id: "playground",
            div { class: "sc-section__head-row",
                SectionHead {
                    eyebrow: "Playground",
                    title: "Prova dal vivo",
                    desc: Some("Ogni controllo aggiorna il componente e il codice qui sotto."),
                }
                ThemeSwatches { compact: true }
            }
            div { class: "sc-playground",
                div { class: "sc-playground__stage",
                    div { class: "{stage_class}", {(playground.render)(&current)} }
                    CodeBlock {
                        code: generated.code,
                        language: "rsx",
                        filename: Some(file),
                        highlight_lines: generated.highlight,
                    }
                }
                if !playground.controls.is_empty() {
                    aside {
                        class: "sc-playground__knobs",
                        aria_label: "Controlli del playground",
                        KnobPanel { controls: playground.controls.clone(), knobs, changed }
                    }
                }
            }
        }
    }
}
