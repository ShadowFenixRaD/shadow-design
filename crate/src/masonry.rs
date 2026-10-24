use dioxus::prelude::*;

use crate::icon::{Icon, IconKind};

/// Configurazione responsive delle colonne per la griglia Masonry.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ResponsiveCols {
    pub base: u8,
    pub sm: u8,
    pub md: u8,
    pub lg: u8,
    pub xl: u8,
}

impl Default for ResponsiveCols {
    fn default() -> Self {
        Self {
            base: 1,
            sm: 2,
            md: 3,
            lg: 4,
            xl: 4,
        }
    }
}

impl ResponsiveCols {
    #[must_use]
    pub fn fixed(cols: u8) -> Self {
        Self {
            base: cols,
            sm: cols,
            md: cols,
            lg: cols,
            xl: cols,
        }
    }

    #[must_use]
    pub fn responsive(base: u8, sm: u8, md: u8, lg: u8, xl: u8) -> Self {
        Self { base, sm, md, lg, xl }
    }
}

/// Spaziatura tokenizzata tra gli elementi della griglia.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ComponentSpacing {
    None,
    Xs,
    #[default]
    Sm,
    Md,
    Lg,
    Xl,
}

impl ComponentSpacing {
    /// CSS value of the spacing token used for the column gap.
    #[must_use]
    pub fn token(&self) -> &'static str {
        match self {
            Self::None => "0",
            Self::Xs => "var(--sd-space-1)",
            Self::Sm => "var(--sd-space-2)",
            Self::Md => "var(--sd-space-4)",
            Self::Lg => "var(--sd-space-6)",
            Self::Xl => "var(--sd-space-8)",
        }
    }

    #[must_use]
    pub fn to_px(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Xs => 4,
            Self::Sm => 8,
            Self::Md => 16,
            Self::Lg => 24,
            Self::Xl => 32,
        }
    }
}

/// Singolo elemento multimediale per la galleria `ImageList`.
#[derive(Clone, PartialEq, Debug)]
pub struct MasonryItem {
    pub id: String,
    pub src: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub aspect_ratio: Option<f64>,
}

impl MasonryItem {
    #[must_use]
    pub fn new(id: impl Into<String>, src: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            src: src.into(),
            title: None,
            subtitle: None,
            aspect_ratio: None,
        }
    }

    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn with_subtitle(mut self, sub: impl Into<String>) -> Self {
        self.subtitle = Some(sub.into());
        self
    }

    #[must_use]
    pub fn with_aspect_ratio(mut self, ratio: f64) -> Self {
        self.aspect_ratio = Some(ratio);
        self
    }
}

impl ResponsiveCols {
    /// Inline custom properties read by `.sd-masonry` (one column count per breakpoint).
    fn style(&self, gap: ComponentSpacing) -> String {
        format!(
            "--sd-masonry-cols: {}; --sd-masonry-cols-sm: {}; --sd-masonry-cols-md: {}; --sd-masonry-cols-lg: {}; --sd-masonry-cols-xl: {}; --sd-masonry-gap: {};",
            self.base,
            self.sm,
            self.md,
            self.lg,
            self.xl,
            gap.token()
        )
    }
}

/// Waterfall grid with columns of uneven height; the column count follows the breakpoints in `cols`.
#[component]
pub fn MasonryGrid(
    #[props(default)] cols: ResponsiveCols,
    #[props(default)] gap: ComponentSpacing,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "sd-masonry {class}",
            style: "{cols.style(gap)}",
            {children}
        }
    }
}

