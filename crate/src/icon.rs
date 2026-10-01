use dioxus::prelude::*;

/// Tipo di icona vettoriale geometrica nitida a 1.5px stroke (zero dipendenze).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IconKind {
    ChevronRight,
    ChevronDown,
    ChevronLeft,
    ChevronUp,
    Check,
    Copy,
    X,
    AlertTriangle,
    AlertCircle,
    Info,
    CheckCircle,
    XCircle,
    Search,
    Sparkles,
    Settings,
    Code,
    Layers,
    Terminal,
    Palette,
    Eye,
    EyeOff,
    ExternalLink,
    Sun,
    Moon,
    Box,
    Menu,
    Spinner,
}

/// Rappresentazione unificata e fortemente tipizzata per icone SVG in Shadow Design.
/// Consente di passare sia icone geometriche native (`IconKind`) sia elementi SVG personalizzati.
#[derive(Clone, PartialEq, Debug)]
pub enum SvgIcon {
    /// Icona predefinita dal set geometrico nativo ad alta precisione (1.5px stroke).
    Kind(IconKind),
    /// Elemento SVG o markup personalizzato Dioxus.
    Custom(Element),
}

impl SvgIcon {
    /// Crea un'icona dal set nativo `IconKind`.
    pub fn kind(kind: IconKind) -> Self {
        Self::Kind(kind)
    }

    /// Crea un'icona da un elemento SVG personalizzato.
    pub fn custom(element: Element) -> Self {
        Self::Custom(element)
    }
}

impl From<IconKind> for SvgIcon {
    fn from(kind: IconKind) -> Self {
        SvgIcon::Kind(kind)
    }
}

impl From<Element> for SvgIcon {
    fn from(element: Element) -> Self {
        SvgIcon::Custom(element)
    }
}

/// Componente per il rendering polimorfico di `SvgIcon`.
#[component]
pub fn RenderSvgIcon(
    icon: SvgIcon,
    #[props(default = 16)] size: u32,
    #[props(default)] class: String,
) -> Element {
    match icon {
        SvgIcon::Kind(kind) => rsx! {
            Icon { kind, size, class }
        },
        SvgIcon::Custom(elem) => rsx! {
            span {
                class: "shrink-0 inline-flex items-center justify-center {class}",
                {elem}
            }
        },
    }
}

