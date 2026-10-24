use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    CheckControl, ChoicePicker, ControlRow, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn CardsPage() -> Element {
    let selected_variant = use_signal(|| CardVariant::Default);
    let selected_shadow = use_signal(|| CardShadow::Md);
    let with_avatar = use_signal(|| true);
    let with_title = use_signal(|| true);
    let with_subtitle = use_signal(|| true);
    let with_progress = use_signal(|| false);
    let with_buttons = use_signal(|| true);
    let with_image = use_signal(|| false);
    let enable_glow = use_signal(|| false);
    let with_max_height = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "id",
            r#type: "Option<String>",
            default: "None",
            description: "Identificatore univoco HTML per il contenitore.",
        },
        PropRow {
            name: "title",
            r#type: "Option<String>",
            default: "None",
            description: "Titolo primario dell'intestazione (renderizzato in h3).",
        },
        PropRow {
            name: "subtitle",
            r#type: "Option<String>",
            default: "None",
            description: "Testo secondario sotto al titolo in grigio attenuato.",
        },
        PropRow {
            name: "avatar",
            r#type: "Option<Element>",
            default: "None",
            description: "Slot per icona o componente Avatar a sinistra dell'intestazione.",
        },
        PropRow {
            name: "variant",
            r#type: "CardVariant",
            default: "CardVariant::Default",
            description: "Variante visiva (Default, Elevated, Interactive, Metric).",
        },
        PropRow {
            name: "shadow",
            r#type: "CardShadow",
            default: "CardShadow::Md",
            description: "Livello di elevazione e profondità dell'ombra ottica (None, Sm, Md, Lg, Xl).",
        },
        PropRow {
            name: "hover_glow",
            r#type: "bool",
            default: "false",
            description: "Micro-glow coordinato con il colore del tema attivo al passaggio del mouse.",
        },
        PropRow {
            name: "glow",
            r#type: "Option<bool>",
            default: "None",
            description: "Controllo esplicito del bagliore ottico della scheda.",
        },
        PropRow {
            name: "onclick / on_click",
            r#type: "Option<EventHandler<MouseEvent>>",
            default: "None",
            description: "Callback di click per CardVariant::Interactive con role='button' e tastiera.",
        },
        PropRow {
            name: "href",
            r#type: "Option<String>",
            default: "None",
            description: "URL di destinazione per CardVariant::Interactive tramite link stretched accessibile.",
        },
        PropRow {
            name: "progress",
            r#type: "Option<f32>",
            default: "None",
            description: "Avanzamento percentuale (0.0 .. 100.0) con barra ProgressBar integrata.",
        },
        PropRow {
            name: "buttons",
            r#type: "Option<Element>",
            default: "None",
            description: "Slot inferiore per azioni rapide e pulsanti.",
        },
        PropRow {
            name: "image",
            r#type: "Option<String>",
            default: "None",
            description: "Immagine a tutta larghezza in basso con adattamento e ritaglio bordi.",
        },
        PropRow {
            name: "image_height",
            r#type: "Option<String>",
            default: "None",
            description: "Altezza personalizzata opzionale per l'immagine inferiore (es. '160px').",
        },
        PropRow {
            name: "footer",
            r#type: "Option<Element>",
            default: "None",
            description: "Slot per la sezione a piè di pagina con sfondo e divisore dedicato.",
        },
        PropRow {
            name: "overflow_hidden",
            r#type: "bool",
            default: "false",
            description: "Forza overflow hidden per ritagliare media senza occludere menu dropdown.",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate al contenitore.",
        },
    ];

    let code_snippet = r#"Card {
    title: Some("Mesh Node Cluster".to_string()),
    subtitle: Some("Topologia di rete crittografica P2P".to_string()),
    variant: CardVariant::Interactive,
    shadow: CardShadow::Lg,
    hover_glow: true,
    avatar: rsx! {
        Avatar { name: Some("Shadow Core".to_string()), size: AvatarSize::Md, status: PresenceStatus::Online }
    },
    progress: Some(82.0),
    buttons: rsx! {
        Button { size: ButtonSize::Sm, variant: ButtonVariant::Primary, "Connetti" }
        Button { size: ButtonSize::Sm, variant: ButtonVariant::Outline, "Dettagli" }
    },
    image: Some("https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?auto=format&fit=crop&w=600&q=80".to_string()),
    footer: rsx! {
        span { class: "sc-subtle-text", "Ultima sincronizzazione 2 minuti fa" }
    },
    p { class: "sc-muted-text", "Nodo di sincronizzazione Kyber-1024 attivo." }
}"#
        .to_string();

    let max_h_class = if with_max_height() { "max-h-[420px]" } else { "" };

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Card (Unificata)".to_string(),
                description: "Superficie modulare avanzata: supporto nativo per avatar, titolo centrato o affiancato, sottotitolo, progress bar, pulsanti d'azione dedicati e cover image inferiore con ritaglio bordi e max-height.".to_string(),
            }

            FeatureBanner { feature_name: "card" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            Card {
                                class: "{max_h_class} w-full max-w-sm",
                                avatar: if with_avatar() {
                                    Some(rsx! {
                                        Avatar {
                                            name: Some("Shadow Core".to_string()),
                                            size: AvatarSize::Md,
                                            status: PresenceStatus::Online,
                                        }
                                    })
                                } else {
                                    None
                                },
                                title: if with_title() { Some("Cluster Mesh #07".to_string()) } else { None },
                                subtitle: if with_subtitle() { Some("Kyber-1024 Post-Quantum • 4 nodi attivi".to_string()) } else { None },
                                variant: selected_variant(),
                                shadow: selected_shadow(),
                                hover_glow: enable_glow(),
                                progress: if with_progress() { Some(68.0) } else { None },
                                buttons: if with_buttons() {
                                    Some(rsx! {
                                        Button { size: ButtonSize::Sm, variant: ButtonVariant::Primary, "Sincronizza" }
                                        Button { size: ButtonSize::Sm, variant: ButtonVariant::Ghost, "Ispeziona" }
                                    })
                                } else {
                                    None
                                },
                                image: if with_image() {
                                    Some("https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?auto=format&fit=crop&w=600&q=80".to_string())
                                } else {
                                    None
                                },
                                div { class: "sc-stack",
                                    p { class: "sc-muted-text", "Contenuto informativo della card con tipografia ad alto contrasto e bordi ottici calibrati." }
                                    Badge { label: "Zero-Knowledge Validated".to_string(), variant: BadgeVariant::Success, dot: true }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Variante Card".to_string(),
                                options: vec![
                                    (CardVariant::Default, "Default".to_string()),
                                    (CardVariant::Elevated, "Elevated".to_string()),
                                    (CardVariant::Interactive, "Interactive".to_string()),
                                    (CardVariant::Metric, "Metric".to_string()),
                                ],
                                value: selected_variant,
                            }
                            ChoicePicker {
                                label: "Livello Ombra".to_string(),
                                options: vec![
                                    (CardShadow::None, "None".to_string()),
                                    (CardShadow::Sm, "Sm".to_string()),
                                    (CardShadow::Md, "Md".to_string()),
                                    (CardShadow::Lg, "Lg".to_string()),
                                    (CardShadow::Xl, "Xl".to_string()),
                                ],
                                value: selected_shadow,
                            }
                            hr { class: "sc-control-divider" }
                            span { class: "sc-control-label", "Sezioni Attive" }
                            ControlRow {
                                CheckControl { label: "Avatar".to_string(), checked: with_avatar }
                                CheckControl { label: "Titolo".to_string(), checked: with_title }
                                CheckControl { label: "Sottotitolo".to_string(), checked: with_subtitle }
                                CheckControl { label: "Progress Bar".to_string(), checked: with_progress }
                                CheckControl { label: "Bottoni in Basso".to_string(), checked: with_buttons }
                                CheckControl { label: "Immagine in Basso".to_string(), checked: with_image }
                            }
                            hr { class: "sc-control-divider" }
                            ToggleControl { label: "Attiva Micro-Glow su Hover".to_string(), checked: enable_glow }
                            ToggleControl { label: "Test Altezza Massima (Taglio Immagine)".to_string(), checked: with_max_height }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Visual Examples
            div { class: "sc-card-grid sc-card-grid--3",
                Card {
                    variant: CardVariant::Metric,
                    div { class: "sc-title-row",
                        div { class: "sc-stack",
                            span { class: "sc-control-label", "Latenza Media" }
                            span { class: "sc-section-title", "1.2 ms" }
                            span { class: "sc-success-text sc-mono", "+14% performance" }
                        }
                        Icon { kind: IconKind::Sparkles, size: 18 }
                    }
                }
                Card {
                    variant: CardVariant::Interactive,
                    hover_glow: true,
                    onclick: move |_| {},
                    div { class: "sc-title-row",
                        div { class: "sc-stack",
                            span { class: "sc-control-label", "Richieste Totali" }
                            span { class: "sc-section-title", "4.8M" }
                            span { class: "sc-accent-text sc-mono", "Hover per micro-glow" }
                        }
                        Icon { kind: IconKind::Layers, size: 18 }
                    }
                }
                Card {
                    variant: CardVariant::Elevated,
                    div { class: "sc-title-row",
                        div { class: "sc-stack",
                            span { class: "sc-control-label", "Affidabilità" }
                            span { class: "sc-section-title", "99.99%" }
                            span { class: "sc-muted-text sc-mono", "Livello Surface 2" }
                        }
                        Icon { kind: IconKind::CheckCircle, size: 18 }
                    }
                }
            }
        }
    }
}
