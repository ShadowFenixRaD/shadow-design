use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn CoachmarkPage() -> Element {
    let mut current_step = use_signal(|| 1usize);
    let mut is_tour_active = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "step",
            r#type: "usize",
            default: "-",
            description: "Numero del passo corrente nella sequenza di onboarding (1-indexed)",
        },
        PropRow {
            name: "total_steps",
            r#type: "usize",
            default: "-",
            description: "Numero totale di passi compresi nel tour guidato",
        },
        PropRow {
            name: "title",
            r#type: "String",
            default: "-",
            description: "Titolo primario del passo di guida",
        },
        PropRow {
            name: "headline",
            r#type: "Option<String>",
            default: "None",
            description: "Sottotitolo o categoria contestuale evidenziata",
        },
        PropRow {
            name: "on_next",
            r#type: "Option<EventHandler<()>>",
            default: "None",
            description: "Callback richiamato alla pressione del tasto Avanti o Completa",
        },
        PropRow {
            name: "on_prev",
            r#type: "Option<EventHandler<()>>",
            default: "None",
            description: "Callback richiamato alla pressione del tasto Indietro",
        },
        PropRow {
            name: "on_dismiss",
            r#type: "Option<EventHandler<()>>",
            default: "None",
            description: "Callback richiamato quando l'utente chiude la bolla con la X",
        },
    ];

    let code_snippet = r#"let mut step = use_signal(|| 1usize);

Coachmark {
    step: step(),
    total_steps: 3,
    title: "Esplora i Componenti".to_string(),
    headline: Some("Navigazione Rapida".to_string()),
    on_next: move |_| step.set((step() + 1).min(3)),
    on_prev: move |_| step.set((step() - 1).max(1)),
    on_dismiss: move |_| println!("Guida chiusa"),
    p { "Usa la barra laterale o la palette Ctrl+K per saltare rapidamente tra le sezioni." }
}"#
    .to_string();

    let step_info = match *current_step.read() {
        1 => (
            "Nuova Esperienza Modulare",
            "Benvenuto in Shadow Design",
            "Tutti i componenti sono isolati da feature flags Cargo per consentire bundle WASM estremamente leggeri.",
        ),
        2 => (
            "Palette & Tema Mantine",
            "Personalizzazione Runtime",
            "Puoi testare i 5 preset cromatici integrati e modificare border-radius e glow direttamente dalla barra superiore.",
        ),
        _ => (
            "Navigazione & Scorciatoie",
            "Produttività Immediata",
            "Premi la scorciatoia Ctrl+K o Cmd+K in qualsiasi momento per filtrare tutti i 50+ componenti ed entrare subito nella demo.",
        ),
    };

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Coachmark / TeachingBubble" }
                Text { dimmed: true, size: TextSize::Sm, "Bolla di guida passo-passo ancorata a elementi specifici per onboarding utenti, con puntatore, pulsanti Avanti/Indietro e contatore di step." }
            }

            FeatureBanner { feature_name: "coachmark" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-10 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-6 shadow-inner",
                        if !*is_tour_active.read() {
                            div { class: "text-center space-y-3 py-6",
                                p { class: "text-xs text-zinc-400", "Il tour guidato è stato completato o chiuso." }
                                Button {
                                    variant: ButtonVariant::Primary,
                                    left_icon: rsx! { Icon { kind: IconKind::Sparkles, size: 15 } },
                                    onclick: move |_| {
                                        current_step.set(1);
                                        is_tour_active.set(true);
                                    },
                                    "Riavvia Guida Interattiva"
                                }
                            }
                        } else {
                            Coachmark {
                                step: *current_step.read(),
                                total_steps: 3,
                                title: step_info.0.to_string(),
                                headline: Some(step_info.1.to_string()),
                                on_next: move |_| {
                                    let s = *current_step.read();
                                    if s >= 3 {
                                        is_tour_active.set(false);
                                    } else {
                                        current_step.set(s + 1);
                                    }
                                },
                                on_prev: move |_| {
                                    let s = *current_step.read();
                                    if s > 1 {
                                        current_step.set(s - 1);
                                    }
                                },
                                on_dismiss: move |_| is_tour_active.set(false),
                                p { "{step_info.2}" }
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
