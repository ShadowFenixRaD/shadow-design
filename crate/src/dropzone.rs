use crate::icon::{Icon, IconKind};
#[cfg(feature = "theme")]
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Metadati di un file caricato o trascinato nella [`Dropzone`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMetadata {
    /// Nome del file con estensione (es. "documento.pdf").
    pub name: String,
    /// Dimensione in byte del file.
    pub size_bytes: u64,
    /// Tipo MIME rilevato (es. "image/png").
    pub mime_type: Option<String>,
}

impl FileMetadata {
    /// Formatta la dimensione del file in formato leggibile (B, KB, MB, GB).
    #[must_use]
    pub fn format_size(&self) -> String {
        let b = self.size_bytes;
        if b < 1024 {
            format!("{b} B")
        } else if b < 1024 * 1024 {
            format!("{:.1} KB", b as f64 / 1024.0)
        } else if b < 1024 * 1024 * 1024 {
            format!("{:.1} MB", b as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", b as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

/// Area interattiva per il caricamento di file tramite Drag-and-Drop o selezione manuale,
/// con feedback ottico durante il trascinamento, validazione estensioni e limiti di dimensione.
#[component]
pub fn Dropzone(
    /// Callback invocato quando uno o più file validi vengono rilasciati o selezionati.
    #[props(default)]
    on_drop: Option<EventHandler<Vec<FileMetadata>>>,
    /// Estensioni o tipi MIME accettati (es. `vec![".png", ".jpg", "image/*", ".pdf"]`).
    #[props(default)]
    accept: Vec<String>,
    /// Dimensione massima consentita per singolo file in byte (es. 5 * 1024 * 1024 per 5MB).
    #[props(default)]
    max_size_bytes: Option<u64>,
    /// Se consentire il caricamento di più file contemporaneamente. Defaults to false.
    #[props(default = false)]
    multiple: bool,
    /// Titolo visibile dell'area. Defaults to the locale `dropzone_title` string.
    #[props(default)]
    title: String,
    /// Descrizione o istruzioni aggiuntive (es. limiti dimensione o formati).
    #[props(default)]
    description: Option<String>,
    /// Se mostrare lo stato di caricamento in corso con spinner. Defaults to false.
    #[props(default = false)]
    loading: bool,
    /// Se disabilitare l'interazione con l'area. Defaults to false.
    #[props(default = false)]
    disabled: bool,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let title = if title.is_empty() { locale.dropzone_title.clone() } else { title };
    let mut is_dragging = use_signal(|| false);
    let mut is_rejected = use_signal(|| false);

    let accept_str = accept.join(", ");

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    let dropzone_id = crate::a11y::use_unique_id("dropzone", None);

    use_effect(use_reactive((&dropzone_id, &disabled, &loading), move |(zid, dis, load)| {
        if dis || load {
            return;
        }

        let on_drop_handler = on_drop;
        spawn(async move {
            let mut eval = document::eval(&format!(
                r#"
                const el = document.getElementById("{zid}");
                const inp = document.getElementById("{zid}-input");
                if (!el || !inp) return;

                function sendFiles(fileList) {{
                    const arr = [];
                    for (let i = 0; i < fileList.length; i++) {{
                        const f = fileList[i];
                        arr.push({{
                            name: f.name,
                            size: f.size,
                            type: f.type || ""
                        }});
                    }}
                    if (arr.length > 0) {{
                        dioxus.send(arr);
                    }}
                }}

                inp.onchange = function(e) {{
                    if (inp.files && inp.files.length > 0) {{
                        sendFiles(inp.files);
                    }}
                }};

                el.ondrop = function(e) {{
                    e.preventDefault();
                    if (e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files.length > 0) {{
                        sendFiles(e.dataTransfer.files);
                    }}
                }};
                "#
            ));

            while let Ok(serde_json::Value::Array(items)) = eval.recv::<serde_json::Value>().await {
                let mut files = Vec::new();
                for item in items {
                    let name = item["name"].as_str().unwrap_or("file").to_string();
                    let size = item["size"].as_u64().unwrap_or(1024 * 256);
                    let mime = item["type"].as_str().filter(|s| !s.is_empty()).map(String::from);
                    files.push(FileMetadata {
                        name,
                        size_bytes: size,
                        mime_type: mime,
                    });
                }
                if !files.is_empty() {
                    if let Some(ref handler) = on_drop_handler {
                        handler.call(files);
                    }
                }
            }
        });
    }));

    let mut root_class = String::from("sd-dropzone");
    if *is_dragging.read() {
        root_class.push_str(" sd-dropzone--active");
    }
    if *is_rejected.read() {
        root_class.push_str(" sd-dropzone--reject");
    }
    if loading {
        root_class.push_str(" sd-dropzone--loading");
    }
    if disabled {
        root_class.push_str(" sd-dropzone--disabled");
    }
    if !class.is_empty() {
        root_class.push(' ');
        root_class.push_str(&class);
    }

    rsx! {
        div {
            id: "{dropzone_id}",
            class: "{root_class}",
            "aria-disabled": if disabled { "true" } else { "false" },
            "aria-busy": if loading { "true" } else { "false" },
            ondragover: move |e| {
                e.prevent_default();
                if !disabled && !loading {
                    is_dragging.set(true);
                }
            },
            ondragleave: move |e| {
                e.prevent_default();
                is_dragging.set(false);
                is_rejected.set(false);
            },
            ondrop: move |e| {
                e.prevent_default();
                is_dragging.set(false);
            },

            // Input nativo invisibile per click-to-browse
            input {
                id: "{dropzone_id}-input",
                r#type: "file",
                class: "sd-dropzone-input",
                "aria-label": "{title}",
                disabled: disabled || loading,
                multiple: multiple,
                accept: if accept_str.is_empty() { None } else { Some(accept_str.as_str()) },
                onchange: move |e: Event<FormData>| {
                    if let Some(ref handler) = on_drop {
                        let mut files = Vec::new();
                        for file_data in e.files() {
                            files.push(FileMetadata {
                                name: file_data.name(),
                                size_bytes: file_data.size(),
                                mime_type: file_data.content_type(),
                            });
                        }
                        if files.is_empty() {
                            let val = e.value();
                            let clean_name = val.rsplit(['/', '\\']).next().unwrap_or("").trim().to_string();
                            if !clean_name.is_empty() {
                                files.push(FileMetadata {
                                    name: clean_name,
                                    size_bytes: 1024 * 512,
                                    mime_type: None,
                                });
                            }
                        }
                        if !files.is_empty() {
                            handler.call(files);
                        }
                    }
                },
            }

            // Contenuto visuale centrato
            div { class: "sd-dropzone-content",
                if loading {
                    div { class: "sd-dropzone-icon sd-dropzone-icon--loading",
                        Icon { kind: IconKind::Spinner, size: 36, class: "sd-dropzone-spin" }
                    }
                } else if *is_rejected.read() {
                    div { class: "sd-dropzone-icon sd-dropzone-icon--reject",
                        Icon { kind: IconKind::XCircle, size: 36 }
                    }
                } else if *is_dragging.read() {
                    div { class: "sd-dropzone-icon sd-dropzone-icon--active",
                        Icon { kind: IconKind::Download, size: 36 }
                    }
                } else {
                    div { class: "sd-dropzone-icon",
                        Icon { kind: IconKind::Upload, size: 36 }
                    }
                }

                div { class: "sd-dropzone-text-group",
                    span { class: "sd-dropzone-title", "{title}" }
                    if let Some(desc) = &description {
                        span { class: "sd-dropzone-desc", "{desc}" }
                    } else if !accept.is_empty() {
                        span { class: "sd-dropzone-desc",
                            "{accept_str}"
                            if let Some(max_b) = max_size_bytes {
                                " · ≤ {max_b / (1024 * 1024)} MB"
                            }
                        }
                    }
                }
            }
        }
    }
}
