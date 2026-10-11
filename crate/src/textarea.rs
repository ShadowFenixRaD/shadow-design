use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Componente `Textarea` multilinea con supporto ad autosize reattivo, conteggio caratteri,
/// cancellazione rapida (clearable) e integrazione con il tema Shadow.
#[component]
pub fn Textarea(
    /// Signal reattivo per il contenuto testuale multilinea.
    value: Signal<String>,
    /// Etichetta descrittiva posizionata sopra al campo.
    #[props(default)]
    label: Option<String>,
    /// Testo segnaposto informativo visibile quando il campo è vuoto.
    #[props(default)]
    placeholder: Option<String>,
    /// Testo di aiuto o istruzione secondaria sotto al campo.
    #[props(default)]
    help_text: Option<String>,
    /// Messaggio di errore che evidenzia il bordo in rosso fenice e mostra il messaggio.
    #[props(default)]
    error: Option<String>,
    /// Disabilita il campo impedendo l'interazione.
    #[props(default)]
    disabled: bool,
    /// Rende il campo in sola lettura.
    #[props(default)]
    readonly: bool,
    /// Numero minimo di righe visualizzate (default: 3).
    #[props(default = 3)]
    min_rows: usize,
    /// Numero massimo di righe per espansione autosize (default: None, illimitato).
    #[props(default)]
    max_rows: Option<usize>,
    /// Abilita l'espansione verticale automatica in proporzione alle righe digitate.
    #[props(default)]
    autosize: bool,
    /// Mostra il conteggio dei caratteri in basso a destra.
    #[props(default)]
    show_count: bool,
    /// Limite massimo di caratteri consentiti.
    #[props(default)]
    max_length: Option<usize>,
    /// Mostra un pulsante rapido (X) per cancellare il contenuto.
    #[props(default)]
    clearable: bool,
    /// Alone ottico perimetrale coordinato con il tema attivo.
    #[props(default)]
    glow: Option<bool>,
    /// Classi CSS o SCSS addizionali.
    #[props(default)]
    class: String,
    /// Callback invocata ad ogni variazione del testo.
    #[props(default)]
    on_change: Option<EventHandler<String>>,
) -> Element {
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let radius_class = theme_val.radius_scale.md();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("textarea").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("textarea", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("textarea", glow);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let radius_class = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow.unwrap_or(false) {
        "sd-textarea--glow".to_string()
    } else {
        String::new()
    };

    let text_val = value();
    let char_count = text_val.chars().count();

    // Calcolo righe dinamiche per autosize
    let rows_to_display = if autosize {
        let lines = text_val.matches('\n').count() + 1;
        let mut r = lines.max(min_rows);
        if let Some(max_r) = max_rows {
            r = r.min(max_r);
        }
        r
    } else {
        min_rows
    };

    let has_clear_btn = clearable && !text_val.is_empty() && !disabled && !readonly;

    let autosize_class = if autosize { "sd-textarea--autosize" } else { "" };
    let error_class = if error.is_some() { "sd-textarea--error" } else { "" };
    let disabled_class = if disabled { "sd-textarea--disabled" } else { "" };
    let clear_padding_class = if has_clear_btn { "sd-textarea--has-clear" } else { "" };

    let is_at_limit = if let Some(max) = max_length {
        char_count >= max
    } else {
        false
    };

    rsx! {
        div {
            class: "sd-textarea-wrapper {class_modifiers} {class}",
            if let Some(lbl) = label {
                label { class: "sd-textarea-label", "{lbl}" }
            }
            div {
                class: "sd-textarea-container",
                textarea {
                    class: "sd-textarea {autosize_class} {error_class} {disabled_class} {clear_padding_class} {base_override} {radius_class} {glow_class}",
                    placeholder: placeholder.unwrap_or_default(),
                    disabled,
                    readonly,
                    rows: "{rows_to_display}",
                    maxlength: max_length.map(|m| m.to_string()),
                    value: "{text_val}",
                    oninput: move |e| {
                        let new_val = e.value();
                        *value.write() = new_val.clone();
                        if let Some(h) = &on_change {
                            h.call(new_val);
                        }
                    },
                }

                if has_clear_btn {
                    button {
                        r#type: "button",
                        class: "sd-textarea-clear",
                        title: "Cancella testo",
                        onclick: move |e| {
                            e.stop_propagation();
                            *value.write() = String::new();
                            if let Some(h) = &on_change {
                                h.call(String::new());
                            }
                        },
                        Icon { kind: IconKind::X, size: 12 }
                    }
                }
            }

            if error.is_some() || help_text.is_some() || show_count {
                div {
                    class: "sd-textarea-footer",
                    if let Some(err) = error {
                        span { class: "sd-textarea-error", "{err}" }
                    } else if let Some(help) = help_text {
                        span { class: "sd-textarea-help", "{help}" }
                    } else {
                        span {}
                    }

                    if show_count {
                        span {
                            class: if is_at_limit { "sd-textarea-counter sd-textarea-counter--limit" } else { "sd-textarea-counter" },
                            if let Some(max) = max_length {
                                "{char_count} / {max}"
                            } else {
                                "{char_count} caratteri"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Variante con espansione verticale dinamica proporzionale al contenuto abilitata per impostazione predefinita.
#[component]
pub fn AutoResizeTextarea(
    /// Signal reattivo per il contenuto testuale multilinea.
    value: Signal<String>,
    /// Etichetta descrittiva superiore.
    #[props(default)]
    label: Option<String>,
    /// Testo segnaposto informativo visibile a campo vuoto.
    #[props(default)]
    placeholder: Option<String>,
    /// Testo di aiuto o istruzione secondaria sotto al campo.
    #[props(default)]
    help_text: Option<String>,
    /// Messaggio di errore.
    #[props(default)]
    error: Option<String>,
    /// Disabilita il campo.
    #[props(default)]
    disabled: bool,
    /// Rende il campo in sola lettura.
    #[props(default)]
    readonly: bool,
    /// Numero minimo di righe visualizzate (default: 2).
    #[props(default = 2)]
    min_rows: usize,
    /// Numero massimo di righe per espansione autosize (default: None).
    #[props(default)]
    max_rows: Option<usize>,
    /// Mostra il conteggio dei caratteri in basso a destra.
    #[props(default)]
    show_count: bool,
    /// Limite massimo di caratteri consentiti.
    #[props(default)]
    max_length: Option<usize>,
    /// Mostra un pulsante rapido per cancellare il testo.
    #[props(default)]
    clearable: bool,
    /// Alone ottico perimetrale coordinato con il tema attivo.
    #[props(default)]
    glow: Option<bool>,
    /// Classi CSS o SCSS addizionali.
    #[props(default)]
    class: String,
    /// Callback invocata ad ogni variazione del testo.
    #[props(default)]
    on_change: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        Textarea {
            value,
            label,
            placeholder,
            help_text,
            error,
            disabled,
            readonly,
            min_rows,
            max_rows,
            autosize: true,
            show_count,
            max_length,
            clearable,
            glow,
            class,
            on_change,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_autosize_row_calculation() {
        let text = "riga 1\nriga 2\nriga 3\nriga 4";
        let lines = text.matches('\n').count() + 1;
        assert_eq!(lines, 4);

        let min_rows = 2;
        let max_rows = Some(3);
        let calculated = lines.max(min_rows).min(max_rows.unwrap());
        assert_eq!(calculated, 3);
    }

    #[test]
    fn test_char_count() {
        let text = "ombra fenice";
        assert_eq!(text.chars().count(), 12);
    }
}
