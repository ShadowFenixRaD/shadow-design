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

/// Griglia a cascata in stile Pinterest con altezze eterogenee distribuite fluidamente.
#[component]
pub fn MasonryGrid(
    #[props(default)] cols: ResponsiveCols,
    #[props(default)] gap: ComponentSpacing,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let gap_px = gap.to_px();

    rsx! {
        div {
            class: "sd-masonry w-full {class}",
            style: "column-count: {cols.base}; column-gap: {gap_px}px;",
            {children}
        }
    }
}

/// Galleria di immagini Masonry con anteprima ad alto contrasto e Lightbox modale interattivo.
#[component]
pub fn ImageList(
    items: Vec<MasonryItem>,
    #[props(default)] cols: ResponsiveCols,
    #[props(default)] gap: ComponentSpacing,
    #[props(default = true)] with_lightbox: bool,
    #[props(default)] class: String,
) -> Element {
    let mut selected_index = use_signal(|| None::<usize>);

    let total = items.len();
    let gap_px = gap.to_px();

    rsx! {
        div { class: "w-full relative {class}",
            div {
                class: "sd-masonry w-full",
                style: "column-count: {cols.base}; column-gap: {gap_px}px;",

                for (idx, item) in items.iter().enumerate() {
                    {
                        let src = item.src.clone();
                        let title = item.title.clone();
                        let sub = item.subtitle.clone();

                        rsx! {
                            div {
                                key: "{item.id}",
                                class: "break-inside-avoid mb-[{gap_px}px] group relative rounded-xl overflow-hidden border border-white/[0.08] bg-zinc-900 cursor-pointer shadow-md hover:border-purple-500/40 transition-all",
                                onclick: move |_| {
                                    if with_lightbox {
                                        selected_index.set(Some(idx));
                                    }
                                },

                                img {
                                    src: "{src}",
                                    alt: "{title.clone().unwrap_or_default()}",
                                    class: "w-full h-auto object-cover group-hover:scale-105 transition-transform duration-300 block",
                                    loading: "lazy",
                                }

                                if title.is_some() || sub.is_some() {
                                    div { class: "absolute inset-x-0 bottom-0 p-3 bg-gradient-to-t from-black/90 via-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity flex flex-col justify-end",
                                        if let Some(t) = &title {
                                            span { class: "text-sm font-semibold text-white truncate", "{t}" }
                                        }
                                        if let Some(s) = &sub {
                                            span { class: "text-xs text-zinc-300 truncate", "{s}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Lightbox Modal
            if let Some(idx) = selected_index() {
                if let Some(item) = items.get(idx) {
                    {
                        let cur_src = item.src.clone();
                        let cur_title = item.title.clone();
                        let cur_sub = item.subtitle.clone();

                        rsx! {
                            div {
                                class: "fixed inset-0 z-[1000] bg-black/95 backdrop-blur-xl flex flex-col items-center justify-center p-4 select-none",
                                onclick: move |_| selected_index.set(None),

                                // Barra superiore
                                div {
                                    class: "absolute top-4 inset-x-4 flex items-center justify-between text-zinc-300 z-10",
                                    onclick: move |e| e.stop_propagation(),

                                    div { class: "flex items-center gap-2",
                                        span { class: "font-mono text-xs px-2.5 py-1 rounded bg-white/10",
                                            "{idx + 1} / {total}"
                                        }
                                        if let Some(t) = &cur_title {
                                            span { class: "text-sm font-semibold text-white", "{t}" }
                                        }
                                    }

                                    button {
                                        r#type: "button",
                                        class: "p-2 rounded-lg bg-white/10 hover:bg-white/20 text-white transition-colors cursor-pointer",
                                        onclick: move |_| selected_index.set(None),
                                        Icon { kind: IconKind::X, size: 20 }
                                    }
                                }

                                // Pulsanti navigazione
                                button {
                                    r#type: "button",
                                    class: "absolute left-4 top-1/2 -translate-y-1/2 p-3 rounded-full bg-white/10 hover:bg-purple-600/80 text-white transition-all cursor-pointer z-10",
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
                                    Icon { kind: IconKind::ChevronLeft, size: 24 }
                                }

                                button {
                                    r#type: "button",
                                    class: "absolute right-4 top-1/2 -translate-y-1/2 p-3 rounded-full bg-white/10 hover:bg-purple-600/80 text-white transition-all cursor-pointer z-10",
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
                                    Icon { kind: IconKind::ChevronRight, size: 24 }
                                }

                                // Immagine principale
                                div {
                                    class: "max-w-5xl max-h-[80vh] flex flex-col items-center justify-center overflow-hidden rounded-xl border border-white/10 shadow-2xl",
                                    onclick: move |e| e.stop_propagation(),

                                    img {
                                        src: "{cur_src}",
                                        alt: "{cur_title.clone().unwrap_or_default()}",
                                        class: "max-w-full max-h-[75vh] object-contain",
                                    }

                                    if let Some(sub_text) = cur_sub {
                                        div { class: "w-full p-3 bg-zinc-950 text-center text-xs text-zinc-300 font-mono border-t border-white/10",
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
