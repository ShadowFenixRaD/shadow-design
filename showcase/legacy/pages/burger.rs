use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, Counter, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs,
};

#[component]
pub fn BurgerPage() -> Element {
    let opened = use_signal(|| false);
    let size = use_signal(|| BurgerSize::Md);

    let props = vec![
        PropRow {
            name: "opened",
            r#type: "Signal<bool>",
            default: "-",
            description: "Signal reattivo che controlla lo stato aperto (icona a X) o chiuso (hamburger a 3 righe)",
        },
        PropRow {
            name: "size",
            r#type: "BurgerSize",
            default: "BurgerSize::Md",
            description: "Dimensioni del pulsante (Sm = 18px, Md = 24px, Lg = 32px)",
        },
        PropRow {
            name: "color",
            r#type: "Option<String>",
            default: "None",
            description: "Colore personalizzato per le linee del burger",
        },
        PropRow {
            name: "aria_label",
            r#type: "String",
            default: "\"Toggle navigation\"",
            description: "Etichetta accessibile per screen reader",
        },
    ];

    let code_snippet = r#"let mut opened = use_signal(|| false);

Burger {
    opened: opened,
    size: BurgerSize::Md,
    aria_label: "Menu di navigazione".to_string(),
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Burger".to_string(),
                description: "Pulsante hamburger animato a 3 linee con transizione ottica fluida in icona di chiusura 'X', ideale per trigger di navigazione mobile e sidebar.".to_string(),
            }

            FeatureBanner { feature_name: "burger" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            Burger {
                                opened: opened,
                                size: *size.read(),
                            }
                            Counter {
                                label: "Stato corrente".to_string(),
                                value: if *opened.read() { "Aperto (X)".to_string() } else { "Chiuso (Hamburger)".to_string() },
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Dimensione".to_string(),
                                options: vec![
                                    (BurgerSize::Sm, "Sm (18px)".to_string()),
                                    (BurgerSize::Md, "Md (24px)".to_string()),
                                    (BurgerSize::Lg, "Lg (32px)".to_string()),
                                ],
                                value: size,
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
