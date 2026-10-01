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
            CardShadow::Sm => "shadow-sm",
            CardShadow::Md => "shadow-md hover:shadow-lg transition-shadow",
            CardShadow::Lg => "shadow-xl hover:shadow-2xl transition-shadow",
            CardShadow::Xl => "shadow-2xl",
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
    let shadow_class = shadow.classes();
    rsx! {
        div {
            class: "bg-zinc-900 border border-zinc-800 text-zinc-100 rounded-xl overflow-hidden {shadow_class} {class}",
            if title.is_some() || subtitle.is_some() {
                div {
                    class: "px-6 py-4 border-b border-zinc-800/80 bg-zinc-900/60",
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
                    class: "px-6 py-3 bg-zinc-950/40 border-t border-zinc-800/80 text-sm text-zinc-400",
                    {f}
                }
            }
        }
    }
}
