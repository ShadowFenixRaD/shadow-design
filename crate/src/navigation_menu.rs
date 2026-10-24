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

/// Mega-menu navigation bar: a row of triggers, each opening a floating panel of link sections.
#[component]
pub fn NavigationMenu(
    items: Vec<NavigationMenuItem>,
    #[props(default)] on_select: Option<EventHandler<NavigationLinkItem>>,
    #[props(default)] class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let mut active_index = use_signal(|| None::<usize>);

    let close_menu = move |_| active_index.set(None);

    rsx! {
        div {
            class: "sd-nav-menu {class}",
            onkeydown: move |e: KeyboardEvent| {
                if e.key() == Key::Escape && active_index().is_some() {
                    e.prevent_default();
                    active_index.set(None);
                }
            },
            // Backdrop while a panel is open
            if active_index().is_some() {
                div {
                    class: "sd-nav-menu__backdrop",
                    onclick: close_menu,
                }
            }

            // Trigger bar
            nav { class: "sd-nav-menu__bar",
                for (idx, item) in items.iter().enumerate() {
                    {
                        let is_active = active_index() == Some(idx);
                        let lbl = item.label.clone();
                        let trigger_cls = if is_active { "sd-nav-menu__trigger sd-nav-menu__trigger--active" } else { "sd-nav-menu__trigger" };
                        let chevron_cls = if is_active { "sd-nav-menu__chevron sd-nav-menu__chevron--open" } else { "sd-nav-menu__chevron" };

                        rsx! {
                            button {
                                key: "nav-trigger-{idx}",
                                r#type: "button",
                                class: "{trigger_cls}",
                                "aria-haspopup": "true",
                                "aria-expanded": "{is_active}",
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
                                    class: "{chevron_cls}",
                                }
                            }
                        }
                    }
                }
            }

            // Floating panel
            if let Some(idx) = active_index() {
                if let Some(item) = items.get(idx) {
                    div {
                        class: "sd-nav-menu__viewport",
                        onclick: move |e| e.stop_propagation(),

                        div { class: "sd-nav-menu__sections",
                            for (sec_idx, sec) in item.sections.iter().enumerate() {
                                div { key: "sec-{sec_idx}", class: "sd-nav-menu__section",
                                    if let Some(h) = &sec.heading {
                                        span { class: "sd-nav-menu__heading", "{h}" }
                                    }

                                    div { class: "sd-nav-menu__links",
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
                                                        class: "sd-nav-menu__link",
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
                                                            span { class: "sd-nav-menu__link-icon",
                                                                Icon { kind: ik, size: 16 }
                                                            }
                                                        }

                                                        span { class: "sd-nav-menu__link-body",
                                                            span { class: "sd-nav-menu__link-head",
                                                                span { class: "sd-nav-menu__link-title", "{title}" }
                                                                if let Some(b) = &badge_opt {
                                                                    span { class: "sd-nav-menu__link-badge", "{b}" }
                                                                }
                                                            }
                                                            if let Some(d) = &desc_opt {
                                                                span { class: "sd-nav-menu__link-desc", "{d}" }
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

                        if let Some(prev) = &item.preview {
                            div { class: "sd-nav-menu__preview",
                                if let Some(img_url) = &prev.image_url {
                                    div { class: "sd-nav-menu__preview-image",
                                        img { src: "{img_url}", alt: "{prev.title}" }
                                    }
                                }

                                div { class: "sd-nav-menu__preview-text",
                                    span { class: "sd-nav-menu__preview-label", "{locale.navigation_featured}" }
                                    h4 { class: "sd-nav-menu__preview-title", "{prev.title}" }
                                    p { class: "sd-nav-menu__preview-desc", "{prev.description}" }
                                }

                                if let Some(cta) = &prev.cta_label {
                                    button {
                                        r#type: "button",
                                        class: "sd-nav-menu__preview-cta",
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
