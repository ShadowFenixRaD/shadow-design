//! Story: ProgressBar (feature `progress`). Stateless: the playground renders it directly.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const SIZES: [ProgressSize; 3] = [ProgressSize::Sm, ProgressSize::Md, ProgressSize::Lg];
const SIZE_OPTS: &[ChoiceOption] = &[
    opt("Sm", "ProgressSize::Sm"),
    opt("Md", "ProgressSize::Md"),
    opt("Lg", "ProgressSize::Lg"),
];
const VARIANTS: [ProgressVariant; 4] = [
    ProgressVariant::Primary,
    ProgressVariant::Success,
    ProgressVariant::Warning,
    ProgressVariant::Danger,
];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Primary", "ProgressVariant::Primary"),
    opt("Success", "ProgressVariant::Success"),
    opt("Warning", "ProgressVariant::Warning"),
    opt("Danger", "ProgressVariant::Danger"),
];

fn render(k: &Knobs) -> Element {
    let label = k.text("label");
    rsx! {
        ProgressBar {
            value: k.number("value") as f32,
            size: k.pick("size", &SIZES),
            variant: k.pick("variant", &VARIANTS),
            indeterminate: k.bool("indeterminate"),
            show_label: k.bool("show_label"),
            label: (!label.is_empty()).then_some(label),
        }
    }
}

#[component]
fn Variants() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Md,
            ProgressBar { value: 64.0, label: Some("Sincronizzazione 64%".to_string()) }
            ProgressBar { value: 100.0, variant: ProgressVariant::Success, label: Some("Backup completato".to_string()) }
            ProgressBar { value: 82.0, variant: ProgressVariant::Warning, label: Some("Disco all'82%".to_string()) }
            ProgressBar { value: 97.0, variant: ProgressVariant::Danger, label: Some("Quota quasi esaurita".to_string()) }
        }
    }
}
const VARIANTS_CODE: &str = r#"Stack { gap: SpacingToken::Md,
    ProgressBar { value: 64.0, label: Some("Sincronizzazione 64%".to_string()) }
    ProgressBar { value: 100.0, variant: ProgressVariant::Success, label: Some("Backup completato".to_string()) }
    ProgressBar { value: 82.0, variant: ProgressVariant::Warning, label: Some("Disco all'82%".to_string()) }
    ProgressBar { value: 97.0, variant: ProgressVariant::Danger, label: Some("Quota quasi esaurita".to_string()) }
}"#;

#[component]
fn Sizes() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Md,
            ProgressBar { value: 40.0, size: ProgressSize::Sm, show_label: true }
            ProgressBar { value: 55.0, size: ProgressSize::Md, show_label: true }
            ProgressBar { value: 70.0, size: ProgressSize::Lg, show_label: true }
            ProgressBar { indeterminate: true, label: Some("Scansione dei peer…".to_string()) }
        }
    }
}
const SIZES_CODE: &str = r#"Stack { gap: SpacingToken::Md,
    ProgressBar { value: 40.0, size: ProgressSize::Sm, show_label: true }
    ProgressBar { value: 55.0, size: ProgressSize::Md, show_label: true }
    ProgressBar { value: 70.0, size: ProgressSize::Lg, show_label: true }
    ProgressBar { indeterminate: true, label: Some("Scansione dei peer…".to_string()) }
}"#;

