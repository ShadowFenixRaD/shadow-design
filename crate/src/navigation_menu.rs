use dioxus::prelude::*;

use crate::icon::{Icon, IconKind};

/// Link o voce di navigazione all'interno di un pannello Mega-Menu.
#[derive(Clone, PartialEq, Debug)]
pub struct NavigationLinkItem {
    pub title: String,
    pub description: Option<String>,
    pub href: Option<String>,
    pub icon: Option<IconKind>,
    pub badge: Option<String>,
}

impl NavigationLinkItem {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            href: None,
            icon: None,
            badge: None,
        }
    }

    #[must_use]
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    #[must_use]
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    #[must_use]
    pub fn with_icon(mut self, icon: IconKind) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }
}

/// Sezione o colonna di link per il pannello MegaMenu.
#[derive(Clone, PartialEq, Debug)]
pub struct NavigationMenuSection {
    pub heading: Option<String>,
    pub links: Vec<NavigationLinkItem>,
}

/// Scheda in evidenza opzionale all'interno del MegaMenu.
#[derive(Clone, PartialEq, Debug)]
pub struct NavigationPreviewCard {
    pub title: String,
    pub description: String,
    pub cta_label: Option<String>,
    pub image_url: Option<String>,
    pub on_cta_click: Option<EventHandler<()>>,
}

/// Singola voce di navigazione a livello principale con Mega-Menu a discesa.
#[derive(Clone, PartialEq, Debug)]
pub struct NavigationMenuItem {
    pub label: String,
    pub sections: Vec<NavigationMenuSection>,
    pub preview: Option<NavigationPreviewCard>,
}

impl NavigationMenuItem {
    #[must_use]
    pub fn new(label: impl Into<String>, sections: Vec<NavigationMenuSection>) -> Self {
        Self {
            label: label.into(),
            sections,
            preview: None,
        }
    }

    #[must_use]
    pub fn with_preview(mut self, preview: NavigationPreviewCard) -> Self {
        self.preview = Some(preview);
        self
    }
}

