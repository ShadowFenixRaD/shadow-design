use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, Counter, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs,
};

#[component]
pub fn CloseButtonPage() -> Element {
    let selected_size = use_signal(|| CloseButtonSize::Md);
    let selected_variant = use_signal(|| CloseButtonVariant::Subtle);
    let mut click_count = use_signal(|| 0usize);

    let props = vec![
        PropRow {
            name: "size",
            r#type: "CloseButtonSize",
            default: "CloseButtonSize::Md",
            description: "Dimensione del pulsante (Sm = 24px, Md = 32px, Lg = 40px)",
        },
        PropRow {
            name: "variant",
            r#type: "CloseButtonVariant",
            default: "CloseButtonVariant::Subtle",
            description: "Variante stilistica (Subtle, Filled, Transparent)",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita il pulsante e blocca le interazioni",
        },
        PropRow {
            name: "aria_label",
            r#type: "String",
            default: "\"Chiudi\"",
            description: "Etichetta di accessibilità per screen reader e tooltip nativo",
        },
    ];

    let code_snippet = r#"CloseButton {
    size: CloseButtonSize::Md,
    variant: CloseButtonVariant::Subtle,
    onclick: move |_| println!("Finestra chiusa"),
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "CloseButton".to_string(),
                description: "Pulsante di chiusura standardizzato con icona 'X' e feedback tattile, coerente su modali, notifiche toast, drawer e schede.".to_string(),
            }

            FeatureBanner { feature_name: "close_button" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            div { class: "sc-actions",
                                CloseButton {
                                    size: *selected_size.read(),
                                    variant: *selected_variant.read(),
                                    onclick: move |_| click_count += 1,
                                }
                                CloseButton {
                                    size: *selected_size.read(),
                                    variant: *selected_variant.read(),
                                    disabled: true,
                                }
                            }
                            Counter { label: "Click registrati".to_string(), value: click_count().to_string() }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Dimensione".to_string(),
                                options: vec![
                                    (CloseButtonSize::Sm, "Sm".to_string()),
                                    (CloseButtonSize::Md, "Md".to_string()),
                                    (CloseButtonSize::Lg, "Lg".to_string()),
                                ],
                                value: selected_size,
                            }
                            ChoicePicker {
                                label: "Variante".to_string(),
                                options: vec![
                                    (CloseButtonVariant::Subtle, "Subtle".to_string()),
                                    (CloseButtonVariant::Filled, "Filled".to_string()),
                                    (CloseButtonVariant::Transparent, "Transparent".to_string()),
                                ],
                                value: selected_variant,
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
