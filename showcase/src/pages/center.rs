use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::banner::FeatureBanner;
use crate::components::props_table::PropRow;
use crate::components::showcase_tabs::ShowcaseTabs;

#[component]
pub fn CenterPage() -> Element {
    let mut inline = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "inline",
            r#type: "bool",
            default: "false",
            description: "Se impostato a true, utilizza display: inline-flex anziché display: flex.",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate al contenitore Center.",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Elementi figli posizionati ed allineati esattamente al centro.",
        },
    ];

    let code_snippet = r#"Center {
    class: "h-48 w-full rounded-xl bg-purple-950/20 border border-purple-800/30",
    div { class: "flex flex-col items-center gap-2",
        Icon { kind: IconKind::Check, size: 28 }
        span { class: "font-mono text-sm text-purple-200 font-bold", "Allineato al centro" }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Center" }
                Text { dimmed: true, size: TextSize::Sm, "Contenitore utility per centrare istantaneamente il contenuto sia in orizzontale che in verticale senza codice CSS boilerplate, con supporto per layout a blocco o inline." }
            }

            FeatureBanner { feature_name: "center" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6 w-full",
                        // Live Center Preview
                        div { class: "w-full",
                            if inline() {
                                div { class: "p-6 rounded-2xl border border-white/10 bg-zinc-950 text-center space-x-2",
                                    span { class: "text-zinc-400 text-sm align-middle", "Testo prima dell'elemento" }
                                    Center {
                                        inline: true,
                                        class: "px-4 py-2 rounded-lg bg-purple-950/40 border border-purple-800/50 text-purple-200 gap-2 align-middle",
                                        Icon { kind: IconKind::Layers, size: 16 }
                                        span { class: "text-xs font-mono font-bold", "Inline Centered" }
                                    }
                                    span { class: "text-zinc-400 text-sm align-middle", "Testo dopo l'elemento" }
                                }
                            } else {
                                Center {
                                    class: "h-56 w-full rounded-2xl border border-white/10 bg-zinc-950 shadow-inner",
                                    div { class: "p-6 rounded-xl bg-purple-950/40 border border-purple-800/40 flex flex-col items-center justify-center text-center max-w-xs",
                                        span { class: "p-3 rounded-full bg-purple-500/10 border border-purple-500/20 text-purple-300 mb-2 flex items-center justify-center",
                                            Icon { kind: IconKind::Check, size: 28 }
                                        }
                                        span { class: "text-sm font-bold text-white font-mono", "Perfettamente Centrato" }
                                        span { class: "text-xs text-purple-300 font-mono mt-1", "display: flex; justify-center; items-center;" }
                                        div { class: "mt-3 flex items-center gap-2",
                                            Badge { label: "H-Center".to_string(), variant: BadgeVariant::Primary }
                                            Badge { label: "V-Center".to_string(), variant: BadgeVariant::Success }
                                        }
                                    }
                                }
                            }
                        }

                        // Configurator Controls
                        div { class: "p-4 rounded-xl bg-white/[0.02] border border-white/[0.08] space-y-4",
                            span { class: "text-xs font-mono uppercase tracking-wider text-zinc-400 font-semibold block", "Modalità Display" }
                            div { class: "flex flex-wrap gap-2",
                                Button {
                                    size: ButtonSize::Sm,
                                    variant: if !inline() { ButtonVariant::Primary } else { ButtonVariant::Outline },
                                    onclick: move |_| inline.set(false),
                                    "Block (display: flex)"
                                }
                                Button {
                                    size: ButtonSize::Sm,
                                    variant: if inline() { ButtonVariant::Primary } else { ButtonVariant::Outline },
                                    onclick: move |_| inline.set(true),
                                    "Inline (display: inline-flex)"
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
