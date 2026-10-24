//! Story: LoadingOverlay (feature `loading_overlay`). Stateless: the playground renders it
//! directly; `visible` is a knob.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::SettingsForm;
use crate::story::schema::*;

/// Children of the playground block, printed verbatim (the live render adds only sc-* classes).
const CHILDREN: &str = r#"Card {
    title: Some("Report settimanale".to_string()),
    subtitle: Some("Cluster Europa · 6 nodi".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, "Esporta CSV" }
            Button { "Invia al team" }
        }
    }),
    p { "Uptime 99,98% · latenza media 21 ms · 3 incidenti chiusi." }
}"#;

fn render(k: &Knobs) -> Element {
    let description = k.text("description");
    rsx! {
        LoadingOverlay {
            visible: k.bool("visible"),
            blur_px: k.number("blur_px") as u32,
            loader_size: k.number("loader_size") as u32,
            description: (!description.is_empty()).then_some(description),
            Card {
                title: Some("Report settimanale".to_string()),
                subtitle: Some("Cluster Europa · 6 nodi".to_string()),
                footer: Some(rsx! {
                    Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                        Button { variant: ButtonVariant::Ghost, "Esporta CSV" }
                        Button { "Invia al team" }
                    }
                }),
                p { class: "sc-muted-text", "Uptime 99,98% · latenza media 21 ms · 3 incidenti chiusi." }
            }
        }
    }
}

#[component]
fn Refresh() -> Element {
    let loading = use_signal(|| true);
    rsx! {
        Stack { gap: SpacingToken::Md,
            Switch { checked: loading, label: Some("Richiesta in corso".to_string()) }
            LoadingOverlay {
                visible: loading(),
                description: Some("Aggiornamento dei peer…".to_string()),
                List { spacing: ListSpacing::Sm,
                    ListItem { "edge-07 · Milano · 18 ms" }
                    ListItem { "edge-22 · Milano · 24 ms" }
                    ListItem { "edge-31 · Francoforte · 4 ms" }
                }
            }
        }
    }
}
const REFRESH_CODE: &str = r#"let loading = use_signal(|| true);

Stack { gap: SpacingToken::Md,
    Switch { checked: loading, label: Some("Richiesta in corso".to_string()) }
    LoadingOverlay {
        visible: loading(),
        description: Some("Aggiornamento dei peer…".to_string()),
        List { spacing: ListSpacing::Sm,
            ListItem { "edge-07 · Milano · 18 ms" }
            ListItem { "edge-22 · Milano · 24 ms" }
            ListItem { "edge-31 · Francoforte · 4 ms" }
        }
    }
}"#;

