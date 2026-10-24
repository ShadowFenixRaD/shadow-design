//! Story: SpeedDial and Fab (feature `speed_dial`). The dial owns its open state; the playground
//! goes through `SpeedDialDemo { knobs }` and sits under a small draft card.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const DIRECTIONS: [SpeedDialDirection; 4] =
    [SpeedDialDirection::Up, SpeedDialDirection::Down, SpeedDialDirection::Left, SpeedDialDirection::Right];
const DIRECTION_OPTS: &[ChoiceOption] = &[
    opt("Up", "SpeedDialDirection::Up"),
    opt("Down", "SpeedDialDirection::Down"),
    opt("Left", "SpeedDialDirection::Left"),
    opt("Right", "SpeedDialDirection::Right"),
];
const ICONS: [IconKind; 3] = [IconKind::Plus, IconKind::Share2, IconKind::MoreHorizontal];
const ICON_OPTS: &[ChoiceOption] =
    &[opt("Plus", "IconKind::Plus"), opt("Share2", "IconKind::Share2"), opt("MoreHorizontal", "IconKind::MoreHorizontal")];

const PRELUDE: &str = "let mut toast = use_notification();";

const ACTIONS_CODE: &str = r#"SpeedDialAction { icon: IconKind::Send, label: Some("Pubblica".to_string()), onclick: move |_| { toast.success("Post-mortem pubblicato"); } }
SpeedDialAction { icon: IconKind::Paperclip, label: Some("Allega i log".to_string()), onclick: move |_| { toast.info("Log allegati"); } }
SpeedDialAction { icon: IconKind::Share2, label: Some("Condividi".to_string()), onclick: move |_| { toast.info("Link copiato"); } }"#;

