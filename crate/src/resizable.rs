use crate::icon::{Icon, IconKind};
#[cfg(feature = "theme")]
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Direzione di divisione e ridimensionamento dei pannelli.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResizableDirection {
    /// Disposizione orizzontale (pannelli affiancati da sinistra a destra).
    #[default]
    Horizontal,
    /// Disposizione verticale (pannelli incolonnati dall'alto in basso).
    Vertical,
}

/// Contenitore per pannelli ridimensionabili con maniglie di separazione trascinabili.
#[component]
pub fn ResizablePanelGroup(
    /// Direzione di divisione dei pannelli. Defaults to `Horizontal`.
    #[props(default)]
    direction: ResizableDirection,
    /// Elementi figli (pannelli [`ResizablePanel`] e maniglie [`ResizableHandle`]).
    children: Element,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let dir_class = match direction {
        ResizableDirection::Horizontal => "sd-resizable-group--horizontal",
        ResizableDirection::Vertical => "sd-resizable-group--vertical",
    };

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div { class: "sd-resizable-group {dir_class} {class}",
            {children}
        }
    }
}

/// Singolo pannello all'interno di un [`ResizablePanelGroup`] con dimensione percentuale e vincoli min/max.
#[component]
pub fn ResizablePanel(
    /// Dimensione percentuale iniziale o corrente (0.0..=100.0). Defaults to 50.0.
    #[props(default = 50.0)]
    default_size: f64,
    /// Dimensione percentuale minima consentita. Defaults to 10.0.
    #[props(default = 10.0)]
    min_size: f64,
    /// Dimensione percentuale massima consentita. Defaults to 90.0.
    #[props(default = 90.0)]
    max_size: f64,
    /// Contenuto interno del pannello.
    children: Element,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let size_clamped = default_size.clamp(min_size, max_size);
    let flex_basis = format!("{size_clamped:.2}%");

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div {
            class: "sd-resizable-panel {class}",
            style: "flex-basis: {flex_basis}; flex-grow: 0; flex-shrink: 0;",
            {children}
        }
    }
}

/// Maniglia divisoria interattiva trascinabile posizionata tra due [`ResizablePanel`].
#[component]
pub fn ResizableHandle(
    /// Se mostrare l'icona centrale con indicatore di presa tattile. Defaults to true.
    #[props(default = true)]
    with_handle: bool,
    /// Se disabilitare il ridimensionamento della maniglia. Defaults to false.
    #[props(default = false)]
    disabled: bool,
    /// Callback invocato durante il trascinamento con delta pixel o percentuale.
    #[props(default)]
    on_drag: Option<EventHandler<f64>>,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let mut is_active = use_signal(|| false);

    let active_class = if *is_active.read() { "sd-resizable-handle--active" } else { "" };
    let disabled_class = if disabled { "sd-resizable-handle--disabled" } else { "" };

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div {
            class: "sd-resizable-handle {active_class} {disabled_class} {class}",
            onmousedown: move |_| {
                if !disabled {
                    is_active.set(true);
                }
            },
            onmouseup: move |_| {
                is_active.set(false);
            },
            if with_handle {
                div { class: "sd-resizable-handle-grip",
                    Icon { kind: IconKind::GripVertical, size: 14 }
                }
            }
        }
    }
}
