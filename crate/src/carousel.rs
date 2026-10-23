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

    let prev_slide = move || {
        let mut idx_sig = current_idx;
        if count > 0 {
            let next_val = if idx == 0 {
                if r#loop { count - 1 } else { 0 }
            } else {
                idx - 1
            };
            idx_sig.set(next_val);
        }
    };

    let next_slide = move || {
        let mut idx_sig = current_idx;
        if count > 0 {
            let next_val = if idx >= count - 1 {
                if r#loop { 0 } else { count - 1 }
            } else {
                idx + 1
            };
            idx_sig.set(next_val);
        }
    };

    let track_transform = format!("translateX(-{}%)", idx * 100);
    let pad_class = if padded { "sd-carousel--padded" } else { "" };

    let carousel_id = crate::a11y::use_unique_id("carousel", None);

    use_effect(use_reactive(&carousel_id, move |cid| {
        let js = format!(
            r#"
            (function() {{
                let attempts = 0;
                function tryInit() {{
                    const carousel = document.getElementById("{cid}");
                    if (!carousel) {{
                        if (attempts++ < 30) setTimeout(tryInit, 30);
                        return;
                    }}

                    let hideTimeout = null;

                    function show() {{
                        carousel.classList.add("sd-carousel--controls-visible");
                        if (hideTimeout) clearTimeout(hideTimeout);
                        hideTimeout = setTimeout(() => {{
                            const isHovered = carousel.matches(":hover") && 
                                (carousel.querySelector(".sd-carousel-control:hover") || 
                                 carousel.querySelector(".sd-carousel-indicators:hover"));
                            if (!isHovered) {{
                                carousel.classList.remove("sd-carousel--controls-visible");
                            }}
                        }}, 2400);
                    }}

                    function hide() {{
                        if (hideTimeout) clearTimeout(hideTimeout);
                        carousel.classList.remove("sd-carousel--controls-visible");
                    }}

                    carousel.addEventListener("mouseenter", show);
                    carousel.addEventListener("mousemove", show);
                    carousel.addEventListener("mouseleave", hide);

                    const controls = carousel.querySelectorAll(".sd-carousel-control, .sd-carousel-indicators");
                    controls.forEach(c => {{
                        c.addEventListener("mouseenter", () => {{
                            if (hideTimeout) clearTimeout(hideTimeout);
                            carousel.classList.add("sd-carousel--controls-visible");
                        }});
                    }});
                }}
                tryInit();
            }})();
            "#
        );
        let _ = document::eval(&js);
    }));

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    let on_prev_click = move |_| prev_slide();
    let on_next_click = move |_| next_slide();
    rsx! {
        div {
            id: "{carousel_id}",
            tabindex: "0",
            role: "region",
            "aria-roledescription": "carousel",
            "aria-label": "Carosello di contenuti",
            onkeydown: move |e: KeyboardEvent| {
                match e.key() {
                    Key::ArrowLeft => {
                        e.prevent_default();
                        prev_slide();
                    }
                    Key::ArrowRight => {
                        e.prevent_default();
                        next_slide();
                    }
                    _ => {}
                }
            },
            class: "sd-carousel {pad_class} {class}",
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
                    onclick: on_prev_click,
                    "aria-label": "Slide precedente",
                    Icon { kind: IconKind::ChevronLeft, size: 18 }
                }
                button {
                    r#type: "button",
                    disabled: !can_go_next,
                    class: format!("sd-carousel-control sd-carousel-control--next {}", if !can_go_next { "sd-carousel-control--disabled" } else { "" }),
                    onclick: on_next_click,
                    "aria-label": "Slide successiva",
                    Icon { kind: IconKind::ChevronRight, size: 18 }
                }
            }

            // Indicatori a punto (Dots)
            if with_indicators && count > 1 {
                div { class: "sd-carousel-indicators", role: "tablist",
                    for i in 0..count {
                        {
                            let is_active = i == idx;
                            let dot_class = if is_active { "sd-carousel-dot sd-carousel-dot--active" } else { "sd-carousel-dot" };
                            rsx! {
                                button {
                                    key: "{i}",
                                    r#type: "button",
                                    role: "tab",
                                    "aria-selected": if is_active { "true" } else { "false" },
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
