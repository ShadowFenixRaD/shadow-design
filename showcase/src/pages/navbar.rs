use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

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
            description: "Classi Tailwind personalizzate per sovrascrivere o arricchire la barra",
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
            span { class: "font-bold text-white tracking-tight", "AppStudio" }
        }
    },
    div { class: "flex items-center gap-4 text-xs font-medium text-zinc-300",
        span { class: "hover:text-white cursor-pointer transition-colors", "Dashboard" }
        span { class: "hover:text-white cursor-pointer transition-colors", "Progetti" }
        Button {
            size: ButtonSize::Sm,
            variant: ButtonVariant::Primary,
            "Accedi"
        }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Navbar" }
                Text { dimmed: true, size: TextSize::Sm, "Barra di navigazione superiore trasparente con sfocatura ottica (backdrop-blur-md), specular top highlight e contenitore responsive." }
            }

            FeatureBanner { feature_name: "navbar" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "rounded-xl border border-white/[0.08] overflow-hidden bg-[#030305] shadow-2xl",
                        Navbar {
                            brand: rsx! {
                                div { class: "flex items-center gap-2.5",
                                    div { class: "w-7 h-7 rounded-lg bg-purple-600/30 border border-purple-500/40 flex items-center justify-center text-purple-300",
                                        Icon { kind: IconKind::Layers, size: 16 }
                                    }
                                    div { class: "flex flex-col leading-none",
                                        span { class: "font-bold text-white text-sm tracking-tight", "ShadowOS" }
                                        span { class: "text-[9px] font-mono text-zinc-500 uppercase", "Cloud Suite" }
                                    }
                                }
                            },
                            div { class: "flex items-center gap-4 text-xs font-medium text-zinc-400",
                                span { class: "hover:text-white cursor-pointer transition-colors hidden sm:inline", "Panoramica" }
                                span { class: "hover:text-white cursor-pointer transition-colors hidden sm:inline", "Componenti" }
                                span { class: "hover:text-white cursor-pointer transition-colors hidden sm:inline", "Documentazione" }
                                Button {
                                    size: ButtonSize::Xs,
                                    variant: ButtonVariant::Primary,
                                    left_icon: rsx! { Icon { kind: IconKind::Sparkles, size: 13 } },
                                    "Accedi"
                                }
                            }
                        }
                        div { class: "p-8 text-center text-zinc-500 text-xs font-mono",
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
