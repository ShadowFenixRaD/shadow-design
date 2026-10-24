#[cfg(feature = "theme")]
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Renderer virtualizzato ad alte prestazioni (60 FPS) per elenchi massivi di elementi.
/// Esegue il rendering nel DOM unicamente degli elementi visibili nella finestra di scorrimento corrente.
#[component]
pub fn VirtualList(
    /// Numero complessivo totale di elementi nell'elenco.
    item_count: usize,
    /// Altezza fissa in pixel di ogni singola riga/elemento. Defaults to 48.
    #[props(default = 48)]
    item_height: u32,
    /// Altezza complessiva in pixel del viewport visibile. Defaults to 360.
    #[props(default = 360)]
    height: u32,
    /// Numero di elementi aggiuntivi da pre-renderizzare sopra e sotto la finestra (overscan). Defaults to 4.
    #[props(default = 4)]
    overscan: usize,
    /// Funzione di rendering per il singolo elemento dato il suo indice numerico (0..item_count).
    render_item: Callback<usize, Element>,
    /// Classi CSS aggiuntive per il contenitore.
    #[props(default)]
    class: String,
) -> Element {
    let mut scroll_top = use_signal(|| 0u32);

    let total_height = (item_count as u64) * (item_height as u64);
    let cur_scroll = *scroll_top.read();

    let safe_item_height = item_height.max(1);
    let start_idx = (cur_scroll / safe_item_height) as usize;
    let visible_count = height.div_ceil(safe_item_height) as usize;

    let render_start = start_idx.saturating_sub(overscan);
    let render_end = (start_idx + visible_count + overscan).min(item_count);

    let offset_y = (render_start as u32) * item_height;

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div {
            class: "sd-virtual-list {class}",
            style: "height: {height}px;",
            onscroll: move |e| {
                let top = (e.data().scroll_top().max(0.0)) as u32;
                scroll_top.set(top);
            },

            // Elemento fantasma che simula l'intera altezza dello scorrimento
            div {
                class: "sd-virtual-list-spacer",
                style: "height: {total_height}px;",
            }

            // Slice renderizzata traslata alla posizione corrente
            div {
                class: "sd-virtual-list-content",
                style: "transform: translateY({offset_y}px);",
                for idx in render_start..render_end {
                    div {
                        key: "{idx}",
                        class: "sd-virtual-list-item",
                        style: "height: {item_height}px;",
                        {render_item.call(idx)}
                    }
                }
            }
        }
    }
}
