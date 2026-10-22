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
    // Chat & Messaggi
    Send,
    MessageSquare,
    Paperclip,
    CheckCheck,
    Clock,
    // Sicurezza & Crittografia
    Lock,
    Unlock,
    Shield,
    ShieldCheck,
    Key,
    QrCode,
    Fingerprint,
    // Connettività & Radio
    Wifi,
    Bluetooth,
    Radio,
    Signal,
    SignalHigh,
    Server,
    Share2,
    // Azioni Chat
    Trash2,
    Flame,
    Download,
    Upload,
    // Azioni Generali & Floating Overlays
    Plus,
    // Tooling & Estensioni
    Calendar,
    GripVertical,
}

impl IconKind {
    pub const ALL: &'static [IconKind] = &[
        IconKind::ChevronRight,
        IconKind::ChevronDown,
        IconKind::ChevronLeft,
        IconKind::ChevronUp,
        IconKind::Check,
        IconKind::Copy,
        IconKind::X,
        IconKind::AlertTriangle,
        IconKind::AlertCircle,
        IconKind::Info,
        IconKind::CheckCircle,
        IconKind::XCircle,
        IconKind::Search,
        IconKind::Sparkles,
        IconKind::Settings,
        IconKind::Code,
        IconKind::Layers,
        IconKind::Terminal,
        IconKind::Palette,
        IconKind::Eye,
        IconKind::EyeOff,
        IconKind::ExternalLink,
        IconKind::Sun,
        IconKind::Moon,
        IconKind::Box,
        IconKind::Menu,
        IconKind::Spinner,
        IconKind::Send,
        IconKind::MessageSquare,
        IconKind::Paperclip,
        IconKind::CheckCheck,
        IconKind::Clock,
        IconKind::Lock,
        IconKind::Unlock,
        IconKind::Shield,
        IconKind::ShieldCheck,
        IconKind::Key,
        IconKind::QrCode,
        IconKind::Fingerprint,
        IconKind::Wifi,
        IconKind::Bluetooth,
        IconKind::Radio,
        IconKind::Signal,
        IconKind::SignalHigh,
        IconKind::Server,
        IconKind::Share2,
        IconKind::Trash2,
        IconKind::Flame,
        IconKind::Download,
        IconKind::Upload,
        IconKind::Plus,
        IconKind::Calendar,
        IconKind::GripVertical,
    ];

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            IconKind::ChevronRight => "ChevronRight",
            IconKind::ChevronDown => "ChevronDown",
            IconKind::ChevronLeft => "ChevronLeft",
            IconKind::ChevronUp => "ChevronUp",
            IconKind::Check => "Check",
            IconKind::Copy => "Copy",
            IconKind::X => "X",
            IconKind::AlertTriangle => "AlertTriangle",
            IconKind::AlertCircle => "AlertCircle",
            IconKind::Info => "Info",
            IconKind::CheckCircle => "CheckCircle",
            IconKind::XCircle => "XCircle",
            IconKind::Search => "Search",
            IconKind::Sparkles => "Sparkles",
            IconKind::Settings => "Settings",
            IconKind::Code => "Code",
            IconKind::Layers => "Layers",
            IconKind::Terminal => "Terminal",
            IconKind::Palette => "Palette",
            IconKind::Eye => "Eye",
            IconKind::EyeOff => "EyeOff",
            IconKind::ExternalLink => "ExternalLink",
            IconKind::Sun => "Sun",
            IconKind::Moon => "Moon",
            IconKind::Box => "Box",
            IconKind::Menu => "Menu",
            IconKind::Spinner => "Spinner",
            IconKind::Send => "Send",
            IconKind::MessageSquare => "MessageSquare",
            IconKind::Paperclip => "Paperclip",
            IconKind::CheckCheck => "CheckCheck",
            IconKind::Clock => "Clock",
            IconKind::Lock => "Lock",
            IconKind::Unlock => "Unlock",
            IconKind::Shield => "Shield",
            IconKind::ShieldCheck => "ShieldCheck",
            IconKind::Key => "Key",
            IconKind::QrCode => "QrCode",
            IconKind::Fingerprint => "Fingerprint",
            IconKind::Wifi => "Wifi",
            IconKind::Bluetooth => "Bluetooth",
            IconKind::Radio => "Radio",
            IconKind::Signal => "Signal",
            IconKind::SignalHigh => "SignalHigh",
            IconKind::Server => "Server",
            IconKind::Share2 => "Share2",
            IconKind::Trash2 => "Trash2",
            IconKind::Flame => "Flame",
            IconKind::Download => "Download",
            IconKind::Upload => "Upload",
            IconKind::Plus => "Plus",
            IconKind::Calendar => "Calendar",
            IconKind::GripVertical => "GripVertical",
        }
    }
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
    #[must_use]
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
    #[props(default)] label: Option<String>,
) -> Element {
    match icon {
        SvgIcon::Kind(kind) => rsx! {
            Icon { kind, size, class, label }
        },
        SvgIcon::Custom(elem) => {
            if let Some(lbl) = label {
                rsx! {
                    span {
                        role: "img",
                        aria_label: "{lbl}",
                        class: "shrink-0 inline-flex items-center justify-center {class}",
                        {elem}
                    }
                }
            } else {
                rsx! {
                    span {
                        aria_hidden: "true",
                        class: "shrink-0 inline-flex items-center justify-center {class}",
                        {elem}
                    }
                }
            }
        }
    }
}