#[component]
fn InContext() -> Element {
    let mut copied = use_signal(|| 3u32);
    let mut toast = use_notification();
    let total = 8u32;
    let pct = copied() as f32 / total as f32 * 100.0;
    let done = copied() >= total;
    rsx! {
        SimpleGrid { cols: 1, md: Some(2), spacing: GridSpacing::Md,
            Card {
                title: Some("Replica dei volumi".to_string()),
                subtitle: Some("edge-07 → edge-22 · 8 volumi".to_string()),
                footer: Some(rsx! {
                    Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                        Button {
                            variant: ButtonVariant::Ghost,
                            onclick: move |_| copied.set(0),
                            "Ricomincia"
                        }
                        Button {
                            disabled: done,
                            onclick: move |_| {
                                copied += 1;
                                if copied() >= total {
                                    toast.success("Replica completata");
                                }
                            },
                            "Copia il prossimo volume"
                        }
                    }
                }),
                ProgressBar {
                    value: pct,
                    variant: if done { ProgressVariant::Success } else { ProgressVariant::Primary },
                    label: Some(format!("{} di {total} volumi copiati", copied())),
                }
            }
            Card {
                title: Some("Quota di archiviazione".to_string()),
                subtitle: Some("Tre regioni · aggiornato ora".to_string()),
                Stack { gap: SpacingToken::Md,
                    ProgressBar { value: 46.0, size: ProgressSize::Sm, label: Some("eu-south · 230 GB di 500 GB".to_string()) }
                    ProgressBar { value: 82.0, size: ProgressSize::Sm, variant: ProgressVariant::Warning, label: Some("eu-west · 410 GB di 500 GB".to_string()) }
                    ProgressBar { value: 97.0, size: ProgressSize::Sm, variant: ProgressVariant::Danger, label: Some("us-east · 485 GB di 500 GB".to_string()) }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut copied = use_signal(|| 3u32);
let mut toast = use_notification();
let total = 8u32;
let pct = copied() as f32 / total as f32 * 100.0;
let done = copied() >= total;

SimpleGrid { cols: 1, md: Some(2), spacing: GridSpacing::Md,
    Card {
        title: Some("Replica dei volumi".to_string()),
        subtitle: Some("edge-07 → edge-22 · 8 volumi".to_string()),
        footer: Some(rsx! {
            Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: move |_| copied.set(0),
                    "Ricomincia"
                }
                Button {
                    disabled: done,
                    onclick: move |_| {
                        copied += 1;
                        if copied() >= total {
                            toast.success("Replica completata");
                        }
                    },
                    "Copia il prossimo volume"
                }
            }
        }),
        ProgressBar {
            value: pct,
            variant: if done { ProgressVariant::Success } else { ProgressVariant::Primary },
            label: Some(format!("{} di {total} volumi copiati", copied())),
        }
    }
    Card {
        title: Some("Quota di archiviazione".to_string()),
        subtitle: Some("Tre regioni · aggiornato ora".to_string()),
        Stack { gap: SpacingToken::Md,
            ProgressBar { value: 46.0, size: ProgressSize::Sm, label: Some("eu-south · 230 GB di 500 GB".to_string()) }
            ProgressBar { value: 82.0, size: ProgressSize::Sm, variant: ProgressVariant::Warning, label: Some("eu-west · 410 GB di 500 GB".to_string()) }
            ProgressBar { value: 97.0, size: ProgressSize::Sm, variant: ProgressVariant::Danger, label: Some("us-east · 485 GB di 500 GB".to_string()) }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("value", "f32", "0.0", "Avanzamento da 0 a 100; i valori fuori intervallo vengono limitati."),
    prop("size", "ProgressSize", "Md", "Spessore della traccia: Sm, Md, Lg."),
    prop("variant", "ProgressVariant", "Primary", "Primary (accento), Success, Warning, Danger."),
    prop("indeterminate", "bool", "false", "Segmento animato per durate sconosciute; nasconde la percentuale."),
    prop("show_label", "bool", "false", "Mostra la percentuale arrotondata sopra la barra."),
    prop("label", "Option<String>", "None", "Testo sopra la barra; sostituisce la percentuale."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul wrapper."),
];

pub fn story() -> Story {
    Story {
        slug: "progress",
        name: "ProgressBar",
        category: Category::Dati,
        features: &["progress"],
        icon: IconKind::Spinner,
        tagline: "Barra di avanzamento per operazioni misurabili o di durata sconosciuta, con tinte semantiche.",
        keywords: &["progress", "progresso", "avanzamento", "barra", "caricamento", "quota", "upload"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "ProgressBar",
            controls: vec![
                Control::number("value", "Valore", 68.0, 0.0, 100.0, 1.0, 1).emit(Emit::Always),
                Control::choice("size", "Spessore", SIZE_OPTS, 1),
                Control::choice("variant", "Variante", VARIANT_OPTS, 0),
                Control::text("label", "Etichetta", "").emit(Emit::Template("label: Some(\"{v}\".to_string())"))
                    .hint("Vuota: con Percentuale attiva mostra il valore."),
                Control::bool("show_label", "Percentuale", true).emit(Emit::Template("show_label: true")),
                Control::bool("indeterminate", "Indeterminato", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "variants",
                title: "Varianti",
                description: "La tinta dice se il valore è un buon segno o un rischio.",
                code: VARIANTS_CODE,
                render: || rsx! { Variants {} },
                span: Span::Half,
            },
            Example {
                id: "sizes",
                title: "Spessori e indeterminato",
                description: "Tre spessori e un caricamento senza durata nota.",
                code: SIZES_CODE,
                render: || rsx! { Sizes {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Una replica che avanza volume per volume e le quote di tre regioni.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "ProgressBar", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Value fill (language.md §3.11)",
            depth: "Well (traccia)",
            tokens: &["--sd-well", "--sd-border-subtle", "--sd-accent", "--sd-glow-sm", "--sd-success", "--sd-danger", "--sd-radius-full"],
            notes: "Traccia incassata, mai bianca; il riempimento è l'accento pieno (o il colore semantico) con il glow minimo.",
        }),
        a11y: &[
            "La traccia ha role=progressbar con aria-valuenow, aria-valuemin=0 e aria-valuemax=100.",
            "L'etichetta visibile è testo nel DOM, letto prima della barra.",
            "In modalità indeterminata il segmento rallenta con prefers-reduced-motion invece di fermarsi.",
            "Il colore semantico non è l'unico segnale: affiancalo a un'etichetta che dice lo stato.",
        ],
        related: &["loading-overlay", "skeletons", "notifications", "charts"],
    }
}
