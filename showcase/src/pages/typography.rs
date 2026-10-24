use crate::components::{Stage, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn TypographyPage() -> Element {
    let props = vec![
        PropRow {
            name: "order",
            r#type: "HeadingOrder (H1..H6)",
            default: "HeadingOrder::H1",
            description: "Livello semantico del titolo con tracking ottico calibrato",
        },
        PropRow {
            name: "size",
            r#type: "TextSize (Xs..Xl)",
            default: "TextSize::Md",
            description: "Scala dimensionale del corpo del testo",
        },
        PropRow {
            name: "dimmed",
            r#type: "bool",
            default: "false",
            description: "Applica il colore attenuato per metadati e spiegazioni",
        },
    ];

    let code_snippet = r#"// Intestazioni con gerarchia ottica:
Heading { order: HeadingOrder::H1, "Heading di Primo Livello" }
Heading { order: HeadingOrder::H3, "Sottotitolo di Sezione" }

// Testo con varianti di dimensione:
Text { size: TextSize::Md, "Testo normale per descrizioni e paragrafi." }
Text { size: TextSize::Sm, dimmed: true, "Metadato o etichetta secondaria attenuata." }

// Badge di tasti di scelta rapida e codice inline:
Kbd { "⌘K" }
Code { "cargo add shadow-design" }"#
        .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Typography".to_string(),
                description: "Componenti per gerarchia testuale, scorciatoie da tastiera e codice inline basati sui font Geist e Geist Mono.".to_string(),
            }

            FeatureBanner { feature_name: "typography" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage { start: true, wide: true,
                        div { class: "sc-stack sc-stack--loose w-full",
                            div { class: "sc-stack",
                                Heading { order: HeadingOrder::H1, "Titolo H1 Display" }
                                Heading { order: HeadingOrder::H2, "Titolo H2 Section" }
                                Heading { order: HeadingOrder::H3, "Titolo H3 Card" }
                                Heading { order: HeadingOrder::H4, "Titolo H4 Subsection" }
                            }
                            hr { class: "sc-control-divider" }
                            div { class: "sc-stack",
                                Text { size: TextSize::Lg, "Paragrafo Large: eccellente leggibilità su display Retina ad alta densità." }
                                Text { size: TextSize::Md, "Paragrafo Standard: progettato con il font Geist Sans per un'esperienza di lettura pulita ed essenziale." }
                                Text { size: TextSize::Sm, dimmed: true, "Testo attenuato (dimmed): ideale per note a piè di pagina e timestamp." }
                            }
                            hr { class: "sc-control-divider" }
                            div { class: "sc-actions",
                                div { class: "sc-actions",
                                    span { class: "sc-muted-text", "Scorciatoia:" }
                                    Kbd { "⌘" }
                                    Kbd { "K" }
                                }
                                div { class: "sc-actions",
                                    span { class: "sc-muted-text", "Salva:" }
                                    Kbd { "Ctrl" }
                                    Kbd { "S" }
                                }
                                div { class: "sc-actions",
                                    span { class: "sc-muted-text", "Comando:" }
                                    Code { "dx serve --hot-reload" }
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
