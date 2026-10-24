//! Story: MessageBubble (feature `chat`). Simple component: the playground render is stateless.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const DIRECTIONS: [BubbleDirection; 2] = [BubbleDirection::Incoming, BubbleDirection::Outgoing];
const DIRECTION_OPTS: &[ChoiceOption] =
    &[opt("Ricevuto", "BubbleDirection::Incoming"), opt("Inviato", "BubbleDirection::Outgoing")];
const STATUSES: [Option<DeliveryStatus>; 5] = [
    None,
    Some(DeliveryStatus::Sending),
    Some(DeliveryStatus::Sent),
    Some(DeliveryStatus::Delivered),
    Some(DeliveryStatus::Failed),
];
const STATUS_OPTS: &[ChoiceOption] = &[
    opt("Nessuno", "None"),
    opt("Sending", "Some(DeliveryStatus::Sending)"),
    opt("Sent", "Some(DeliveryStatus::Sent)"),
    opt("Delivered", "Some(DeliveryStatus::Delivered)"),
    opt("Failed", "Some(DeliveryStatus::Failed)"),
];

fn render(k: &Knobs) -> Element {
    rsx! {
        MessageBubble {
            direction: k.pick("direction", &DIRECTIONS),
            sender_name: Some(k.text("sender_name")).filter(|s| !s.is_empty()),
            timestamp: Some(k.text("timestamp")).filter(|s| !s.is_empty()),
            status: k.pick("status", &STATUSES),
            badge: k.bool("badge").then(|| rsx! {
                Icon { kind: IconKind::ShieldCheck, size: 11 }
            }),
            {k.text("text")}
        }
    }
}

#[component]
fn Delivery() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Sm,
            MessageBubble { direction: BubbleDirection::Outgoing, timestamp: Some("09:41".to_string()), status: Some(DeliveryStatus::Sending), "Riavvio edge-07 tra 5 minuti." }
            MessageBubble { direction: BubbleDirection::Outgoing, timestamp: Some("09:40".to_string()), status: Some(DeliveryStatus::Sent), "Patch applicata su staging." }
            MessageBubble { direction: BubbleDirection::Outgoing, timestamp: Some("09:38".to_string()), status: Some(DeliveryStatus::Delivered), "Log allegati al ticket." }
            MessageBubble { direction: BubbleDirection::Outgoing, timestamp: Some("09:35".to_string()), status: Some(DeliveryStatus::Failed), "Il peer non risponde." }
        }
    }
}
const DELIVERY_CODE: &str = r#"Stack { gap: SpacingToken::Sm,
    MessageBubble { direction: BubbleDirection::Outgoing, timestamp: Some("09:41".to_string()), status: Some(DeliveryStatus::Sending), "Riavvio edge-07 tra 5 minuti." }
    MessageBubble { direction: BubbleDirection::Outgoing, timestamp: Some("09:40".to_string()), status: Some(DeliveryStatus::Sent), "Patch applicata su staging." }
    MessageBubble { direction: BubbleDirection::Outgoing, timestamp: Some("09:38".to_string()), status: Some(DeliveryStatus::Delivered), "Log allegati al ticket." }
    MessageBubble { direction: BubbleDirection::Outgoing, timestamp: Some("09:35".to_string()), status: Some(DeliveryStatus::Failed), "Il peer non risponde." }
}"#;

