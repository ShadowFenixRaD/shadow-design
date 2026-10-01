use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
    Success,
}

impl ButtonVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => {
                "bg-purple-800 hover:bg-purple-700 active:bg-purple-900 text-white shadow-lg shadow-purple-950/70 border border-purple-600/70 hover:border-purple-400 hover:shadow-purple-900/60"
            }
            ButtonVariant::Secondary => {
                "bg-zinc-950 hover:bg-zinc-900 text-zinc-200 shadow-md border border-purple-950/80 hover:border-purple-800/60 hover:text-white"
            }
            ButtonVariant::Outline => {
                "bg-transparent hover:bg-purple-950/30 text-purple-300 border border-purple-800/60 hover:border-purple-500 hover:text-purple-100"
            }
            ButtonVariant::Ghost => {
                "bg-transparent hover:bg-purple-950/40 text-zinc-300 hover:text-purple-200"
            }
            ButtonVariant::Danger => {
                "bg-rose-700 hover:bg-rose-600 text-white shadow-md shadow-rose-950/50 border border-rose-600"
            }
            ButtonVariant::Success => {
                "bg-emerald-700 hover:bg-emerald-600 text-white shadow-md shadow-emerald-950/50 border border-emerald-600"
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ButtonSize {
    pub fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Sm => "px-2.5 py-1 text-xs rounded",
            ButtonSize::Md => "px-4 py-2 text-sm rounded-lg",
            ButtonSize::Lg => "px-6 py-3 text-base rounded-xl font-semibold",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let base = "inline-flex items-center justify-center font-medium transition-all duration-200 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none select-none";
    let variant_classes = variant.classes();
    let size_classes = size.classes();

    rsx! {
        button {
            class: "{base} {variant_classes} {size_classes} {class}",
            disabled,
            onclick: move |evt| {
                if !disabled {
                    if let Some(handler) = onclick.as_ref() {
                        handler.call(evt);
                    }
                }
            },
            {children}
        }
    }
}
