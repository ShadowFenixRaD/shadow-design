use dioxus::prelude::*;

use crate::avatar::{Avatar, AvatarSize, PersonaProps};
use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::icon::{Icon, IconKind};
use crate::textarea::Textarea;

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

    let size_class = if compact { "sd-activity-item--compact" } else { "" };
    let unread_class = if event.unread { "sd-activity-item--unread" } else { "" };

    rsx! {
        div { class: "sd-activity-item {size_class} {unread_class}",
            // Vertical connector (centred on the avatar)
            if !is_last {
                div { class: "sd-activity-item__connector" }
            }

            // Author avatar with the event icon mark (top-right, clear of the presence dot)
            div { class: "sd-activity-item__lead",
                Avatar {
                    src: author.avatar_url.clone(),
                    name: Some(author.name.clone()),
                    size: if compact { AvatarSize::Sm } else { AvatarSize::Md },
                    status: author.status,
                }
                span { class: "sd-activity-item__mark",
                    Icon { kind: icon_kind, size: 10 }
                }
            }

            // Event details
            div { class: "sd-activity-item__body",
                div { class: "sd-activity-item__head",
                    span { class: "sd-activity-item__author", "{author.name}" }
                    span { class: "sd-activity-item__action", "{action}" }
                    if let Some(t) = &target {
                        span { class: "sd-activity-item__target", "{t}" }
                    }
                    span { class: "sd-activity-item__time", "{ts}" }
                }

                if let Some(d) = &desc {
                    div { class: "sd-activity-item__desc", "{d}" }
                }

                if !event.comments.is_empty() {
                    div { class: "sd-activity-item__comments",
                        for c in &event.comments {
                            div { key: "{c.timestamp}", class: "sd-activity-item__comment",
                                Avatar {
                                    src: c.author.avatar_url.clone(),
                                    name: Some(c.author.name.clone()),
                                    size: AvatarSize::Xs,
                                }
                                div { class: "sd-activity-item__comment-body",
                                    div { class: "sd-activity-item__comment-head",
                                        span { class: "sd-activity-item__comment-author", "{c.author.name}" }
                                        span { class: "sd-activity-item__time", "{c.timestamp}" }
                                    }
                                    p { class: "sd-activity-item__comment-text", "{c.text}" }
                                }
                            }
                        }
                    }
                }

                if allow_replies {
                    div { class: "sd-activity-item__reply",
                        if !is_replying() {
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Xs,
                                onclick: move |_| is_replying.set(true),
                                "{locale.reply}"
                            }
                        } else {
                            div { class: "sd-activity-item__composer",
                                Textarea {
                                    value: reply_text,
                                    placeholder: Some(locale.activity_reply_placeholder.clone()),
                                }
                                div { class: "sd-activity-item__composer-actions",
                                    Button {
                                        variant: ButtonVariant::Ghost,
                                        size: ButtonSize::Sm,
                                        onclick: move |_| is_replying.set(false),
                                        "{locale.cancel}"
                                    }
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        size: ButtonSize::Sm,
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
        div { class: "sd-activity-feed {class}",
            if items.is_empty() {
                div { class: "sd-activity-feed__empty",
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
