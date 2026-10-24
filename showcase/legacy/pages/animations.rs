use dioxus::prelude::*;
use shadow_design::prelude::*;
use crate::components::{ChoicePicker, PageHeader, Playground, PropRow, ShowcaseTabs};

#[component]
pub fn AnimationsPage() -> Element {
    let selected_variant = use_signal(|| AnimationVariant::FadeIn);
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
        class: "sc-demo-cell",
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
        Playground {
            stage: rsx! {
                div {
                    key: "stage-{replay_counter()}-{selected_variant():?}",
                    Transition {
                        variant: selected_variant(),
                        Card {
                            div { class: "sc-stack",
                                Icon { kind: IconKind::Sparkles, size: 28 }
                                span { class: "sc-section-title", "Animation: {selected_variant():?}" }
                                span { class: "sc-subtle-text sc-mono", "{selected_variant().classes()}" }
                            }
                        }
                    }
                }
            },
            controls: rsx! {
                ChoicePicker {
                    label: "Animazione".to_string(),
                    options: variants.iter().map(|v| (*v, format!("{v:?}"))).collect::<Vec<_>>(),
                    value: selected_variant,
                    on_change: move |_| replay_counter += 1,
                }
                Button {
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Sm,
                    onclick: move |_| replay_counter += 1,
                    Icon { kind: IconKind::Sparkles, size: 14 }
                    "Riproduci Animazione"
                }
            },
        }
    };

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Animation & Transitions".to_string(),
                description: "Sistema di micro-animazioni Dioxus ad alte prestazioni con accelerazione hardware GPU a 60fps.".to_string(),
            }
            ShowcaseTabs {
                preview: preview,
                code: code_snippet,
                props: props_rows,
            }
        }
    }
}
