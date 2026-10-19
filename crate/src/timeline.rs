use dioxus::prelude::*;

/// Allineamento del contenuto rispetto alla linea temporale.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimelineAlign {
    #[default]
    Left,
    Right,
}

impl TimelineAlign {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            Self::Left => "sd-timeline--align-left",
            Self::Right => "sd-timeline--align-right",
        }
    }
}

/// Variante stilistica della linea di connessione.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimelineLineVariant {
    #[default]
    Solid,
    Dashed,
}

impl TimelineLineVariant {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            Self::Solid => "sd-timeline--line-solid",
            Self::Dashed => "sd-timeline--line-dashed",
        }
    }
}

/// Stato operativo del marcatore temporale (bullet).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimelineBulletState {
    #[default]
    Default,
    Active,
    Completed,
    Pending,
    Error,
}

impl TimelineBulletState {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            Self::Default => "sd-timeline-item--state-default",
            Self::Active => "sd-timeline-item--state-active",
            Self::Completed => "sd-timeline-item--state-completed",
            Self::Pending => "sd-timeline-item--state-pending",
            Self::Error => "sd-timeline-item--state-error",
        }
    }
}

/// Contesto condiviso della timeline fornito dal componente padre.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TimelineContext {
    pub active: usize,
    pub align: TimelineAlign,
    pub bullet_size: u32,
    pub line_width: u32,
    pub line_variant: TimelineLineVariant,
}

impl Default for TimelineContext {
    fn default() -> Self {
        Self {
            active: 0,
            align: TimelineAlign::Left,
            bullet_size: 24,
            line_width: 2,
            line_variant: TimelineLineVariant::Solid,
        }
    }
}

/// Componente Timeline: linea guida temporale per tracciamento eventi, storico transazioni o stati di consegna.
#[component]
pub fn Timeline(
    #[props(default)] active: usize,
    #[props(default)] align: TimelineAlign,
    #[props(default = 24)] bullet_size: u32,
    #[props(default = 2)] line_width: u32,
    #[props(default)] line_variant: TimelineLineVariant,
    #[props(default)] class: String,
    #[props(default)] id: Option<String>,
    children: Element,
) -> Element {
    let align_class = align.scss_class();
    let line_class = line_variant.scss_class();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("timeline").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("timeline", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let combined_class = format!(
        "sd-timeline {align_class} {line_class} {base_override} {class_modifiers} {class}"
    );

    provide_context(TimelineContext {
        active,
        align,
        bullet_size,
        line_width,
        line_variant,
    });

    rsx! {
        div {
            class: "{combined_class}",
            id: id,
            role: "list",
            {children}
        }
    }
}

/// Singolo evento o voce all'interno di una `Timeline`.
#[component]
pub fn TimelineItem(
    #[props(default)] title: Option<String>,
    #[props(default)] time: Option<String>,
    #[props(default)] bullet: Option<Element>,
    #[props(default)] state: Option<TimelineBulletState>,
    #[props(default)] line_variant: Option<TimelineLineVariant>,
    #[props(default)] class: String,
    #[props(default)] id: Option<String>,
    children: Element,
) -> Element {
    let ctx = use_context::<TimelineContext>();

    let effective_state = state.unwrap_or(TimelineBulletState::Default);
    let state_class = effective_state.scss_class();
    let effective_line = line_variant.unwrap_or(ctx.line_variant);
    let line_class = effective_line.scss_class();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("timeline_item").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("timeline_item", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let combined_class = format!(
        "sd-timeline-item {state_class} {line_class} {base_override} {class_modifiers} {class}"
    );

    let b_size = ctx.bullet_size;

    rsx! {
        div {
            class: "{combined_class}",
            id: id,
            role: "listitem",
            // Linea connettiva
            div {
                class: "sd-timeline-item__line",
                style: "left: {b_size / 2}px;",
                aria_hidden: "true",
            }
            // Bullet circolare con icona opzionale
            div {
                class: "sd-timeline-item__bullet",
                style: "width: {b_size}px; height: {b_size}px; min-width: {b_size}px;",
                aria_hidden: "true",
                if let Some(b) = bullet {
                    {b}
                } else {
                    span { class: "sd-timeline-item__dot" }
                }
            }
            // Corpo informativo (intestazione, timestamp, contenuto)
            div {
                class: "sd-timeline-item__body",
                if title.is_some() || time.is_some() {
                    div {
                        class: "sd-timeline-item__header",
                        if let Some(t) = title {
                            span { class: "sd-timeline-item__title", "{t}" }
                        }
                        if let Some(tm) = time {
                            span { class: "sd-timeline-item__time", "{tm}" }
                        }
                    }
                }
                div {
                    class: "sd-timeline-item__content",
                    {children}
                }
            }
        }
    }
}