/// Masonry image gallery with an optional modal lightbox.
#[component]
pub fn ImageList(
    items: Vec<MasonryItem>,
    #[props(default)] cols: ResponsiveCols,
    #[props(default)] gap: ComponentSpacing,
    #[props(default = true)] with_lightbox: bool,
    #[props(default)] class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let mut selected_index = use_signal(|| None::<usize>);

    let total = items.len();
    let item_class = if with_lightbox {
        "sd-masonry__item sd-masonry__item--interactive"
    } else {
        "sd-masonry__item"
    };

    rsx! {
        div { class: "sd-image-list {class}",
            div {
                class: "sd-masonry",
                style: "{cols.style(gap)}",

                for (idx, item) in items.iter().enumerate() {
                    {
                        let src = item.src.clone();
                        let title = item.title.clone();
                        let sub = item.subtitle.clone();

                        rsx! {
                            div {
                                key: "{item.id}",
                                class: "{item_class}",
                                role: if with_lightbox { Some("button") } else { None },
                                tabindex: if with_lightbox { "0" } else { "-1" },
                                onclick: move |_| {
                                    if with_lightbox {
                                        selected_index.set(Some(idx));
                                    }
                                },
                                onkeydown: move |e: KeyboardEvent| {
                                    if with_lightbox && (e.key() == Key::Enter || e.key() == Key::Character(" ".to_string())) {
                                        e.prevent_default();
                                        selected_index.set(Some(idx));
                                    }
                                },

                                img {
                                    src: "{src}",
                                    alt: "{title.clone().unwrap_or_default()}",
                                    class: "sd-masonry__image",
                                    loading: "lazy",
                                }

                                if title.is_some() || sub.is_some() {
                                    div { class: "sd-masonry__caption",
                                        if let Some(t) = &title {
                                            span { class: "sd-masonry__caption-title", "{t}" }
                                        }
                                        if let Some(s) = &sub {
                                            span { class: "sd-masonry__caption-sub", "{s}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Lightbox: modal layer over a backdrop.
            if let Some(idx) = selected_index() {
                if let Some(item) = items.get(idx) {
                    {
                        let cur_src = item.src.clone();
                        let cur_title = item.title.clone();
                        let cur_sub = item.subtitle.clone();

                        rsx! {
                            div {
                                class: "sd-masonry__lightbox",
                                role: "dialog",
                                "aria-modal": "true",
                                "aria-label": "{cur_title.clone().unwrap_or_default()}",
                                onclick: move |_| selected_index.set(None),
                                onkeydown: move |e: KeyboardEvent| {
                                    if e.key() == Key::Escape {
                                        selected_index.set(None);
                                    }
                                },

                                div {
                                    class: "sd-masonry__lightbox-bar",
                                    onclick: move |e| e.stop_propagation(),

                                    div { class: "sd-masonry__lightbox-meta",
                                        span { class: "sd-masonry__lightbox-count",
                                            "{idx + 1} / {total}"
                                        }
                                        if let Some(t) = &cur_title {
                                            span { class: "sd-masonry__lightbox-title", "{t}" }
                                        }
                                    }

                                    button {
                                        r#type: "button",
                                        class: "sd-masonry__lightbox-btn",
                                        "aria-label": "{locale.close}",
                                        onclick: move |_| selected_index.set(None),
                                        Icon { kind: IconKind::X, size: 20 }
                                    }
                                }

                                button {
                                    r#type: "button",
                                    class: "sd-masonry__lightbox-btn sd-masonry__lightbox-btn--prev",
                                    "aria-label": "{locale.previous}",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        if let Some(i) = selected_index() {
                                            if i > 0 {
                                                selected_index.set(Some(i - 1));
                                            } else if total > 0 {
                                                selected_index.set(Some(total - 1));
                                            }
                                        }
                                    },
                                    Icon { kind: IconKind::ChevronLeft, size: 20 }
                                }

                                button {
                                    r#type: "button",
                                    class: "sd-masonry__lightbox-btn sd-masonry__lightbox-btn--next",
                                    "aria-label": "{locale.next}",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        if let Some(i) = selected_index() {
                                            if i + 1 < total {
                                                selected_index.set(Some(i + 1));
                                            } else {
                                                selected_index.set(Some(0));
                                            }
                                        }
                                    },
                                    Icon { kind: IconKind::ChevronRight, size: 20 }
                                }

                                div {
                                    class: "sd-masonry__lightbox-frame",
                                    onclick: move |e| e.stop_propagation(),

                                    img {
                                        src: "{cur_src}",
                                        alt: "{cur_title.clone().unwrap_or_default()}",
                                        class: "sd-masonry__lightbox-image",
                                    }

                                    if let Some(sub_text) = cur_sub {
                                        div { class: "sd-masonry__lightbox-sub",
                                            "{sub_text}"
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
}
