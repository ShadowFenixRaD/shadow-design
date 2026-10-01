use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CardShadow {
    None,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl CardShadow {
    pub fn classes(&self) -> &'static str {
        match self {
            CardShadow::None => "shadow-none",
            CardShadow::Sm => "shadow-sm shadow-black",
            CardShadow::Md => "shadow-lg shadow-black/80 hover:shadow-purple-950/40 transition-shadow",
            CardShadow::Lg => "shadow-xl shadow-black/90 hover:shadow-purple-950/60 transition-shadow",
            CardShadow::Xl => "shadow-2xl shadow-purple-950/50 border-purple-800/40",
        }
    }
}

#[component]
pub fn Card(
    #[props(default)] title: Option<String>,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] shadow: CardShadow,
    #[props(default)] class: String,
    #[props(default)] footer: Option<Element>,
    children: Element,
) -> Element {
    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let shadow_class = theme().card_shadow_classes(shadow).to_string();
    #[cfg(feature = "theme")]
    let base_class = theme().card_base.clone();
    #[cfg(feature = "theme")]
    let header_class = theme().card_header.clone();
    #[cfg(feature = "theme")]
    let footer_class = theme().card_footer.clone();

    #[cfg(not(feature = "theme"))]
    let shadow_class = shadow.classes();
    #[cfg(not(feature = "theme"))]
    let base_class = "bg-[#08080a] border border-purple-950/60 text-zinc-100 rounded-xl overflow-hidden hover:border-purple-900/50 transition-all";
    #[cfg(not(feature = "theme"))]
    let header_class = "px-6 py-4 border-b border-purple-950/50 bg-[#0d0d12]/90";
    #[cfg(not(feature = "theme"))]
    let footer_class = "px-6 py-3 bg-[#050507]/90 border-t border-purple-950/50 text-sm text-zinc-400";

    rsx! {
        div {
            class: "{base_class} {shadow_class} {class}",
            if title.is_some() || subtitle.is_some() {
                div {
                    class: "{header_class}",
                    if let Some(t) = title {
                        h3 { class: "text-lg font-semibold text-white", "{t}" }
                    }
                    if let Some(s) = subtitle {
                        p { class: "text-sm text-zinc-400 mt-1", "{s}" }
                    }
                }
            }
            div {
                class: "p-6",
                {children}
            }
            if let Some(f) = footer {
                div {
                    class: "{footer_class}",
                    {f}
                }
            }
        }
    }
}
