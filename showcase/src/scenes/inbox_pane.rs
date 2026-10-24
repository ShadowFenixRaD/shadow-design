//! Scene: the operators' inbox (REVAMP §1.3). Tabs, a message list with presence and unread
//! indicators, a context menu, a filter dropdown, a loading row and pagination.
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[derive(Clone, Copy, PartialEq)]
struct Message {
    from: &'static str,
    subject: &'static str,
    preview: &'static str,
    time: &'static str,
    tag: Option<(&'static str, BadgeVariant)>,
    unread: bool,
    mention: bool,
    status: PresenceStatus,
}

const MESSAGES: [Message; 4] = [
    Message {
        from: "Giulia Ferri",
        subject: "Rollout edge-22",
        preview: "Il canary è verde su Milano, procedo con Francoforte alle 14:00?",
        time: "09:42",
        tag: Some(("Deploy", BadgeVariant::Primary)),
        unread: true,
        mention: true,
        status: PresenceStatus::Online,
    },
    Message {
        from: "Marco Bellini",
        subject: "Latenza relay-03",
        preview: "Picco a 3.8 ms dalle 08:10, sembra la tratta verso Parigi.",
        time: "08:57",
        tag: Some(("Incidente", BadgeVariant::Warning)),
        unread: true,
        mention: false,
        status: PresenceStatus::Busy,
    },
    Message {
        from: "Sara Conti",
        subject: "Revisione chiavi",
        preview: "Ho ruotato le chiavi Kyber dei nodi core, nessun impatto sui peer.",
        time: "Ieri",
        tag: None,
        unread: false,
        mention: true,
        status: PresenceStatus::Offline,
    },
    Message {
        from: "Luca Moretti",
        subject: "Commit a91f3c2",
        preview: "Merge della nuova pipeline di telemetria, aspetto il tuo ok.",
        time: "Ieri",
        tag: Some(("Review", BadgeVariant::Default)),
        unread: false,
        mention: false,
        status: PresenceStatus::Online,
    },
];

#[component]
fn MessageRow(message: Message) -> Element {
    let m = message;
    let avatar = rsx! {
        Avatar { name: Some(m.from.to_string()), size: AvatarSize::Sm, status: Some(m.status) }
    };
    rsx! {
        ListItem {
            icon: Some(rsx! {
                if m.unread {
                    Indicator { size: IndicatorSize::Sm, label: None, {avatar} }
                } else {
                    {avatar}
                }
            }),
            div { class: "sc-inbox__row",
                div { class: "sc-inbox__line",
                    span { class: if m.unread { "sc-inbox__from sc-inbox__from--unread" } else { "sc-inbox__from" }, "{m.from}" }
                    span { class: "sc-hint", "{m.time}" }
                }
                div { class: "sc-inbox__line",
                    span { class: "sc-inbox__subject", "{m.subject}" }
                    if let Some((tag, variant)) = m.tag {
                        Badge { label: tag.to_string(), variant, pill: true }
                    }
                }
                p { class: "sc-inbox__preview", "{m.preview}" }
            }
        }
    }
}

#[component]
pub fn InboxPane(#[props(default)] compact: bool) -> Element {
    let tab = use_signal(|| "all".to_string());
    let mut filter_open = use_signal(|| false);
    let page = use_signal(|| 1usize);
    let mut toast = use_notification();
    let shown: Vec<Message> = MESSAGES
        .iter()
        .copied()
        .filter(|m| match tab().as_str() {
            "unread" => m.unread,
            "mentions" => m.mention,
            _ => true,
        })
        .collect();
    let unread = MESSAGES.iter().filter(|m| m.unread).count().to_string();

    rsx! {
        div { class: "sc-scene-body",
            div { class: "sc-scene__head",
                div { class: "sc-scene__title-group",
                    span { class: "sc-eyebrow", "Inbox operatori" }
                    span { class: "sc-scene__title", "Messaggi" }
                }
                Dropdown {
                    is_open: filter_open,
                    placement: DropdownPlacement::BottomRight,
                    trigger: rsx! {
                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            right_icon: Some(rsx! {
                                Icon { kind: IconKind::ChevronDown, size: 14 }
                            }),
                            "Filtra"
                        }
                    },
                    DropdownItem {
                        icon: Some(IconKind::Clock),
                        onclick: move |_| filter_open.set(false),
                        "Più recenti"
                    }
                    DropdownItem {
                        icon: Some(IconKind::AlertTriangle),
                        onclick: move |_| filter_open.set(false),
                        "Solo incidenti"
                    }
                    DropdownDivider {}
                    DropdownItem {
                        icon: Some(IconKind::Trash2),
                        danger: true,
                        onclick: move |_| {
                            filter_open.set(false);
                            toast.warning("Messaggi letti archiviati");
                        },
                        "Archivia i letti"
                    }
                }
            }
            Tabs {
                TabList { aria_label: Some("Cartelle".to_string()),
                    Tab { value: "all", active: tab, "Tutti" }
                    Tab { value: "unread", active: tab, badge: Some(unread), "Non letti" }
                    Tab { value: "mentions", active: tab, "Menzioni" }
                }
            }
            ContextMenu {
                ContextMenuTrigger {
                    div { class: "sc-inbox",
                        List { spacing: ListSpacing::Sm,
                            for m in shown {
                                MessageRow { key: "{m.subject}", message: m }
                            }
                            if !compact {
                                ListItem {
                                    icon: Some(rsx! {
                                        Skeleton { shape: SkeletonShape::Circle, width: Some("32px".to_string()), height: Some("32px".to_string()) }
                                    }),
                                    div { class: "sc-inbox__row",
                                        Skeleton { shape: SkeletonShape::Text, width: Some("40%".to_string()) }
                                        Skeleton { shape: SkeletonShape::Text, width: Some("75%".to_string()) }
                                    }
                                }
                            }
                        }
                    }
                }
                ContextMenuContent {
                    ContextMenuItem { label: "Segna come letto".to_string(), icon: Some(IconKind::CheckCheck), shortcut: Some("R".to_string()) }
                    ContextMenuItem { label: "Rispondi".to_string(), icon: Some(IconKind::Send), shortcut: Some("Invio".to_string()) }
                    ContextMenuSeparator {}
                    ContextMenuItem {
                        label: "Elimina".to_string(),
                        icon: Some(IconKind::Trash2),
                        danger: true,
                        onclick: move |_| {
                            toast.danger("Messaggio eliminato");
                        },
                    }
                }
            }
            div { class: "sc-scene__foot",
                span { class: "sc-hint", "Tasto destro su un messaggio per le azioni rapide" }
                Pagination { page, total: 4, size: ControlSize::Sm }
            }
        }
    }
}
