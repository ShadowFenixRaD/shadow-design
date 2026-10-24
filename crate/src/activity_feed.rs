use dioxus::prelude::*;

use crate::avatar::{Avatar, AvatarSize, PersonaProps};
use crate::icon::{Icon, IconKind};

/// Singolo commento o risposta all'interno di un evento di attività.
#[derive(Clone, PartialEq, Debug)]
pub struct ActivityComment {
    pub author: PersonaProps,
    pub text: String,
    pub timestamp: String,
}

impl ActivityComment {
    #[must_use]
    pub fn new(author: PersonaProps, text: impl Into<String>, timestamp: impl Into<String>) -> Self {
        Self {
            author,
            text: text.into(),
            timestamp: timestamp.into(),
        }
    }
}

/// Singolo evento di attività collaborativa per timeline o feed.
#[derive(Clone, PartialEq, Debug)]
pub struct ActivityEvent {
    pub id: String,
    pub author: PersonaProps,
    pub icon: Option<IconKind>,
    pub action: String,
    pub target: Option<String>,
    pub description: Option<String>,
    pub timestamp: String,
    pub comments: Vec<ActivityComment>,
    pub unread: bool,
}

impl ActivityEvent {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        author: PersonaProps,
        action: impl Into<String>,
        timestamp: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            author,
            icon: None,
            action: action.into(),
            target: None,
            description: None,
            timestamp: timestamp.into(),
            comments: Vec::new(),
            unread: false,
        }
    }

    #[must_use]
    pub fn with_icon(mut self, icon: IconKind) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    #[must_use]
    pub fn with_comments(mut self, comments: Vec<ActivityComment>) -> Self {
        self.comments = comments;
        self
    }

    #[must_use]
    pub fn with_unread(mut self, unread: bool) -> Self {
        self.unread = unread;
        self
    }
}

