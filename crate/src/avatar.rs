use dioxus::prelude::*;

#[cfg(feature = "icon")]
use crate::icon::{Icon, IconKind, RenderSvgIcon, SvgIcon};

/// Dimensioni disponibili per il componente `Avatar`.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum AvatarSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl AvatarSize {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            AvatarSize::Xs => "sd-avatar--xs",
            AvatarSize::Sm => "sd-avatar--sm",
            AvatarSize::Md => "sd-avatar--md",
            AvatarSize::Lg => "sd-avatar--lg",
            AvatarSize::Xl => "sd-avatar--xl",
        }
    }

    #[must_use]
    pub fn px_size(&self) -> u32 {
        match self {
            AvatarSize::Xs => 24,
            AvatarSize::Sm => 32,
            AvatarSize::Md => 40,
            AvatarSize::Lg => 48,
            AvatarSize::Xl => 64,
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            AvatarSize::Xs => "xs",
            AvatarSize::Sm => "sm",
            AvatarSize::Md => "md",
            AvatarSize::Lg => "lg",
            AvatarSize::Xl => "xl",
        }
    }
}

/// Stato di presenza e connettività per profili e nodi mesh/P2P.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PresenceStatus {
    /// Utente attivo e connesso (dot verde pulsante).
    Online,
    /// Utente disconnesso (dot grigio neutro).
    Offline,
    /// Utente occupato o non disturbare (dot rosso/ambra).
    Busy,
    /// Connessione diretta Peer-to-Peer o tramite mesh radio (dot ciano/viola pulsante ad alta intensità).
    DirectP2P,
}

impl PresenceStatus {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            PresenceStatus::Online => "sd-avatar__status--online",
            PresenceStatus::Offline => "sd-avatar__status--offline",
            PresenceStatus::Busy => "sd-avatar__status--busy",
            PresenceStatus::DirectP2P => "sd-avatar__status--direct-p2p",
        }
    }

    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            PresenceStatus::Online => "Online",
            PresenceStatus::Offline => "Offline",
            PresenceStatus::Busy => "Occupato",
            PresenceStatus::DirectP2P => "Direct P2P / Mesh",
        }
    }
}

/// Calcola le iniziali (1 o 2 lettere) da un nome.
fn compute_initials(name: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().filter(|s| !s.is_empty()).collect();
    match words.len() {
        0 => String::new(),
        1 => {
            let mut chars = words[0].chars();
            chars.next().map(|c| c.to_uppercase().to_string()).unwrap_or_default()
        }
        _ => {
            let first = words[0].chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default();
            let last = words[words.len() - 1].chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default();
            format!("{first}{last}")
        }
    }
}

/// Genera una tonalità HSL coerente e deterministica da un nome per lo sfondo scuro.
fn deterministic_hue(name: &str) -> u32 {
    let mut hash: u32 = 5381;
    for byte in name.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    hash % 360
}

/// Componente Avatar per profili utente, rubrica e nodi mesh in Shadow Design.
#[component]
pub fn Avatar(
    #[props(default)] src: Option<String>,
    #[props(default)] name: Option<String>,
    #[props(default)] size: AvatarSize,
    #[props(default)] status: Option<PresenceStatus>,
    #[props(default)] class: String,
    #[props(default)] alt: Option<String>,
    #[cfg(feature = "icon")]
    #[props(default)] icon: Option<SvgIcon>,
) -> Element {
    let size_class = size.scss_class();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("avatar").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("avatar", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let initials = name.as_deref().map(compute_initials).unwrap_or_default();
    let alt_text = alt.or_else(|| name.clone()).unwrap_or_else(|| "Avatar".to_string());

    // Stile di sfondo dinamico per iniziali se non c'è immagine
    let bg_style = if src.is_none() && !initials.is_empty() {
        let hue = name.as_deref().map(deterministic_hue).unwrap_or(270);
        let hue2 = (hue + 40) % 360;
        format!(
            "background: linear-gradient(135deg, hsl({hue}, 55%, 18%) 0%, hsl({hue2}, 60%, 10%) 100%); border-color: hsla({hue}, 60%, 45%, 0.4);"
        )
    } else {
        String::new()
    };

    rsx! {
        div {
            class: "sd-avatar {size_class} {base_override} {class_modifiers} {class}",
            style: "{bg_style}",
            title: "{alt_text}",

            if let Some(img_url) = src {
                img {
                    src: "{img_url}",
                    alt: "{alt_text}",
                    class: "sd-avatar__img",
                }
            } else if !initials.is_empty() {
                span {
                    class: "sd-avatar__initials",
                    "{initials}"
                }
            } else {
                if let Some(custom_icon) = icon {
                    RenderSvgIcon {
                        icon: custom_icon,
                        size: size.px_size() / 2,
                        class: "sd-avatar__icon".to_string(),
                    }
                } else {
                    Icon {
                        kind: IconKind::Sparkles,
                        size: size.px_size() / 2,
                        class: "sd-avatar__icon text-zinc-400".to_string(),
                    }
                }
            }

            if let Some(pres) = status {
                span {
                    class: "sd-avatar__status {pres.scss_class()}",
                    title: "{pres.label()}",
                }
            }
        }
    }
}

/// Gruppo di avatar sovrapposti con spaziatura negativa.
#[component]
pub fn AvatarGroup(
    children: Element,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        div {
            class: "sd-avatar-group {class}",
            {children}
        }
    }
}
