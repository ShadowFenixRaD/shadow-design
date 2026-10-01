use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};
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
            description: "Applica colore attenuato text-zinc-400 per metadati e spiegazioni",
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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Typography" }
                Text { dimmed: true, size: TextSize::Sm, "Componenti per gerarchia testuale, scorciatoie da tastiera e codice inline basati sui font Geist e Geist Mono." }
            }

            FeatureBanner { feature_name: "typography" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6",
                        div { class: "space-y-2",
                            Heading { order: HeadingOrder::H1, "Titolo H1 Display" }
                            Heading { order: HeadingOrder::H2, "Titolo H2 Section" }
                            Heading { order: HeadingOrder::H3, "Titolo H3 Card" }
                            Heading { order: HeadingOrder::H4, "Titolo H4 Subsection" }
                        }
                        div { class: "pt-4 border-t border-white/[0.06] space-y-3",
                            Text { size: TextSize::Lg, "Paragrafo Large: eccellente leggibilità su display Retina ad alta densità." }
                            Text { size: TextSize::Md, "Paragrafo Standard: progettato con il font Geist Sans per un'esperienza di lettura pulita ed essenziale." }
                            Text { size: TextSize::Sm, dimmed: true, "Testo attenuato (dimmed): ideale per note a piè di pagina e timestamp." }
                        }
                        div { class: "pt-4 border-t border-white/[0.06] flex flex-wrap items-center gap-4",
                            div { class: "flex items-center gap-1.5",
                                span { class: "text-xs text-zinc-400", "Scorciatoia:" }
                                Kbd { "⌘" }
                                Kbd { "K" }
                            }
                            div { class: "flex items-center gap-1.5",
                                span { class: "text-xs text-zinc-400", "Salva:" }
                                Kbd { "Ctrl" }
                                Kbd { "S" }
                            }
                            div { class: "flex items-center gap-1.5",
                                span { class: "text-xs text-zinc-400", "Comando:" }
                                Code { "dx serve --hot-reload" }
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