#[component]
fn Senders() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Sm,
            MessageBubble {
                sender_name: Some("Giulia Ferri".to_string()),
                timestamp: Some("10:02".to_string()),
                "La latenza verso Francoforte è salita a 84 ms."
            }
            MessageBubble {
                sender_name: Some("relay-02".to_string()),
                timestamp: Some("10:03".to_string()),
                badge: rsx! {
                    Badge { label: "bot".to_string(), variant: BadgeVariant::Outline }
                },
                "Instradamento alternativo attivo via Milano."
            }
        }
    }
}
const SENDERS_CODE: &str = r#"Stack { gap: SpacingToken::Sm,
    MessageBubble {
        sender_name: Some("Giulia Ferri".to_string()),
        timestamp: Some("10:02".to_string()),
        "La latenza verso Francoforte è salita a 84 ms."
    }
    MessageBubble {
        sender_name: Some("relay-02".to_string()),
        timestamp: Some("10:03".to_string()),
        badge: rsx! {
            Badge { label: "bot".to_string(), variant: BadgeVariant::Outline }
        },
        "Instradamento alternativo attivo via Milano."
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut messages = use_signal(|| {
        vec![
            (BubbleDirection::Incoming, "Ciao! Il nodo edge-07 è di nuovo raggiungibile?".to_string(), "11:20"),
            (BubbleDirection::Outgoing, "Sì, il relay ha ripreso alle 11:18.".to_string(), "11:21"),
            (BubbleDirection::Incoming, "Perfetto, chiudo l'incidente.".to_string(), "11:22"),
        ]
    });
    let mut draft = use_signal(String::new);
    rsx! {
        Card {
            title: Some("Supporto · Giulia Ferri".to_string()),
            subtitle: Some("Canale cifrato end-to-end".to_string()),
            avatar: rsx! {
                Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm, status: Some(PresenceStatus::Online) }
            },
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, wrap: false, align: AlignItems::Center,
                    TextInput { value: draft, placeholder: Some("Scrivi un messaggio".to_string()), class: "w-full".to_string() }
                    Button {
                        aria_label: Some("Invia".to_string()),
                        disabled: draft().trim().is_empty(),
                        onclick: move |_| {
                            let text = draft();
                            messages.write().push((BubbleDirection::Outgoing, text, "ora"));
                            draft.set(String::new());
                        },
                        Icon { kind: IconKind::Send, size: 16 }
                    }
                }
            }),
            Stack { gap: SpacingToken::Sm,
                for (direction, text, time) in messages() {
                    MessageBubble {
                        direction,
                        timestamp: Some(time.to_string()),
                        status: (direction == BubbleDirection::Outgoing).then_some(DeliveryStatus::Delivered),
                        "{text}"
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut messages = use_signal(|| {
    vec![
        (BubbleDirection::Incoming, "Ciao! Il nodo edge-07 è di nuovo raggiungibile?".to_string(), "11:20"),
        (BubbleDirection::Outgoing, "Sì, il relay ha ripreso alle 11:18.".to_string(), "11:21"),
        (BubbleDirection::Incoming, "Perfetto, chiudo l'incidente.".to_string(), "11:22"),
    ]
});
let mut draft = use_signal(String::new);

Card {
    title: Some("Supporto · Giulia Ferri".to_string()),
    subtitle: Some("Canale cifrato end-to-end".to_string()),
    avatar: rsx! {
        Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm, status: Some(PresenceStatus::Online) }
    },
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, wrap: false, align: AlignItems::Center,
            TextInput { value: draft, placeholder: Some("Scrivi un messaggio".to_string()), class: "w-full".to_string() }
            Button {
                aria_label: Some("Invia".to_string()),
                disabled: draft().trim().is_empty(),
                onclick: move |_| {
                    let text = draft();
                    messages.write().push((BubbleDirection::Outgoing, text, "ora"));
                    draft.set(String::new());
                },
                Icon { kind: IconKind::Send, size: 16 }
            }
        }
    }),
    Stack { gap: SpacingToken::Sm,
        for (direction, text, time) in messages() {
            MessageBubble {
                direction,
                timestamp: Some(time.to_string()),
                status: (direction == BubbleDirection::Outgoing).then_some(DeliveryStatus::Delivered),
                "{text}"
            }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Contenuto del messaggio: testo, link o un AttachmentCard."),
    prop("direction", "BubbleDirection", "Incoming", "Incoming (a sinistra, superficie) o Outgoing (a destra, accento)."),
    prop("sender_name", "Option<String>", "None", "Mittente sopra il testo, utile nei gruppi."),
    prop("timestamp", "Option<String>", "None", "Ora già formattata."),
    prop("status", "Option<DeliveryStatus>", "None", "Sending, Sent, Delivered o Failed, con icona e title da Locale."),
    prop("badge", "Option<Element>", "None", "Piccolo elemento accanto all'ora (icona, Badge)."),
    prop("class", "String", "\"\"", "Classi aggiuntive sulla bolla."),
];

pub fn story() -> Story {
    Story {
        slug: "chat",
        name: "MessageBubble",
        category: Category::Contenuti,
        features: &["chat"],
        icon: IconKind::MessageSquare,
        tagline: "Bolle di messaggio per conversazioni, con mittente, ora e stato di consegna.",
        keywords: &["chat", "messaggio", "bolla", "conversazione", "message bubble", "consegna"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "MessageBubble",
            controls: vec![
                Control::text("text", "Testo", "Tunnel cifrato stabilito con relay-02.").emit(Emit::Skip),
                Control::choice("direction", "Direzione", DIRECTION_OPTS, 1).emit(Emit::Always),
                Control::text("sender_name", "Mittente", "").emit(Emit::Template("sender_name: Some(\"{v}\".to_string())")),
                Control::text("timestamp", "Ora", "14:28").emit(Emit::Template("timestamp: Some(\"{v}\".to_string())")),
                Control::choice("status", "Consegna", STATUS_OPTS, 3).emit(Emit::Always),
                Control::bool("badge", "Badge cifratura", true)
                    .emit(Emit::Template("badge: Some(rsx! { Icon { kind: IconKind::ShieldCheck, size: 11 } })")),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some("\"{text}\""),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "delivery",
                title: "Stati di consegna",
                description: "Orologio, spunta, doppia spunta ed errore.",
                code: DELIVERY_CODE,
                render: || rsx! { Delivery {} },
                span: Span::Half,
            },
            Example {
                id: "senders",
                title: "Mittenti e badge",
                description: "Messaggi ricevuti in un gruppo, uno da un bot.",
                code: SENDERS_CODE,
                render: || rsx! { Senders {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Una conversazione di supporto: scrivi e invia un messaggio.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "MessageBubble", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Container (§3.1) · semantic accent per i messaggi inviati (§3.10)",
            depth: "Flat-lit (ricevuto) · accent soft (inviato)",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-accent-soft", "--sd-accent-border", "--sd-radius-lg", "--sd-text-xs", "--sd-font-mono"],
            notes: "Nessuna bolla ha riempimento pieno: l'inviato è una tinta accento, il ricevuto una superficie; l'ora è mono subtle.",
        }),
        a11y: &[
            "Il contenuto è testo normale, letto in ordine di sorgente.",
            "Lo stato di consegna ha un title da Locale (delivery_sending, delivery_sent, …); l'icona è decorativa.",
            "La direzione non è annunciata: usa sender_name nei gruppi per dire chi scrive.",
        ],
        related: &["avatars", "attachment-cards", "activity-feed", "textarea"],
    }
}
