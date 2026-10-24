//! Story: Spoiler (feature `spoiler`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const CHILDREN: &str = r#"Stack { gap: SpacingToken::Sm,
    Text { "Ogni nodo mantiene un vettore di versioni e scambia solo le differenze firmate con i peer vicini." }
    Text { "I pacchetti sono divisi in blocchi da 64 KB con codici di correzione: la rete regge la perdita di quattro nodi." }
    Text { "Se la rete si divide, ogni gruppo continua a lavorare in locale e riconcilia le transazioni al ritorno del collegamento." }
}"#;

fn render(k: &Knobs) -> Element {
    rsx! {
        Spoiler {
            max_height: k.number("max_height") as u32,
            show_label: k.text("show_label"),
            hide_label: k.text("hide_label"),
            Stack { gap: SpacingToken::Sm,
                Text { "Ogni nodo mantiene un vettore di versioni e scambia solo le differenze firmate con i peer vicini." }
                Text { "I pacchetti sono divisi in blocchi da 64 KB con codici di correzione: la rete regge la perdita di quattro nodi." }
                Text { "Se la rete si divide, ogni gruppo continua a lavorare in locale e riconcilia le transazioni al ritorno del collegamento." }
            }
        }
    }
}

#[component]
fn Controlled() -> Element {
    let mut expanded = use_signal(|| false);
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Group { justify: JustifyContent::SpaceBetween,
                Text { size: TextSize::Sm, dimmed: true, "Changelog 0.5.2" }
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| expanded.toggle(),
                    if expanded() { "Comprimi" } else { "Leggi tutto" }
                }
            }
            Spoiler { max_height: 72, expanded: Some(expanded),
                List { spacing: ListSpacing::Xs,
                    ListItem { "Motore dei temi con override per componente." }
                    ListItem { "Tre ruoli tipografici: Kanit, Onest, Source Code Pro." }
                    ListItem { "Nuovo ScrollArea con dissolvenza ai bordi." }
                    ListItem { "Select: gruppi e creazione di voci." }
                    ListItem { "Correzioni di contrasto su Crimson ed Emerald." }
                }
            }
        }
    }
}
const CONTROLLED_CODE: &str = r#"let mut expanded = use_signal(|| false);

Stack { gap: SpacingToken::Sm,
    Group { justify: JustifyContent::SpaceBetween,
        Text { size: TextSize::Sm, dimmed: true, "Changelog 0.5.2" }
        Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| expanded.toggle(),
            if expanded() { "Comprimi" } else { "Leggi tutto" }
        }
    }
    Spoiler { max_height: 72, expanded: Some(expanded),
        List { spacing: ListSpacing::Xs,
            ListItem { "Motore dei temi con override per componente." }
            ListItem { "Tre ruoli tipografici: Kanit, Onest, Source Code Pro." }
            ListItem { "Nuovo ScrollArea con dissolvenza ai bordi." }
            ListItem { "Select: gruppi e creazione di voci." }
            ListItem { "Correzioni di contrasto su Crimson ed Emerald." }
        }
    }
}"#;