#[component]
pub fn Icon(
    kind: IconKind,
    #[props(default = 16)] size: u32,
    #[props(default)] class: String,
    #[props(default)] label: Option<String>,
) -> Element {
    let size_str = size.to_string();
    let spinner_class = if matches!(kind, IconKind::Spinner) { " animate-spin" } else { "" };
    let paths = render_icon_paths(kind);

    if let Some(lbl) = label {
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
                "role": "img",
                "aria-label": "{lbl}",
                "focusable": "false",
                class: "shrink-0 {class}{spinner_class}",
                {paths}
            }
        }
    } else {
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
                "aria-hidden": "true",
                "focusable": "false",
                class: "shrink-0 {class}{spinner_class}",
                {paths}
            }
        }
    }
}

fn render_icon_paths(kind: IconKind) -> Element {
    rsx! {
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
                        d: "M21 12a9 9 0 1 1-6.219-8.56"
                    }
                },
                IconKind::Send => rsx! {
                    path { d: "m22 2-7 20-4-9-9-4Z" }
                    path { d: "M22 2 11 13" }
                },
                IconKind::MessageSquare => rsx! {
                    path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                },
                IconKind::Paperclip => rsx! {
                    path { d: "m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48" }
                },
                IconKind::CheckCheck => rsx! {
                    path { d: "M18 6 7 17l-5-5" }
                    path { d: "m22 10-7.5 7.5L13 16" }
                },
                IconKind::Clock => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                    polyline { points: "12 6 12 12 16 14" }
                },
                IconKind::Lock => rsx! {
                    rect { width: "18", height: "11", x: "3", y: "11", rx: "2", ry: "2" }
                    path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                },
                IconKind::Unlock => rsx! {
                    rect { width: "18", height: "11", x: "3", y: "11", rx: "2", ry: "2" }
                    path { d: "M7 11V7a5 5 0 0 1 9.9-1" }
                },
                IconKind::Shield => rsx! {
                    path { d: "M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" }
                },
                IconKind::ShieldCheck => rsx! {
                    path { d: "M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" }
                    path { d: "m9 12 2 2 4-4" }
                },
                IconKind::Key => rsx! {
                    circle { cx: "7.5", cy: "15.5", r: "5.5" }
                    path { d: "m21 2-9.6 9.6" }
                    path { d: "m15.5 7.5 3 3L22 7l-3-3" }
                },
                IconKind::QrCode => rsx! {
                    rect { width: "5", height: "5", x: "3", y: "3", rx: "1" }
                    rect { width: "5", height: "5", x: "16", y: "3", rx: "1" }
                    rect { width: "5", height: "5", x: "3", y: "16", rx: "1" }
                    path { d: "M21 16h-3a2 2 0 0 0-2 2v3" }
                    path { d: "M21 21v.01" }
                    path { d: "M12 7v3a2 2 0 0 1-2 2H7" }
                    path { d: "M3 12h.01" }
                    path { d: "M12 3h.01" }
                    path { d: "M12 16v.01" }
                    path { d: "M16 12h1" }
                    path { d: "M21 12v.01" }
                    path { d: "M12 21v-1" }
                },
                IconKind::Fingerprint => rsx! {
                    path { d: "M12 10a2 2 0 0 0-2 2c0 1.02-.1 2.51-.26 4" }
                    path { d: "M14 13.12c0 2.38 0 6.38-1 8.88" }
                    path { d: "M17.29 21.02c.12-.6.43-2.3.5-3.02" }
                    path { d: "M2 12a10 10 0 0 1 18-6" }
                    path { d: "M2 16h.01" }
                    path { d: "M21.8 16c.2-2 .131-5.354 0-6" }
                    path { d: "M5 19.5C5.5 18 6 15 6 12a6 6 0 0 1 .34-2" }
                    path { d: "M8.65 22c.21-.66.45-1.32.57-2" }
                    path { d: "M9 6.8a6 6 0 0 1 9 5.2v2" }
                },
                IconKind::Wifi => rsx! {
                    path { d: "M12 20h.01" }
                    path { d: "M2 8.82a15 15 0 0 1 20 0" }
                    path { d: "M5 12.86a10 10 0 0 1 14 0" }
                    path { d: "M8.5 16.43a5 5 0 0 1 7 0" }
                },
                IconKind::Bluetooth => rsx! {
                    path { d: "m7 7 10 10-5 5V2l5 5L7 17" }
                },
                IconKind::Radio => rsx! {
                    path { d: "M4.9 19.1C1 15.2 1 8.8 4.9 4.9" }
                    path { d: "M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5" }
                    circle { cx: "12", cy: "12", r: "2" }
                    path { d: "M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5" }
                    path { d: "M19.1 4.9C23 8.8 23 15.2 19.1 19.1" }
                },
                IconKind::Signal | IconKind::SignalHigh => rsx! {
                    path { d: "M2 20h.01" }
                    path { d: "M7 20v-4" }
                    path { d: "M12 20v-8" }
                    path { d: "M17 20V8" }
                    path { d: "M22 20V4" }
                },
                IconKind::Server => rsx! {
                    rect { width: "20", height: "8", x: "2", y: "2", rx: "2", ry: "2" }
                    rect { width: "20", height: "8", x: "2", y: "14", rx: "2", ry: "2" }
                    line { x1: "6", x2: "6.01", y1: "6", y2: "6" }
                    line { x1: "6", x2: "6.01", y1: "18", y2: "18" }
                },
                IconKind::Share2 => rsx! {
                    circle { cx: "18", cy: "5", r: "3" }
                    circle { cx: "6", cy: "12", r: "3" }
                    circle { cx: "18", cy: "19", r: "3" }
                    line { x1: "8.59", x2: "15.42", y1: "13.51", y2: "17.49" }
                    line { x1: "15.41", x2: "8.59", y1: "6.51", y2: "10.49" }
                },
                IconKind::Trash2 => rsx! {
                    path { d: "M3 6h18" }
                    path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                    path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                    line { x1: "10", x2: "10", y1: "11", y2: "17" }
                    line { x1: "14", x2: "14", y1: "11", y2: "17" }
                },
                IconKind::Flame => rsx! {
                    path { d: "M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z" }
                },
                IconKind::Download => rsx! {
                    path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                    polyline { points: "7 10 12 15 17 10" }
                    line { x1: "12", x2: "12", y1: "15", y2: "3" }
                },
                IconKind::Upload => rsx! {
                    path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                    polyline { points: "17 8 12 3 7 8" }
                    line { x1: "12", x2: "12", y1: "3", y2: "15" }
                },
                IconKind::Plus => rsx! {
                    line { x1: "12", x2: "12", y1: "5", y2: "19" }
                    line { x1: "5", x2: "19", y1: "12", y2: "12" }
                },
                IconKind::Calendar => rsx! {
                    rect { width: "18", height: "18", x: "3", y: "4", rx: "2" }
                    line { x1: "16", x2: "16", y1: "2", y2: "6" }
                    line { x1: "8", x2: "8", y1: "2", y2: "6" }
                    line { x1: "3", x2: "21", y1: "10", y2: "10" }
                },
                IconKind::GripVertical => rsx! {
                    circle { cx: "9", cy: "12", r: "1" }
                    circle { cx: "9", cy: "5", r: "1" }
                    circle { cx: "9", cy: "19", r: "1" }
                    circle { cx: "15", cy: "12", r: "1" }
                    circle { cx: "15", cy: "5", r: "1" }
                    circle { cx: "15", cy: "19", r: "1" }
                },
            }
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_kind_all_completeness() {
        assert_eq!(IconKind::ALL.len(), 53);
        for icon in IconKind::ALL {
            assert!(!icon.name().is_empty());
        }
    }
}
