use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn NavbarDemoPage() -> Element {
    let props = vec![
        PropRow {
            name: "brand",
            r#type: "Element",
            default: "-",
            description: "Slot principale per il logo, titolo o identificatore dell'applicazione",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS/SCSS personalizzate per sovrascrivere o arricchire la barra",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Slot per la navigazione destra (link, pulsanti di azione, switcher tema)",
        },
    ];

    let code_snippet = r#"Navbar {
    brand: rsx! {
        div { class: "flex items-center gap-2",
            Icon { kind: IconKind::Terminal, size: 18 }
            strong { "AppStudio" }
        }
    },
    div { class: "flex items-center gap-2",
        Button { size: ButtonSize::Xs, variant: ButtonVariant::Ghost, "Dashboard" }
        Button { size: ButtonSize::Xs, variant: ButtonVariant::Ghost, "Progetti" }
        Button {
            size: ButtonSize::Sm,
            variant: ButtonVariant::Primary,
            "Accedi"
        }
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Navbar".to_string(),
                description: "Barra di navigazione superiore trasparente con sfocatura ottica (backdrop-blur-md), specular top highlight e contenitore responsive.".to_string(),
            }

            FeatureBanner { feature_name: "navbar" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "sc-demo-frame w-full",
                        Navbar {
                            brand: rsx! {
                                div { class: "flex items-center gap-2",
                                    Icon { kind: IconKind::Layers, size: 18 }
                                    div { class: "flex flex-col leading-none",
                                        strong { "ShadowOS" }
                                        span { class: "sc-subtle-text sc-mono", "Cloud Suite" }
                                    }
                                }
                            },
                            div { class: "flex items-center gap-2",
                                Button { size: ButtonSize::Xs, variant: ButtonVariant::Ghost, "Panoramica" }
                                Button { size: ButtonSize::Xs, variant: ButtonVariant::Ghost, "Componenti" }
                                Button { size: ButtonSize::Xs, variant: ButtonVariant::Ghost, "Documentazione" }
                                Button {
                                    size: ButtonSize::Xs,
                                    variant: ButtonVariant::Primary,
                                    left_icon: rsx! { Icon { kind: IconKind::Sparkles, size: 13 } },
                                    "Accedi"
                                }
                            }
                        }
                        p { class: "sc-page-desc",
                            "Area sottostante con scroll e supporto trasparenza sticky top-0."
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