#[component]
fn Short() -> Element {
    rsx! {
        Spoiler { max_height: 160,
            Text { size: TextSize::Sm, "Testo breve: sta sotto max_height, quindi il pulsante non compare." }
        }
    }
}
const SHORT_CODE: &str = r#"Spoiler { max_height: 160,
    Text { size: TextSize::Sm, "Testo breve: sta sotto max_height, quindi il pulsante non compare." }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        Card {
            avatar: Some(rsx! {
                Avatar { name: Some("Marco Lenzi".to_string()) }
            }),
            title: Some("Rapporto di audit · edge-03".to_string()),
            subtitle: Some("Marco Lenzi · 2 ore fa".to_string()),
            buttons: Some(rsx! {
                Badge { label: "2 rilievi".to_string(), variant: BadgeVariant::Warning }
            }),
            Spoiler { max_height: 96, show_label: "Leggi il rapporto".to_string(), hide_label: "Riduci".to_string(),
                Stack { gap: SpacingToken::Sm,
                    Text { size: TextSize::Sm, "La rotazione delle chiavi funziona come previsto su tutti i nodi della regione." }
                    Text { size: TextSize::Sm, "Rilievo 1: il nodo edge-03 accetta ancora suite TLS 1.2 per i client legacy." }
                    Text { size: TextSize::Sm, "Rilievo 2: i log di accesso non vengono firmati prima dell'archiviazione." }
                    Text { size: TextSize::Sm, "Raccomandazione: disattivare TLS 1.2 entro la prossima finestra e abilitare la firma dei log." }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"Card {
    avatar: Some(rsx! {
        Avatar { name: Some("Marco Lenzi".to_string()) }
    }),
    title: Some("Rapporto di audit · edge-03".to_string()),
    subtitle: Some("Marco Lenzi · 2 ore fa".to_string()),
    buttons: Some(rsx! {
        Badge { label: "2 rilievi".to_string(), variant: BadgeVariant::Warning }
    }),
    Spoiler { max_height: 96, show_label: "Leggi il rapporto".to_string(), hide_label: "Riduci".to_string(),
        Stack { gap: SpacingToken::Sm,
            Text { size: TextSize::Sm, "La rotazione delle chiavi funziona come previsto su tutti i nodi della regione." }
            Text { size: TextSize::Sm, "Rilievo 1: il nodo edge-03 accetta ancora suite TLS 1.2 per i client legacy." }
            Text { size: TextSize::Sm, "Rilievo 2: i log di accesso non vengono firmati prima dell'archiviazione." }
            Text { size: TextSize::Sm, "Raccomandazione: disattivare TLS 1.2 entro la prossima finestra e abilitare la firma dei log." }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("max_height", "u32", "120", "Altezza in px oltre la quale il contenuto viene troncato."),
    prop("show_label / hide_label", "String", "\"\" (da Locale)", "Testo del pulsante chiuso e aperto; vuoto usa Locale."),
    prop("expanded", "Option<Signal<bool>>", "None", "Stato aperto controllato dall'esterno."),
    prop("default_expanded", "bool", "false", "Stato iniziale se non controllato."),
    prop("on_toggle", "Option<EventHandler<bool>>", "None", "Nuovo stato dopo il click."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "spoiler",
        name: "Spoiler",
        category: Category::Layout,
        features: &["spoiler"],
        icon: IconKind::ChevronDown,
        tagline: "Tronca un testo lungo con una dissolvenza e lo apre con un pulsante.",
        keywords: &["spoiler", "mostra di più", "show more", "tronca", "espandi", "read more"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Spoiler",
            controls: vec![
                Control::number("max_height", "Altezza massima (px)", 120.0, 48.0, 240.0, 8.0, 0),
                Control::text("show_label", "Etichetta chiuso", "")
                    .emit(Emit::Template("show_label: \"{v}\".to_string()"))
                    .hint("Vuota: \"Mostra di più\" dal Locale attivo."),
                Control::text("hide_label", "Etichetta aperto", "")
                    .emit(Emit::Template("hide_label: \"{v}\".to_string()"))
                    .hint("Vuota: \"Mostra meno\" dal Locale attivo."),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(CHILDREN),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "controlled",
                title: "Controllato",
                description: "Un pulsante esterno apre lo stesso Spoiler.",
                code: CONTROLLED_CODE,
                render: || rsx! { Controlled {} },
                span: Span::Half,
            },
            Example {
                id: "short",
                title: "Contenuto breve",
                description: "Nessun pulsante se il testo entra.",
                code: SHORT_CODE,
                render: || rsx! { Short {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Anteprima di un rapporto di audit in una card.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Spoiler", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Contenuto nel container ospite (§3.1) + pulsante ghost (§3.5)",
            depth: "Void (testo) · Lit control ghost (pulsante)",
            tokens: &["--sd-surface-1", "--sd-text-muted", "--sd-control-sm", "--sd-tint-1", "--sd-dur"],
            notes: "La dissolvenza va verso la superficie del contenitore; il pulsante è ghost, mai primario.",
        }),
        a11y: &[
            "Il pulsante è un Button nativo, raggiungibile con Tab e attivabile con Invio o Spazio.",
            "Il testo troncato resta nel DOM: i lettori di schermo lo leggono per intero.",
            "Le etichette predefinite vengono da Locale (spoiler_show_more, spoiler_show_less).",
            "Il pulsante appare solo se il contenuto supera max_height.",
        ],
        related: &["accordion", "cards", "rich-text", "timelines"],
    }
}
