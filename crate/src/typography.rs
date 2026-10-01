use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum HeadingOrder {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingOrder {
    pub fn classes(&self) -> &'static str {
        match self {
            HeadingOrder::H1 => "text-3xl sm:text-4xl font-extrabold tracking-tight text-white",
            HeadingOrder::H2 => "text-2xl sm:text-3xl font-bold tracking-tight text-white",
            HeadingOrder::H3 => "text-xl sm:text-2xl font-bold tracking-tight text-white",
            HeadingOrder::H4 => "text-lg font-semibold tracking-tight text-white",
            HeadingOrder::H5 => "text-base font-semibold text-zinc-100",
            HeadingOrder::H6 => "text-sm font-semibold text-zinc-200",
        }
    }
}

#[component]
pub fn Heading(
    #[props(default)] order: HeadingOrder,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let order_classes = order.classes();

    match order {
        HeadingOrder::H1 => rsx! { h1 { class: "{order_classes} {class}", {children} } },
        HeadingOrder::H2 => rsx! { h2 { class: "{order_classes} {class}", {children} } },
        HeadingOrder::H3 => rsx! { h3 { class: "{order_classes} {class}", {children} } },
        HeadingOrder::H4 => rsx! { h4 { class: "{order_classes} {class}", {children} } },
        HeadingOrder::H5 => rsx! { h5 { class: "{order_classes} {class}", {children} } },
        HeadingOrder::H6 => rsx! { h6 { class: "{order_classes} {class}", {children} } },
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum TextSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl TextSize {
    pub fn classes(&self) -> &'static str {
        match self {
            TextSize::Xs => "text-xs",
            TextSize::Sm => "text-sm",
            TextSize::Md => "text-base",
            TextSize::Lg => "text-lg",
            TextSize::Xl => "text-xl",
        }
    }
}

#[component]
pub fn Text(
    #[props(default)] size: TextSize,
    #[props(default)] dimmed: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let size_class = size.classes();
    let color_class = if dimmed {
        "text-zinc-400"
    } else {
        "text-zinc-200"
    };

    rsx! {
        p {
            class: "leading-relaxed {size_class} {color_class} {class}",
            {children}
        }
    }
}

#[component]
pub fn Kbd(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        kbd {
            class: "inline-flex items-center px-1.5 py-0.5 text-[10px] font-mono font-medium rounded bg-[#121218] text-zinc-300 border border-white/10 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.1)] select-none {class}",
            {children}
        }
    }
}

#[component]
pub fn Code(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        code {
            class: "inline-block px-1.5 py-0.5 text-xs font-mono rounded bg-[#0a0a0f] text-purple-300 border border-white/10 {class}",
            {children}
        }
    }
}