/// Singola riga di attività con connettore, icona evento e commenti inline.
#[component]
pub fn ActivityItem(
    event: ActivityEvent,
    #[props(default)] is_last: bool,
    #[props(default = false)] compact: bool,
    #[props(default = true)] allow_replies: bool,
    #[props(default)] on_reply: Option<EventHandler<(String, String)>>,
) -> Element {
    let locale = crate::locale::use_locale();
    let mut is_replying = use_signal(|| false);
    let mut reply_text = use_signal(String::new);

    let event_id = event.id.clone();
    let author = event.author.clone();
    let action = event.action.clone();
    let target = event.target.clone();
    let desc = event.description.clone();
    let ts = event.timestamp.clone();
    let icon_kind = event.icon.unwrap_or(IconKind::Sparkles);

    rsx! {
        div { class: "sd-activity-item relative flex items-start gap-3.5 w-full group",
            // Connettore verticale
            if !is_last {
                div {
                    class: "absolute left-4 top-10 bottom-0 w-px bg-white/[0.08]",
                }
            }

            // Avatar autore con icona sovrapposta
            div { class: "relative shrink-0 z-10",
                Avatar {
                    src: author.avatar_url.clone(),
                    name: Some(author.name.clone()),
                    size: if compact { AvatarSize::Sm } else { AvatarSize::Md },
                    status: author.status,
                }
                span { class: "absolute -bottom-1 -right-1 p-1 rounded-full bg-zinc-950 border border-white/10 text-purple-400 shadow-sm",
                    Icon { kind: icon_kind, size: 10 }
                }
            }

            // Dettagli evento
            div { class: "flex-1 flex flex-col gap-1.5 pb-6 min-w-0",
                div { class: "flex flex-wrap items-baseline gap-1.5 text-xs",
                    span { class: "font-semibold text-white", "{author.name}" }
                    span { class: "text-zinc-400", "{action}" }
                    if let Some(t) = &target {
                        span { class: "font-semibold text-purple-300 font-mono", "{t}" }
                    }
                    span { class: "text-[11px] text-zinc-500 font-mono ml-auto", "{ts}" }
                }

                if let Some(d) = &desc {
                    div { class: "p-3 rounded-xl bg-white/[0.03] border border-white/[0.06] text-xs text-zinc-300 leading-relaxed font-sans mt-0.5",
                        "{d}"
                    }
                }

                // Commenti o risposte
                if !event.comments.is_empty() {
                    div { class: "flex flex-col gap-2 mt-2 pl-3 border-l-2 border-white/[0.06]",
                        for c in &event.comments {
                            div { key: "{c.timestamp}", class: "flex items-start gap-2 text-xs",
                                Avatar {
                                    src: c.author.avatar_url.clone(),
                                    name: Some(c.author.name.clone()),
                                    size: AvatarSize::Xs,
                                }
                                div { class: "flex flex-col flex-1 bg-white/[0.02] p-2 rounded-lg border border-white/[0.04]",
                                    div { class: "flex items-center justify-between text-[11px]",
                                        span { class: "font-semibold text-zinc-200", "{c.author.name}" }
                                        span { class: "text-zinc-500 font-mono", "{c.timestamp}" }
                                    }
                                    p { class: "text-zinc-300 mt-1", "{c.text}" }
                                }
                            }
                        }
                    }
                }

                // Azione Rispondi
                if allow_replies {
                    div { class: "flex items-center gap-2 mt-1",
                        if !is_replying() {
                            button {
                                r#type: "button",
                                class: "text-[11px] font-mono text-zinc-500 hover:text-purple-300 transition-colors cursor-pointer",
                                onclick: move |_| is_replying.set(true),
                                "{locale.reply}"
                            }
                        } else {
                            div { class: "w-full flex flex-col gap-2 p-2.5 rounded-xl bg-zinc-900 border border-white/10 mt-1",
                                textarea {
                                    class: "w-full p-2 bg-transparent text-xs text-zinc-200 placeholder-zinc-500 outline-none resize-none min-h-[60px]",
                                    placeholder: "{locale.activity_reply_placeholder}",
                                    value: "{reply_text}",
                                    oninput: move |e| reply_text.set(e.value()),
                                }
                                div { class: "flex items-center justify-end gap-2",
                                    button {
                                        r#type: "button",
                                        class: "px-2.5 py-1 rounded text-xs text-zinc-400 hover:text-white transition-colors cursor-pointer",
                                        onclick: move |_| is_replying.set(false),
                                        "{locale.cancel}"
                                    }
                                    button {
                                        r#type: "button",
                                        class: "px-3 py-1 rounded bg-purple-600 hover:bg-purple-500 text-xs font-semibold text-white transition-colors cursor-pointer disabled:opacity-40",
                                        disabled: reply_text().trim().is_empty(),
                                        onclick: {
                                            let eid = event_id.clone();
                                            move |_| {
                                                let t = reply_text().trim().to_string();
                                                if !t.is_empty() {
                                                    if let Some(h) = &on_reply {
                                                        h.call((eid.clone(), t));
                                                    }
                                                    reply_text.set(String::new());
                                                    is_replying.set(false);
                                                }
                                            }
                                        },
                                        "{locale.send}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Feed completo di attività collaborative per audit log, timeline notifiche e workspace feed.
#[component]
pub fn ActivityFeed(
    items: Vec<ActivityEvent>,
    #[props(default = false)] compact: bool,
    #[props(default = true)] allow_replies: bool,
    #[props(default)] on_reply: Option<EventHandler<(String, String)>>,
    #[props(default)] class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let total = items.len();

    rsx! {
        div { class: "sd-activity-feed flex flex-col w-full {class}",
            if items.is_empty() {
                div { class: "p-8 text-center text-xs text-zinc-500 font-mono",
                    "{locale.activity_empty}"
                }
            } else {
                for (idx, ev) in items.into_iter().enumerate() {
                    ActivityItem {
                        key: "{ev.id}",
                        event: ev,
                        is_last: idx + 1 == total,
                        compact: compact,
                        allow_replies: allow_replies,
                        on_reply: on_reply,
                    }
                }
            }
        }
    }
}