#[component]
fn Minimal() -> Element {
    rsx! {
        LoadingOverlay { visible: true, blur_px: 0, loader_size: 20,
            Card {
                title: Some("Chiavi API".to_string()),
                subtitle: Some("2 chiavi attive".to_string()),
                Group { gap: SpacingToken::Sm,
                    Badge { label: "prod".to_string(), variant: BadgeVariant::Success, pill: true }
                    Badge { label: "staging".to_string(), pill: true }
                }
            }
        }
    }
}
const MINIMAL_CODE: &str = r#"LoadingOverlay { visible: true, blur_px: 0, loader_size: 20,
    Card {
        title: Some("Chiavi API".to_string()),
        subtitle: Some("2 chiavi attive".to_string()),
        Group { gap: SpacingToken::Sm,
            Badge { label: "prod".to_string(), variant: BadgeVariant::Success, pill: true }
            Badge { label: "staging".to_string(), pill: true }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut saving = use_signal(|| false);
    let mut toast = use_notification();
    rsx! {
        Stack { gap: SpacingToken::Md,
            Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
                Badge { label: if saving() { "Pubblicazione".to_string() } else { "Bozza".to_string() }, variant: BadgeVariant::Warning, pill: true, dot: true }
                Group { gap: SpacingToken::Sm,
                    if saving() {
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| {
                                saving.set(false);
                                toast.success("Impostazioni pubblicate su 6 nodi");
                            },
                            "Simula risposta del server"
                        }
                    }
                    Button {
                        loading: saving(),
                        onclick: move |_| saving.set(true),
                        "Pubblica"
                    }
                }
            }
            LoadingOverlay {
                visible: saving(),
                description: Some("Distribuzione della configurazione…".to_string()),
                SettingsForm { compact: true }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut saving = use_signal(|| false);
let mut toast = use_notification();

Stack { gap: SpacingToken::Md,
    Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
        Badge { label: if saving() { "Pubblicazione".to_string() } else { "Bozza".to_string() }, variant: BadgeVariant::Warning, pill: true, dot: true }
        Group { gap: SpacingToken::Sm,
            if saving() {
                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| {
                        saving.set(false);
                        toast.success("Impostazioni pubblicate su 6 nodi");
                    },
                    "Simula risposta del server"
                }
            }
            Button {
                loading: saving(),
                onclick: move |_| saving.set(true),
                "Pubblica"
            }
        }
    }
    LoadingOverlay {
        visible: saving(),
        description: Some("Distribuzione della configurazione…".to_string()),
        SettingsForm { compact: true }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("visible", "bool", "-", "Mostra lo strato sopra i figli; obbligatoria."),
    prop("blur_px", "u32", "3", "Sfocatura del contenuto sotto lo strato, in px (0 per nessuna)."),
    prop("loader_size", "u32", "32", "Dimensione dello spinner in px."),
    prop("description", "Option<String>", "None", "Testo sotto lo spinner: di' cosa sta succedendo."),
    prop("children", "Element", "-", "La sezione da coprire; il wrapper è position: relative."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul wrapper."),
];

pub fn story() -> Story {
    Story {
        slug: "loading-overlay",
        name: "LoadingOverlay",
        category: Category::Dati,
        features: &["loading_overlay"],
        icon: IconKind::Spinner,
        tagline: "Copre una sola sezione con velo e spinner mentre aspetta una risposta, lasciando usabile il resto della pagina.",
        keywords: &["loading", "overlay", "caricamento", "spinner", "attesa", "blocco", "busy"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "LoadingOverlay",
            controls: vec![
                Control::bool("visible", "Visibile", true).emit(Emit::Always),
                Control::text("description", "Descrizione", "Generazione del report…")
                    .emit(Emit::Template("description: Some(\"{v}\".to_string())")),
                Control::number("blur_px", "Sfocatura (px)", 3.0, 0.0, 8.0, 1.0, 0),
                Control::number("loader_size", "Spinner (px)", 32.0, 16.0, 48.0, 4.0, 0),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(CHILDREN),
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "refresh",
                title: "Ricarica di un elenco",
                description: "Lo strato segue un signal: accendilo quando parte la richiesta.",
                code: REFRESH_CODE,
                render: || rsx! { Refresh {} },
                span: Span::Half,
            },
            Example {
                id: "minimal",
                title: "Senza sfocatura",
                description: "blur_px: 0 e spinner piccolo per riquadri compatti.",
                code: MINIMAL_CODE,
                render: || rsx! { Minimal {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Pubblicare le impostazioni blocca solo il form; la barra azioni resta attiva.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "LoadingOverlay", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Backdrop (language.md §3.3), limitato al contenitore",
            depth: "Velo sopra la sezione, sotto i layer flottanti",
            tokens: &["--sd-scrim", "--sd-accent", "--sd-text", "--sd-text-sm", "--sd-dur", "--sd-space-3"],
            notes: "Stesso velo dei dialoghi, ma position: absolute dentro il wrapper; lo spinner è un loop essenziale e rallenta con reduced motion.",
        }),
        a11y: &[
            "Lo strato ha aria-busy=true e aria-live=polite: la description viene annunciata quando appare.",
            "Copre i figli per il puntatore, ma non li rende inert: evita che il focus da tastiera resti dentro e disabilita i comandi se serve.",
            "Lo spinner è un loop essenziale: con prefers-reduced-motion rallenta invece di fermarsi.",
            "Usa una description che dica cosa sta succedendo, non solo \"Caricamento\".",
        ],
        related: &["skeletons", "progress", "notifications", "cards"],
    }
}
