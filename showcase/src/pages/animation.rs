use dioxus::prelude::*;
use shadow_design::prelude::*;
use crate::components::ShowcaseTabs;
use crate::components::props_table::PropRow;

#[component]
pub fn AnimationsPage() -> Element {
    let mut selected_variant = use_signal(|| AnimationVariant::FadeIn);
    let mut replay_counter = use_signal(|| 0usize);

    let variants = [
        AnimationVariant::FadeIn,
        AnimationVariant::FadeOut,
        AnimationVariant::SlideInRight,
        AnimationVariant::SlideInLeft,
        AnimationVariant::SlideInBottom,
        AnimationVariant::SlideInTop,
        AnimationVariant::ScaleIn,
        AnimationVariant::ModalEnter,
        AnimationVariant::ToastEnter,
        AnimationVariant::ToastExit,
        AnimationVariant::DropdownEnter,
        AnimationVariant::DrawerSlideLeft,
        AnimationVariant::DrawerSlideRight,
        AnimationVariant::PulseGlow,
        AnimationVariant::TactilePress,
        AnimationVariant::Spin,
        AnimationVariant::Indeterminate,
    ];

    let code_snippet = format!(
        r#"Transition {{
    variant: AnimationVariant::{:?},
    div {{
        class: "p-6 rounded-xl bg-purple-950/40 border border-purple-500/30",
        "Animazione attiva"
    }}
}}"#,
        selected_variant()
    );

    let props_rows = vec![
        PropRow {
            name: "variant",
            r#type: "AnimationVariant",
            default: "FadeIn",
            description: "Tipologia di micro-animazione o transizione (Fade, Slide, Scale, Pulse, Spin).",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate all'elemento contenitore.",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Elementi o componenti racchiusi all'interno della transizione.",
        },
    ];

    let preview = rsx! {
        div { class: "flex flex-col gap-6",
            // Controls
            div { class: "flex flex-wrap items-center justify-between gap-4 p-4 rounded-xl border border-white/[0.08] bg-[#0c0c12]/80",
                div { class: "flex flex-wrap items-center gap-1.5",
                    for v in variants {
                        button {
                            key: "{v:?}",
                            r#type: "button",
                            class: if selected_variant() == v {
                                "px-3 py-1.5 text-xs font-mono font-medium rounded-lg bg-purple-950/80 text-purple-200 border border-purple-500/40 cursor-pointer shadow-sm"
                            } else {
                                "px-3 py-1.5 text-xs font-mono text-zinc-400 hover:text-white rounded-lg bg-white/[0.02] hover:bg-white/[0.06] border border-white/[0.06] cursor-pointer transition-colors"
                            },
                            onclick: move |_| {
                                selected_variant.set(v);
                                replay_counter += 1;
                            },
                            "{v:?}"
                        }
                    }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Sm,
                    onclick: move |_| replay_counter += 1,
                    Icon { kind: IconKind::Sparkles, size: 14 }
                    "Riproduci Animazione"
                }
            }

            // Stage
            div { class: "flex items-center justify-center p-12 min-h-[240px] rounded-2xl border border-white/[0.06] bg-[#060608] relative overflow-hidden",
                div {
                    key: "stage-{replay_counter()}-{selected_variant():?}",
                    Transition {
                        variant: selected_variant(),
                        class: "sd-motion-demo cursor-pointer",
                        div { class: "p-8 rounded-2xl border border-purple-500/40 bg-purple-950/30 text-purple-200 shadow-[0_0_24px_rgba(168,85,247,0.25)] flex flex-col items-center gap-2 select-none",
                            Icon { kind: IconKind::Sparkles, size: 28 }
                            span { class: "font-semibold text-sm", "Animation: {selected_variant():?}" }
                            span { class: "text-xs font-mono text-purple-400/80", "{selected_variant().classes()}" }
                        }
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "flex flex-col gap-2",
                Heading { order: HeadingOrder::H1, "Animation & Transitions" }
                Text { size: TextSize::Md, class: "text-zinc-400",
                    "Sistema di micro-animazioni Dioxus ad alte prestazioni con accelerazione hardware GPU a 60fps."
                }
            }
            ShowcaseTabs {
                preview: preview,
                code: code_snippet,
                props: props_rows,
            }
        }
    }
}
