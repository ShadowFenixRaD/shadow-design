use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Semi-transparent backdrop with animated spinner positioned over containers to block interaction during async operations.
#[component]
pub fn LoadingOverlay(
    /// Whether the loading overlay is currently active.
    visible: bool,
    /// Backdrop blur radius in pixels. Defaults to 3px.
    #[props(default = 3)]
    blur_px: u32,
    /// Size of the central animated spinner icon in pixels. Defaults to 32px.
    #[props(default = 32)]
    loader_size: u32,
    /// Optional status description text rendered below the spinner.
    #[props(default)]
    description: Option<String>,
    /// Additional CSS classes for the root container.
    #[props(default)]
    class: String,
    /// Content wrapped by the loading overlay.
    #[props(default)]
    children: Element,
) -> Element {
    let backdrop_style = format!("backdrop-filter: blur({blur_px}px); -webkit-backdrop-filter: blur({blur_px}px);");

    rsx! {
        div { class: "sd-loading-overlay-wrapper {class}",
            {children}
            if visible {
                div {
                    class: "sd-loading-overlay",
                    style: "{backdrop_style}",
                    "aria-busy": "true",
                    "aria-live": "polite",
                    div { class: "sd-loading-overlay-content",
                        div { class: "sd-loading-spinner",
                            Icon { kind: IconKind::Spinner, size: loader_size }
                        }
                        if let Some(desc) = description {
                            p { class: "sd-loading-overlay-desc", "{desc}" }
                        }
                    }
                }
            }
        }
    }
}
