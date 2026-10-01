use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl AlertVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            AlertVariant::Info => {
                "bg-purple-950/40 text-purple-100 border-purple-900/60 border-l-4 border-l-purple-600 shadow-lg shadow-purple-950/30"
            }
            AlertVariant::Success => {
                "bg-emerald-950/40 text-emerald-100 border-emerald-900/60 border-l-4 border-l-emerald-500 shadow-lg shadow-black/40"
            }
            AlertVariant::Warning => {
                "bg-amber-950/40 text-amber-100 border-amber-900/60 border-l-4 border-l-amber-500 shadow-lg shadow-black/40"
            }
            AlertVariant::Danger => {
                "bg-rose-950/40 text-rose-100 border-rose-900/60 border-l-4 border-l-rose-500 shadow-lg shadow-black/40"
            }
        }
    }

    pub fn title_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "text-purple-300",
            AlertVariant::Success => "text-emerald-300",
            AlertVariant::Warning => "text-amber-300",
            AlertVariant::Danger => "text-rose-300",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            AlertVariant::Info => "ℹ️",
            AlertVariant::Success => "✅",
            AlertVariant::Warning => "⚠️",
            AlertVariant::Danger => "🚫",
        }
    }
}

#[component]
pub fn Alert(
    #[props(default)] title: Option<String>,
    #[props(default)] variant: AlertVariant,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let variant_classes = variant.classes();
    let title_class = variant.title_class();
    let icon = variant.icon();

    rsx! {
        div {
            class: "p-4 rounded-xl border flex items-start gap-3 shadow-md {variant_classes} {class}",
            span { class: "text-lg shrink-0 select-none mt-0.5", "{icon}" }
            div { class: "flex flex-col gap-1 flex-1",
                if let Some(t) = title {
                    h4 { class: "font-semibold text-sm {title_class}", "{t}" }
                }
                div { class: "text-sm opacity-90 leading-relaxed", {children} }
            }
        }
    }
}
