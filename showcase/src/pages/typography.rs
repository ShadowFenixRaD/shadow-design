//! Typography: the three type roles of ADR-0005 side by side, with specimens and the
//! `Heading` / `Text` / `Kbd` / `Code` components.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::sections::SectionHead;

#[derive(Clone, Copy, PartialEq)]
struct Role {
    family: &'static str,
    token: &'static str,
    role: &'static str,
    usage: &'static str,
    specimen_class: &'static str,
}

const ROLES: [Role; 3] = [
    Role {
        family: "Kanit",
        token: "--sd-font-heading · --sd-weight-heading",
        role: "Titoli",
        usage: "Titoli di pagina e sezione, Title, titoli di card, modali, popover e toast, metriche grandi.",
        specimen_class: "sc-specimen sc-specimen--heading",
    },
    Role {
        family: "Onest",
        token: "--sd-font-body",
        role: "Testo e controlli",
        usage: "Tutto ciò che si legge o si usa: testo, etichette, pulsanti, voci, campi, celle, badge.",
        specimen_class: "sc-specimen sc-specimen--body",
    },
    Role {
        family: "Source Code Pro",
        token: "--sd-font-mono",
        role: "Codice ed etichette",
        usage: "Codice, kbd, etichette maiuscole di sezione, valori numerici in tabella, assi dei grafici.",
        specimen_class: "sc-specimen sc-specimen--mono",
    },
];

const HEADINGS_CODE: &str = r#"Heading { order: HeadingOrder::H1, "Operazioni di rete" }
Heading { order: HeadingOrder::H2, "Nodi attivi" }
Heading { order: HeadingOrder::H3, "edge-07 · Milano" }
Heading { order: HeadingOrder::H4, "Peer collegati" }"#;

const TEXT_CODE: &str = r#"Text { size: TextSize::Lg, "Vetro nero illuminato dall'alto." }
Text { size: TextSize::Md, "Il corpo del testo è Onest a 14px." }
Text { size: TextSize::Sm, dimmed: true, "Metadati e didascalie in --sd-text-muted." }"#;

const INLINE_CODE: &str = r#"Kbd { "Ctrl" } Kbd { "K" }
Code { "cargo add shadow-design" }"#;

#[component]
pub fn TypographyPage() -> Element {
    rsx! {
        document::Title { "Typography · Shadow Design" }
        div { class: "sc-story",
            header { class: "sc-story__head",
                span { class: "sc-eyebrow", "Fondamenta · feature typography" }
                h1 { class: "sc-title", "Typography" }
                p { class: "sc-lede",
                    "Tre famiglie, ognuna con un ruolo (ADR-0005). I titoli non usano mai il corpo e i controlli non usano mai il titolo."
                }
            }

            section { class: "sc-section",
                SectionHead { eyebrow: "Ruoli", title: "Kanit, Onest, Source Code Pro" }
                div { class: "sc-token-grid sc-token-grid--3",
                    for r in ROLES {
                        Card { key: "{r.family}", title: Some(r.role.to_string()), subtitle: Some(r.token.to_string()),
                            div { class: "sc-role",
                                span { class: "{r.specimen_class}", "{r.family}" }
                                span { class: "{r.specimen_class} sc-specimen--glyphs", "Aa Bb 0123 — àèìòù" }
                                p { class: "sc-muted-text", "{r.usage}" }
                            }
                        }
                    }
                }
            }

            section { class: "sc-section",
                SectionHead {
                    eyebrow: "Heading",
                    title: "Gerarchia dei titoli",
                    desc: Some("Kanit a peso 600, interlinea --sd-leading-tight."),
                }
                Card {
                    Stack { gap: SpacingToken::Sm,
                        Heading { order: HeadingOrder::H1, "Operazioni di rete" }
                        Heading { order: HeadingOrder::H2, "Nodi attivi" }
                        Heading { order: HeadingOrder::H3, "edge-07 · Milano" }
                        Heading { order: HeadingOrder::H4, "Peer collegati" }
                    }
                }
                CodeBlock { code: HEADINGS_CODE.to_string(), language: "rsx", with_line_numbers: false }
            }

            section { class: "sc-section",
                SectionHead { eyebrow: "Text", title: "Corpo del testo", desc: Some("Onest; tre livelli di colore, mai più tenui.") }
                Card {
                    Stack { gap: SpacingToken::Sm,
                        Text { size: TextSize::Lg, "Vetro nero illuminato dall'alto." }
                        Text { size: TextSize::Md, "Il corpo del testo è Onest a 14px." }
                        Text { size: TextSize::Sm, dimmed: true, "Metadati e didascalie in --sd-text-muted." }
                    }
                }
                CodeBlock { code: TEXT_CODE.to_string(), language: "rsx", with_line_numbers: false }
            }

            section { class: "sc-section",
                SectionHead { eyebrow: "Kbd e Code", title: "Scorciatoie e codice in linea" }
                Card {
                    Group { gap: SpacingToken::Md, align: AlignItems::Center,
                        Group { gap: SpacingToken::Xs, align: AlignItems::Center,
                            Kbd { "Ctrl" }
                            Kbd { "K" }
                        }
                        Code { "cargo add shadow-design" }
                    }
                }
                CodeBlock { code: INLINE_CODE.to_string(), language: "rsx", with_line_numbers: false }
            }
        }
    }
}
