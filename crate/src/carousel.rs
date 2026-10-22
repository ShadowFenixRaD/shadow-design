use crate::icon::{Icon, IconKind};
#[cfg(feature = "theme")]
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Componente Carosello/Slider interattivo per schede, contenuti e immagini con navigazione,
/// indicatori a punto (*dots*), loop ciclico e transizioni fluide.
#[component]
pub fn Carousel(
    /// Numero totale di slide presenti nel carosello.
    slide_count: usize,
    /// Indice della slide attiva (opzionale se gestito esternamente).
    active_index: Option<Signal<usize>>,
    /// Se consentire il ciclo continuo infinito tra prima e ultima slide. Defaults to true.
    #[props(default = true)]
    r#loop: bool,
    /// Se mostrare le frecce laterali di navigazione precedente/successiva. Defaults to true.
    #[props(default = true)]
    with_controls: bool,
    /// Se mostrare gli indicatori a punto (*dots*) in basso. Defaults to true.
    #[props(default = true)]
    with_indicators: bool,
    /// Se applicare una spaziatura interna protetta tra il contenuto e i bordi/controlli del carosello. Defaults to false.
    #[props(default = false)]
    padded: bool,
    /// Elementi figli (istanze di [`CarouselSlide`]).
    children: Element,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let internal_index = use_signal(|| 0usize);
    let mut current_idx = active_index.unwrap_or(internal_index);

    let count = slide_count.max(1);
    let idx = *current_idx.read();

    let can_go_prev = r#loop || idx > 0;
    let can_go_next = r#loop || idx < count - 1;

    let handle_prev = move |_| {
        if count > 0 {
            let next_val = if idx == 0 {
                if r#loop { count - 1 } else { 0 }
            } else {
                idx - 1
            };
            current_idx.set(next_val);
        }
    };

    let handle_next = move |_| {
        if count > 0 {
            let next_val = if idx >= count - 1 {
                if r#loop { 0 } else { count - 1 }
            } else {
                idx + 1
            };
            current_idx.set(next_val);
        }
    };

    let track_transform = format!("translateX(-{}%)", idx * 100);
    let pad_class = if padded { "sd-carousel--padded" } else { "" };

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div { class: "sd-carousel {pad_class} {class}",
            // Viewport con overflow nascosto
            div { class: "sd-carousel-viewport",
                // Track delle slide con trasformazione animata
                div {
                    class: "sd-carousel-track",
                    style: "transform: {track_transform};",
                    {children}
                }
            }

            // Frecce di controllo
            if with_controls && count > 1 {
                button {
                    r#type: "button",
                    disabled: !can_go_prev,
                    class: format!("sd-carousel-control sd-carousel-control--prev {}", if !can_go_prev { "sd-carousel-control--disabled" } else { "" }),
                    onclick: handle_prev,
                    "aria-label": "Slide precedente",
                    Icon { kind: IconKind::ChevronLeft, size: 18 }
                }
                button {
                    r#type: "button",
                    disabled: !can_go_next,
                    class: format!("sd-carousel-control sd-carousel-control--next {}", if !can_go_next { "sd-carousel-control--disabled" } else { "" }),
                    onclick: handle_next,
                    "aria-label": "Slide successiva",
                    Icon { kind: IconKind::ChevronRight, size: 18 }
                }
            }

            // Indicatori a punto (Dots)
            if with_indicators && count > 1 {
                div { class: "sd-carousel-indicators",
                    for i in 0..count {
                        {
                            let is_active = i == idx;
                            let dot_class = if is_active { "sd-carousel-dot sd-carousel-dot--active" } else { "sd-carousel-dot" };
                            rsx! {
                                button {
                                    key: "{i}",
                                    r#type: "button",
                                    class: "{dot_class}",
                                    "aria-label": "Vai alla slide {i + 1}",
                                    onclick: move |_| current_idx.set(i),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Singola slide all'interno del [`Carousel`].
#[component]
pub fn CarouselSlide(
    /// Contenuto interno della slide.
    children: Element,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    rsx! {
        div { class: "sd-carousel-slide {class}",
            {children}
        }
    }
}
