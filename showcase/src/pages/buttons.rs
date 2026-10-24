use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, ControlRow, Counter, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn ButtonsPage() -> Element {
    let mut click_count = use_signal(|| 0);
    let selected_variant = use_signal(|| ButtonVariant::Primary);
    let selected_size = use_signal(|| ButtonSize::Md);
    let is_loading = use_signal(|| false);
    let is_disabled = use_signal(|| false);
    let show_left_icon = use_signal(|| true);
    let show_right_icon = use_signal(|| false);
    let is_full_width = use_signal(|| false);
    let enable_glow = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "variant",
            r#type: "ButtonVariant",
            default: "ButtonVariant::Primary",
            description: "Stile cromatico (Primary, Secondary, Outline, Ghost, Danger, Success)",
        },
        PropRow {
            name: "size",
            r#type: "ButtonSize",
            default: "ButtonSize::Md",
            description: "Scala dimensionale (Xs, Sm, Md, Lg, Xl)",
        },
        PropRow {
            name: "loading",
            r#type: "bool",
            default: "false",
            description: "Mostra uno spinner SVG rotante e imposta aria-busy e aria-disabled",
        },
        PropRow {
            name: "loading_label",
            r#type: "Option<String>",
            default: "None",
            description: "Testo opzionale per lettori di schermo durante lo stato di caricamento",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita il pulsante e riduce l'opacità con aria-disabled",
        },
        PropRow {
            name: "full_width",
            r#type: "bool",
            default: "false",
            description: "Espande il pulsante al 100% della larghezza del contenitore",
        },
        PropRow {
            name: "glow",
            r#type: "Option<bool>",
            default: "None",
            description: "Attiva o disattiva esplicitamente il micro-glow perimetrale",
        },
        PropRow {
            name: "left_icon",
            r#type: "Option<Element>",
            default: "None",
            description: "Elemento icona visualizzato prima del testo (con aria-hidden)",
        },
        PropRow {
            name: "right_icon",
            r#type: "Option<Element>",
            default: "None",
            description: "Elemento icona visualizzato dopo il testo (con aria-hidden)",
        },
        PropRow {
            name: "onclick",
            r#type: "Option<EventHandler<MouseEvent>>",
            default: "None",
            description: "Callback scatenata al click o all'attivazione da tastiera",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate all'elemento <button>",
        },
    ];

    let code_snippet = r#"Button {
    variant: ButtonVariant::Primary,
    size: ButtonSize::Md,
    left_icon: rsx! { Icon { kind: IconKind::Sparkles, size: 16 } },
    onclick: move |_| println!("Pulsante premuto!"),
    "Azione Principale"
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Button".to_string(),
                description: "Pulsanti tattili con feedback ottico alla pressione, supporto a icone native, stato loading accessibile e raggio scalabile da tema.".to_string(),
            }

            FeatureBanner { feature_name: "button" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            div { class: if is_full_width() { "w-full" } else { "" },
                                Button {
                                    variant: selected_variant(),
                                    size: selected_size(),
                                    loading: is_loading(),
                                    disabled: is_disabled(),
                                    full_width: is_full_width(),
                                    glow: if enable_glow() { Some(true) } else { None },
                                    left_icon: if show_left_icon() {
                                        Some(rsx! { Icon { kind: IconKind::Sparkles, size: 15 } })
                                    } else {
                                        None
                                    },
                                    right_icon: if show_right_icon() {
                                        Some(rsx! { Icon { kind: IconKind::ChevronRight, size: 15 } })
                                    } else {
                                        None
                                    },
                                    onclick: move |_| *click_count.write() += 1,
                                    "Pulsante Interattivo"
                                }
                            }
                            Counter { label: "Click registrati".to_string(), value: click_count().to_string() }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Variante".to_string(),
                                options: vec![
                                    (ButtonVariant::Primary, "Primary".to_string()),
                                    (ButtonVariant::Secondary, "Secondary".to_string()),
                                    (ButtonVariant::Outline, "Outline".to_string()),
                                    (ButtonVariant::Ghost, "Ghost".to_string()),
                                    (ButtonVariant::Success, "Success".to_string()),
                                    (ButtonVariant::Danger, "Danger".to_string()),
                                ],
                                value: selected_variant,
                            }
                            ChoicePicker {
                                label: "Dimensione".to_string(),
                                options: vec![
                                    (ButtonSize::Xs, "Xs".to_string()),
                                    (ButtonSize::Sm, "Sm".to_string()),
                                    (ButtonSize::Md, "Md".to_string()),
                                    (ButtonSize::Lg, "Lg".to_string()),
                                    (ButtonSize::Xl, "Xl".to_string()),
                                ],
                                value: selected_size,
                            }
                            hr { class: "sc-control-divider" }
                            ControlRow {
                                ToggleControl { label: "Stato Loading".to_string(), checked: is_loading }
                                ToggleControl { label: "Disabilitato".to_string(), checked: is_disabled }
                                ToggleControl { label: "Icona Sinistra".to_string(), checked: show_left_icon }
                                ToggleControl { label: "Icona Destra".to_string(), checked: show_right_icon }
                                ToggleControl { label: "Larghezza 100%".to_string(), checked: is_full_width }
                                ToggleControl { label: "Attiva Glow".to_string(), checked: enable_glow }
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }

            Card {
                title: "Catalogo Dimensioni e Varianti".to_string(),
                subtitle: "Tutte le scale dimensionali con feedback tattile".to_string(),
                div { class: "sc-stack",
                    div { class: "sc-actions",
                        Button { size: ButtonSize::Xs, variant: ButtonVariant::Primary, "Extra Small" }
                        Button { size: ButtonSize::Sm, variant: ButtonVariant::Primary, "Small" }
                        Button { size: ButtonSize::Md, variant: ButtonVariant::Primary, "Medium" }
                        Button { size: ButtonSize::Lg, variant: ButtonVariant::Primary, "Large" }
                        Button { size: ButtonSize::Xl, variant: ButtonVariant::Primary, "Extra Large" }
                    }
                    hr { class: "sc-control-divider" }
                    div { class: "sc-actions",
                        Button { variant: ButtonVariant::Primary, "Primary" }
                        Button { variant: ButtonVariant::Secondary, "Secondary" }
                        Button { variant: ButtonVariant::Outline, "Outline" }
                        Button { variant: ButtonVariant::Ghost, "Ghost" }
                        Button { variant: ButtonVariant::Success, "Success" }
                        Button { variant: ButtonVariant::Danger, "Danger" }
                    }
                }
            }
        }
    }
}