#[component]
pub fn Icon(
    kind: IconKind,
    #[props(default = 16)] size: u32,
    #[props(default)] class: String,
) -> Element {
    let size_str = size.to_string();

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size_str}",
            height: "{size_str}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: "shrink-0 {class}",
            match kind {
                IconKind::ChevronRight => rsx! {
                    path { d: "m9 18 6-6-6-6" }
                },
                IconKind::ChevronDown => rsx! {
                    path { d: "m6 9 6 6 6-6" }
                },
                IconKind::ChevronLeft => rsx! {
                    path { d: "m15 18-6-6 6-6" }
                },
                IconKind::ChevronUp => rsx! {
                    path { d: "m18 15-6-6-6 6" }
                },
                IconKind::Check => rsx! {
                    path { d: "M20 6 9 17l-5-5" }
                },
                IconKind::Copy => rsx! {
                    rect { width: "14", height: "14", x: "8", y: "8", rx: "2", ry: "2" }
                    path { d: "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" }
                },
                IconKind::X => rsx! {
                    path { d: "M18 6 6 18" }
                    path { d: "m6 6 12 12" }
                },
                IconKind::AlertTriangle => rsx! {
                    path { d: "m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" }
                    line { x1: "12", x2: "12", y1: "9", y2: "13" }
                    line { x1: "12", x2: "12.01", y1: "17", y2: "17" }
                },
                IconKind::AlertCircle => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                    line { x1: "12", x2: "12", y1: "8", y2: "12" }
                    line { x1: "12", x2: "12.01", y1: "16", y2: "16" }
                },
                IconKind::Info => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                    line { x1: "12", x2: "12", y1: "16", y2: "12" }
                    line { x1: "12", x2: "12.01", y1: "8", y2: "8" }
                },
                IconKind::CheckCircle => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                    path { d: "m9 12 2 2 4-4" }
                },
                IconKind::XCircle => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                    path { d: "m15 9-6 6" }
                    path { d: "m9 9 6 6" }
                },
                IconKind::Search => rsx! {
                    circle { cx: "11", cy: "11", r: "8" }
                    path { d: "m21 21-4.3-4.3" }
                },
                IconKind::Sparkles => rsx! {
                    path { d: "m12 3-1.9 5.8a2 2 0 0 1-1.3 1.3L3 12l5.8 1.9a2 2 0 0 1 1.3 1.3L12 21l1.9-5.8a2 2 0 0 1 1.3-1.3L21 12l-5.8-1.9a2 2 0 0 1-1.3-1.3L12 3Z" }
                },
                IconKind::Settings => rsx! {
                    path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                    circle { cx: "12", cy: "12", r: "3" }
                },
                IconKind::Code => rsx! {
                    polyline { points: "16 18 22 12 16 6" }
                    polyline { points: "8 6 2 12 8 18" }
                },
                IconKind::Layers => rsx! {
                    path { d: "m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.84Z" }
                    path { d: "m22 12.5-9.42 4.29a2 2 0 0 1-1.16 0L2 12.5" }
                    path { d: "m22 17.5-9.42 4.29a2 2 0 0 1-1.16 0L2 17.5" }
                },
                IconKind::Terminal => rsx! {
                    polyline { points: "4 17 10 11 4 5" }
                    line { x1: "12", x2: "20", y1: "19", y2: "19" }
                },
                IconKind::Palette => rsx! {
                    circle { cx: "13.5", cy: "6.5", r: ".5", fill: "currentColor" }
                    circle { cx: "17.5", cy: "10.5", r: ".5", fill: "currentColor" }
                    circle { cx: "8.5", cy: "7.5", r: ".5", fill: "currentColor" }
                    circle { cx: "6.5", cy: "12.5", r: ".5", fill: "currentColor" }
                    path { d: "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" }
                },
                IconKind::Eye => rsx! {
                    path { d: "M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" }
                    circle { cx: "12", cy: "12", r: "3" }
                },
                IconKind::EyeOff => rsx! {
                    path { d: "M9.88 9.88a3 3 0 1 0 4.24 4.24" }
                    path { d: "M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68" }
                    path { d: "M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61" }
                    line { x1: "2", x2: "22", y1: "2", y2: "22" }
                },
                IconKind::ExternalLink => rsx! {
                    path { d: "M15 3h6v6" }
                    path { d: "M10 14 21 3" }
                    path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                },
                IconKind::Sun => rsx! {
                    circle { cx: "12", cy: "12", r: "4" }
                    path { d: "M12 2v2" }
                    path { d: "M12 20v2" }
                    path { d: "m4.93 4.93 1.41 1.41" }
                    path { d: "m17.66 17.66 1.41 1.41" }
                    path { d: "M2 12h2" }
                    path { d: "M20 12h2" }
                    path { d: "m6.34 17.66-1.41 1.41" }
                    path { d: "m19.07 4.93-1.41 1.41" }
                },
                IconKind::Moon => rsx! {
                    path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                },
                IconKind::Box => rsx! {
                    path { d: "M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" }
                    path { d: "m3.3 7 8.7 5 8.7-5" }
                    path { d: "M12 22V12" }
                },
                IconKind::Menu => rsx! {
                    line { x1: "4", x2: "20", y1: "12", y2: "12" }
                    line { x1: "4", x2: "20", y1: "6", y2: "6" }
                    line { x1: "4", x2: "20", y1: "18", y2: "18" }
                },
                IconKind::Spinner => rsx! {
                    path {
                        class: "animate-spin origin-center",
                        d: "M21 12a9 9 0 1 1-6.219-8.56"
                    }
                },
            }
        }
    }
}