/// Barra di navigazione avanzata Mega-Menu per applicazioni complesse in stile shadcn/ui.
#[component]
pub fn NavigationMenu(
    items: Vec<NavigationMenuItem>,
    #[props(default)] on_select: Option<EventHandler<NavigationLinkItem>>,
    #[props(default)] class: String,
) -> Element {
    let mut active_index = use_signal(|| None::<usize>);

    let close_menu = move |_| active_index.set(None);

    rsx! {
        div { class: "sd-nav-menu relative w-full {class}",
            // Backdrop di chiusura quando un menu è aperto
            if active_index().is_some() {
                div {
                    class: "fixed inset-0 z-[985] bg-black/40 backdrop-blur-[2px] transition-opacity",
                    onclick: close_menu,
                }
            }

            // Barra dei trigger orizzontale
            nav { class: "relative z-[989] flex items-center gap-1 p-1 rounded-xl bg-zinc-950/80 border border-white/[0.08] backdrop-blur-md shadow-lg",
                for (idx, item) in items.iter().enumerate() {
                    {
                        let is_active = active_index() == Some(idx);
                        let lbl = item.label.clone();
                        let trigger_cls = if is_active {
                            "bg-white/[0.08] text-white shadow-inner border border-white/10"
                        } else {
                            "text-zinc-300 hover:text-white hover:bg-white/[0.04] border border-transparent"
                        };
                        let chevron_cls = if is_active {
                            "rotate-180 text-purple-400"
                        } else {
                            "text-zinc-400"
                        };

                        rsx! {
                            button {
                                key: "nav-trigger-{idx}",
                                r#type: "button",
                                class: "flex items-center gap-1.5 px-3.5 py-2 rounded-lg text-sm font-medium transition-all cursor-pointer {trigger_cls}",
                                onclick: move |_| {
                                    if is_active {
                                        active_index.set(None);
                                    } else {
                                        active_index.set(Some(idx));
                                    }
                                },
                                span { "{lbl}" }
                                Icon {
                                    kind: IconKind::ChevronDown,
                                    size: 14,
                                    class: "transition-transform duration-200 {chevron_cls}",
                                }
                            }
                        }
                    }
                }
            }

            // Pannello Mega-Menu fluttuante (viewport)
            if let Some(idx) = active_index() {
                if let Some(item) = items.get(idx) {
                    div {
                        class: "sd-nav-menu__viewport absolute left-0 top-full mt-2 z-[990] w-full max-w-4xl p-6 rounded-2xl border border-white/[0.14] bg-zinc-950/95 shadow-2xl backdrop-blur-2xl animate-scale-in flex flex-col md:flex-row gap-6",
                        onclick: move |e| e.stop_propagation(),

                            // Colonne sezioni link
                            div { class: "flex-1 grid grid-cols-1 sm:grid-cols-2 gap-6",
                                for (sec_idx, sec) in item.sections.iter().enumerate() {
                                    div { key: "sec-{sec_idx}", class: "flex flex-col gap-2",
                                        if let Some(h) = &sec.heading {
                                            span { class: "text-xs font-mono font-semibold uppercase tracking-wider text-purple-400/90 pb-1 border-b border-white/[0.06]",
                                                "{h}"
                                            }
                                        }

                                        div { class: "flex flex-col gap-1",
                                            for link in &sec.links {
                                                {
                                                    let l_item = link.clone();
                                                    let icon_opt = link.icon;
                                                    let title = link.title.clone();
                                                    let desc_opt = link.description.clone();
                                                    let badge_opt = link.badge.clone();

                                                    rsx! {
                                                        button {
                                                            key: "{title}",
                                                            r#type: "button",
                                                            class: "flex items-start gap-3 p-2.5 rounded-xl hover:bg-white/[0.05] transition-colors text-left group cursor-pointer border border-transparent hover:border-white/[0.08]",
                                                            onclick: {
                                                                let item_clone = l_item.clone();
                                                                move |_| {
                                                                    if let Some(handler) = &on_select {
                                                                        handler.call(item_clone.clone());
                                                                    }
                                                                    active_index.set(None);
                                                                }
                                                            },

                                                            if let Some(ik) = icon_opt {
                                                                span { class: "p-2 rounded-lg bg-white/[0.04] border border-white/[0.06] text-zinc-400 group-hover:text-purple-300 group-hover:bg-purple-950/40 transition-colors shrink-0",
                                                                    Icon { kind: ik, size: 16 }
                                                                }
                                                            }

                                                            div { class: "flex flex-col min-w-0 flex-1",
                                                                div { class: "flex items-center gap-2",
                                                                    span { class: "text-sm font-semibold text-zinc-200 group-hover:text-white transition-colors truncate",
                                                                        "{title}"
                                                                    }
                                                                    if let Some(b) = &badge_opt {
                                                                        span { class: "px-1.5 py-0.5 rounded text-[10px] font-mono bg-purple-500/20 text-purple-300 border border-purple-500/30",
                                                                            "{b}"
                                                                        }
                                                                    }
                                                                }
                                                                if let Some(d) = &desc_opt {
                                                                    span { class: "text-xs text-zinc-400 line-clamp-1",
                                                                        "{d}"
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

                            // Scheda di anteprima opzionale
                            if let Some(prev) = &item.preview {
                                div { class: "w-full md:w-64 p-4 rounded-xl border border-white/[0.08] bg-gradient-to-br from-purple-950/40 via-zinc-900 to-zinc-950 flex flex-col justify-between shrink-0",
                                    if let Some(img_url) = &prev.image_url {
                                        div { class: "w-full h-28 rounded-lg overflow-hidden mb-3 border border-white/10",
                                            img { src: "{img_url}", class: "w-full h-full object-cover", alt: "{prev.title}" }
                                        }
                                    }

                                    div { class: "flex flex-col gap-1.5",
                                        span { class: "text-xs font-mono text-purple-400 font-bold", "In evidenza" }
                                        h4 { class: "text-sm font-bold text-white", "{prev.title}" }
                                        p { class: "text-xs text-zinc-400 leading-relaxed", "{prev.description}" }
                                    }

                                    if let Some(cta) = &prev.cta_label {
                                        button {
                                            r#type: "button",
                                            class: "mt-4 w-full py-2 px-3 rounded-lg bg-purple-600 hover:bg-purple-500 text-white text-xs font-semibold shadow-md transition-colors cursor-pointer",
                                            onclick: {
                                                let cta_handler = prev.on_cta_click;
                                                move |_| {
                                                    if let Some(h) = &cta_handler {
                                                        h.call(());
                                                    }
                                                    active_index.set(None);
                                                }
                                            },
                                            "{cta}"
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
