use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs,
};

#[component]
pub fn AffixPage() -> Element {
    let selected_pos = use_signal(|| AffixPosition::BottomRight);

    let props = vec![
        PropRow {
            name: "position",
            r#type: "AffixPosition",
            default: "AffixPosition::BottomRight",
            description: "Angolo di ancoraggio fisso nella viewport (BottomRight, BottomLeft, TopRight, TopLeft)",
        },
        PropRow {
            name: "scroll_threshold",
            r#type: "Option<u32>",
            default: "None",
            description: "Soglia minima di scroll della pagina in pixel per attivare la visibilità dell'elemento",
        },
        PropRow {
            name: "z_index",
            r#type: "u32",
            default: "60",
            description: "Livello di elevazione z-index per sovrapporsi ai contenuti di pagina",
        },
        PropRow {
            name: "BackToTop",
            r#type: "Component",
            default: "-",
            description: "Pulsante autonomo con scroll fluido a inizio pagina (window.scrollTo)",
        },
    ];

    let code_snippet = r#"// Affix generico con soglia di scroll
Affix {
    position: AffixPosition::BottomRight,
    scroll_threshold: Some(250),
    Button {
        variant: ButtonVariant::Primary,
        left_icon: rsx! { Icon { kind: IconKind::Terminal, size: 14 } },
        "Terminale Rapido"
    }
}

// Oppure il componente dedicato BackToTop
BackToTop {
    scroll_threshold: 200,
    label: Some("Torna su".to_string()),
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Affix & BackToTop".to_string(),
                description: "Contenitore fisso in overlay a coordinate assolute dello schermo con attivazione condizionale allo scroll e componente rapido BackToTop.".to_string(),
            }

            FeatureBanner { feature_name: "affix" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            p { class: "sc-muted-text",
                                "Il componente Affix fissa qualsiasi elemento a una posizione della viewport. Il pulsante BackToTop rileva lo scroll verticale della pagina ed effettua uno scroll fluido verso l'alto."
                            }
                            button {
                                r#type: "button",
                                class: "sd-back-to-top",
                                onclick: move |_| {
                                    let eval = document::eval("window.scrollTo({ top: 0, behavior: 'smooth' });");
                                    spawn(async move { let _ = eval.await; });
                                },
                                Icon { kind: IconKind::ChevronUp, size: 16 }
                                span { "Anteprima BackToTop" }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Angolo".to_string(),
                                options: vec![
                                    (AffixPosition::BottomRight, "Bottom-Right".to_string()),
                                    (AffixPosition::BottomLeft, "Bottom-Left".to_string()),
                                ],
                                value: selected_pos,
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Componente BackToTop attivo sullo scroll reale della pagina
            BackToTop {
                scroll_threshold: 150,
                label: Some("Inizio pagina".to_string()),
            }
        }
    }
}
