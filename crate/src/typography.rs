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
    pub fn scss_class(&self) -> &'static str {
        match self {
            HeadingOrder::H1 => "sd-heading--h1",
            HeadingOrder::H2 => "sd-heading--h2",
            HeadingOrder::H3 => "sd-heading--h3",
            HeadingOrder::H4 => "sd-heading--h4",
            HeadingOrder::H5 => "sd-heading--h5",
            HeadingOrder::H6 => "sd-heading--h6",
        }
    }
}

#[component]
pub fn Heading(
    #[props(default)] order: HeadingOrder,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let order_class = order.scss_class();

    match order {
        HeadingOrder::H1 => rsx! { h1 { class: "sd-heading {order_class} {class}", {children} } },
        HeadingOrder::H2 => rsx! { h2 { class: "sd-heading {order_class} {class}", {children} } },
        HeadingOrder::H3 => rsx! { h3 { class: "sd-heading {order_class} {class}", {children} } },
        HeadingOrder::H4 => rsx! { h4 { class: "sd-heading {order_class} {class}", {children} } },
        HeadingOrder::H5 => rsx! { h5 { class: "sd-heading {order_class} {class}", {children} } },
        HeadingOrder::H6 => rsx! { h6 { class: "sd-heading {order_class} {class}", {children} } },
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
    pub fn scss_class(&self) -> &'static str {
        match self {
            TextSize::Xs => "sd-text--xs",
            TextSize::Sm => "sd-text--sm",
            TextSize::Md => "sd-text--md",
            TextSize::Lg => "sd-text--lg",
            TextSize::Xl => "sd-text--xl",
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
    let size_class = size.scss_class();
    let dimmed_class = if dimmed { "sd-text--dimmed" } else { "" };

    rsx! {
        p {
            class: "sd-text {size_class} {dimmed_class} {class}",
            {children}
        }
    }
}

#[component]
pub fn Kbd(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        kbd {
            class: "sd-kbd {class}",
            {children}
        }
    }
}

#[component]
pub fn Code(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        code {
            class: "sd-code {class}",
            {children}
        }
    }
}