#[component]
fn SpeedDialDemo(knobs: Knobs) -> Element {
    let mut toast = use_notification();
    // close_on_action is read once when the dial mounts: remount on change.
    let remount = knobs.bool("close_on_action").to_string();
    rsx! {
        Stack { gap: SpacingToken::Md,
            Card { title: Some("Post-mortem INC-2041".to_string()), subtitle: Some("Bozza salvata 2 minuti fa".to_string()),
                Text { size: TextSize::Sm, dimmed: true, "Picco di latenza su relay-03 dalle 08:10 alle 08:52; traffico reindirizzato su edge-12." }
            }
            Group { justify: JustifyContent::End,
                for key in [remount] {
                    SpeedDial {
                        key: "{key}",
                        direction: knobs.pick("direction", &DIRECTIONS),
                        icon: knobs.pick("icon", &ICONS),
                        active_icon: knobs.bool("active_icon").then_some(IconKind::X),
                        close_on_action: knobs.bool("close_on_action"),
                        close_on_click_outside: knobs.bool("close_on_click_outside"),
                        SpeedDialAction {
                            icon: IconKind::Send,
                            label: Some("Pubblica".to_string()),
                            onclick: move |_| {
                                toast.success("Post-mortem pubblicato");
                            },
                        }
                        SpeedDialAction {
                            icon: IconKind::Paperclip,
                            label: Some("Allega i log".to_string()),
                            onclick: move |_| {
                                toast.info("Log allegati");
                            },
                        }
                        SpeedDialAction {
                            icon: IconKind::Share2,
                            label: Some("Condividi".to_string()),
                            onclick: move |_| {
                                toast.info("Link copiato");
                            },
                        }
                    }
                }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        SpeedDialDemo { knobs: k.clone() }
    }
}

#[component]
fn SingleFab() -> Element {
    let mut toast = use_notification();
    rsx! {
        Group { gap: SpacingToken::Md, justify: JustifyContent::Center,
            Fab {
                icon: IconKind::Plus,
                title: Some("Nuovo nodo".to_string()),
                onclick: move |_| {
                    toast.success("Nodo edge-22 in provisioning");
                },
            }
            Fab {
                icon: IconKind::MessageSquare,
                title: Some("Scrivi al reperibile".to_string()),
                onclick: move |_| {
                    toast.info("Chat aperta");
                },
            }
        }
    }
}
const SINGLE_FAB_CODE: &str = r#"let mut toast = use_notification();

Group { gap: SpacingToken::Md, justify: JustifyContent::Center,
    Fab {
        icon: IconKind::Plus,
        title: Some("Nuovo nodo".to_string()),
        onclick: move |_| {
            toast.success("Nodo edge-22 in provisioning");
        },
    }
    Fab {
        icon: IconKind::MessageSquare,
        title: Some("Scrivi al reperibile".to_string()),
        onclick: move |_| {
            toast.info("Chat aperta");
        },
    }
}"#;

#[component]
fn Horizontal() -> Element {
    rsx! {
        Group {
            SpeedDial { direction: SpeedDialDirection::Right, icon: IconKind::Share2, active_icon: Some(IconKind::X),
                SpeedDialAction { icon: IconKind::Copy, label: Some("Copia link".to_string()) }
                SpeedDialAction { icon: IconKind::Send, label: Some("Invia".to_string()) }
                SpeedDialAction { icon: IconKind::QrCode, label: Some("Codice QR".to_string()) }
            }
        }
    }
}
const HORIZONTAL_CODE: &str = r#"Group {
    SpeedDial { direction: SpeedDialDirection::Right, icon: IconKind::Share2, active_icon: Some(IconKind::X),
        SpeedDialAction { icon: IconKind::Copy, label: Some("Copia link".to_string()) }
        SpeedDialAction { icon: IconKind::Send, label: Some("Invia".to_string()) }
        SpeedDialAction { icon: IconKind::QrCode, label: Some("Codice QR".to_string()) }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut note = use_signal(|| "Causa: tratta Milano → Parigi satura.".to_string());
    rsx! {
        Card { title: Some("Note dell'incidente".to_string()), subtitle: Some("INC-2041 · relay-03".to_string()),
            Stack { gap: SpacingToken::Md,
                Textarea { value: note, label: Some("Cronologia".to_string()), min_rows: 4, autosize: true }
                Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
                    Text { size: TextSize::Xs, dimmed: true, "Inserisci blocchi con il pulsante a destra" }
                    SpeedDial { direction: SpeedDialDirection::Left, active_icon: Some(IconKind::X),
                        SpeedDialAction {
                            icon: IconKind::Clock,
                            label: Some("Orario".to_string()),
                            onclick: move |_| note.write().push_str("\n08:52 · "),
                        }
                        SpeedDialAction {
                            icon: IconKind::Code,
                            label: Some("Comando".to_string()),
                            onclick: move |_| note.write().push_str("\n$ mesh route --via edge-12"),
                        }
                        SpeedDialAction {
                            icon: IconKind::CheckCircle,
                            label: Some("Risolto".to_string()),
                            onclick: move |_| note.write().push_str("\nStato: risolto."),
                        }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut note = use_signal(|| "Causa: tratta Milano → Parigi satura.".to_string());

Card { title: Some("Note dell'incidente".to_string()), subtitle: Some("INC-2041 · relay-03".to_string()),
    Stack { gap: SpacingToken::Md,
        Textarea { value: note, label: Some("Cronologia".to_string()), min_rows: 4, autosize: true }
        Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
            Text { size: TextSize::Xs, dimmed: true, "Inserisci blocchi con il pulsante a destra" }
            SpeedDial { direction: SpeedDialDirection::Left, active_icon: Some(IconKind::X),
                SpeedDialAction {
                    icon: IconKind::Clock,
                    label: Some("Orario".to_string()),
                    onclick: move |_| note.write().push_str("\n08:52 · "),
                }
                SpeedDialAction {
                    icon: IconKind::Code,
                    label: Some("Comando".to_string()),
                    onclick: move |_| note.write().push_str("\n$ mesh route --via edge-12"),
                }
                SpeedDialAction {
                    icon: IconKind::CheckCircle,
                    label: Some("Risolto".to_string()),
                    onclick: move |_| note.write().push_str("\nStato: risolto."),
                }
            }
        }
    }
}"#;

const DIAL_PROPS: &[PropDoc] = &[
    prop("icon", "IconKind", "Plus", "Icona del pulsante principale."),
    prop("active_icon", "Option<IconKind>", "None", "Icona da aperto; senza, l'icona ruota di 45°."),
    prop("direction", "SpeedDialDirection", "Up", "Verso di apertura delle azioni: Up, Down, Left, Right."),
    prop("opened", "Option<Signal<bool>>", "None", "Stato controllato dall'esterno; senza, il dial gestisce il proprio."),
    prop("close_on_action", "bool", "true", "Chiude il dial dopo il clic su un'azione."),
    prop("close_on_click_outside", "bool", "true", "Chiude il dial con un clic fuori."),
    prop("fixed", "bool", "false", "Ancorato in basso a destra della finestra (z('speed-dial'))."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
    prop("children", "Element", "-", "Le SpeedDialAction."),
];
const ACTION_PROPS: &[PropDoc] = &[
    prop("icon", "IconKind", "-", "Icona del pulsante d'azione."),
    prop("label", "Option<String>", "None", "Etichetta accanto al pulsante e suo aria-label."),
    prop("onclick", "Option<EventHandler<()>>", "None", "Azione."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul pulsante."),
];
const FAB_PROPS: &[PropDoc] = &[
    prop("icon", "IconKind", "-", "Icona del pulsante flottante."),
    prop("title", "Option<String>", "None", "Testo del tooltip nativo."),
    prop("onclick", "Option<EventHandler<()>>", "None", "Azione."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "speed-dial",
        name: "SpeedDial",
        category: Category::Overlay,
        features: &["speed_dial"],
        icon: IconKind::Plus,
        tagline: "Pulsante d'azione flottante che apre, in una direzione, le azioni correlate più frequenti.",
        keywords: &["speed dial", "fab", "floating action button", "azioni rapide", "pulsante flottante", "menu"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "SpeedDial",
            controls: vec![
                Control::choice("direction", "Direzione", DIRECTION_OPTS, 0),
                Control::choice("icon", "Icona", ICON_OPTS, 0),
                Control::bool("active_icon", "Icona X da aperto", false).emit(Emit::Template("active_icon: Some(IconKind::X)")),
                Control::bool("close_on_action", "Chiudi dopo l'azione", true),
                Control::bool("close_on_click_outside", "Chiudi al clic fuori", true),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &[],
            children_code: Some(ACTIONS_CODE),
            stage: Stage::Fill,
        }),
        examples: vec![
            Example {
                id: "fab",
                title: "Fab",
                description: "Un'unica azione flottante, senza menu.",
                code: SINGLE_FAB_CODE,
                render: || rsx! { SingleFab {} },
                span: Span::Half,
            },
            Example {
                id: "horizontal",
                title: "In orizzontale",
                description: "Azioni verso destra, icona X da aperto.",
                code: HORIZONTAL_CODE,
                render: || rsx! { Horizontal {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Inserimento rapido di blocchi nelle note di un incidente.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "SpeedDial", rows: DIAL_PROPS },
            PropGroup { component: "SpeedDialAction", rows: ACTION_PROPS },
            PropGroup { component: "Fab", rows: FAB_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Raised control (language.md §3.5)",
            depth: "Lit control",
            tokens: &["--sd-accent", "--sd-accent-deep", "--sd-accent-fg", "--sd-glow-sm", "--sd-radius-full", "--sd-surface-3"],
            notes: "Il pulsante principale è l'azione primaria (pill, lit fill); le azioni figlie sono secondarie.",
        }),
        a11y: &[
            "Il pulsante principale è un <button> nativo con aria-expanded sincronizzato.",
            "Ogni SpeedDialAction usa label come aria-label: senza label l'azione resta senza nome, quindi passala sempre.",
            "Le etichette visibili sono aria-hidden per non essere lette due volte.",
            "Fab usa title come suggerimento nativo: per gli screen reader preferisci una SpeedDialAction etichettata o un Button con aria_label.",
        ],
        related: &["buttons", "dropdown", "affix", "tooltip"],
    }
}
