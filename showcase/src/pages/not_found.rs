use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::registry;
use crate::Route;

/// Slugs suggested on the 404 page: the components people look for first.
const SUGGESTED: [&str; 5] = ["buttons", "inputs", "select", "tables", "modals"];

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let navigator = use_navigator();
    let path = format!("/{}", segments.join("/"));
    rsx! {
        document::Title { "Pagina non trovata · Shadow Design" }
        div { class: "sc-story",
            header { class: "sc-story__head",
                span { class: "sc-eyebrow", "Errore 404" }
                h1 { class: "sc-title", "Pagina non trovata" }
            }
            Alert { variant: AlertVariant::Warning, title: Some("Nessuna pagina a questo indirizzo".to_string()),
                "L'indirizzo {path} non corrisponde a un componente né a una pagina dello showcase."
            }
            Group { gap: SpacingToken::Sm,
                Button {
                    left_icon: Some(rsx! {
                        Icon { kind: IconKind::ChevronLeft, size: 15 }
                    }),
                    onclick: move |_| {
                        navigator.push(Route::Home {});
                    },
                    "Torna alla panoramica"
                }
            }
            section { class: "sc-section",
                crate::story::sections::SectionHead { eyebrow: "Suggeriti", title: "Forse cercavi" }
                div { class: "sc-related",
                    for slug in SUGGESTED {
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
}
