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
                "bg-blue-600 hover:bg-blue-700 text-white shadow-md hover:shadow-lg border border-blue-500"
            }
            ButtonVariant::Secondary => {
                "bg-zinc-800 hover:bg-zinc-700 text-zinc-100 shadow-md border border-zinc-700"
            }
            ButtonVariant::Outline => {
                "bg-transparent hover:bg-zinc-800/70 text-zinc-200 border border-zinc-600 hover:border-zinc-400"
            }
            ButtonVariant::Ghost => {
                "bg-transparent hover:bg-zinc-800 text-zinc-300 hover:text-white"
            }
            ButtonVariant::Danger => {
                "bg-rose-600 hover:bg-rose-700 text-white shadow-md border border-rose-500"
            }
            ButtonVariant::Success => {
                "bg-emerald-600 hover:bg-emerald-700 text-white shadow-md border border-emerald-500"
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
