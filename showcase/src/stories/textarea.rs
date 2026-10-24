//! Story: Textarea and AutoResizeTextarea (feature `textarea`). Stateful: the playground goes
//! through `TextareaDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const PRELUDE: &str = r#"let value = use_signal(|| "Ruota le chiavi Kyber dei peer edge ogni 24 ore.".to_string());"#;

#[component]
fn TextareaDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| "Ruota le chiavi Kyber dei peer edge ogni 24 ore.".to_string());
    rsx! {
        Textarea {
            value,
            label: Some(knobs.text("label")).filter(|s| !s.is_empty()),
            placeholder: Some(knobs.text("placeholder")).filter(|s| !s.is_empty()),
            help_text: Some(knobs.text("help_text")).filter(|s| !s.is_empty()),
            min_rows: knobs.number("min_rows") as usize,
            autosize: knobs.bool("autosize"),
            show_count: knobs.bool("show_count"),
            max_length: knobs.bool("max_length").then_some(280),
            clearable: knobs.bool("clearable"),
            resize_height: knobs.bool("resize_height"),
            readonly: knobs.bool("readonly"),
            disabled: knobs.bool("disabled"),
            error: knobs.bool("error").then(|| "Descrivi almeno l'impatto sui peer".to_string()),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        TextareaDemo { knobs: k.clone() }
    }
}

#[component]
fn AutoResize() -> Element {
    let notes = use_signal(|| "Scrivi qui: il campo cresce riga per riga fino a sei righe.".to_string());
    rsx! {
        AutoResizeTextarea {
            value: notes,
            label: Some("Appunti di turno".to_string()),
            min_rows: 2,
            max_rows: Some(6),
            clearable: true,
        }
    }
}
const AUTO_RESIZE_CODE: &str = r#"let notes = use_signal(|| "Scrivi qui: il campo cresce riga per riga fino a sei righe.".to_string());

AutoResizeTextarea {
    value: notes,
    label: Some("Appunti di turno".to_string()),
    min_rows: 2,
    max_rows: Some(6),
    clearable: true,
}"#;

#[component]
fn Count() -> Element {
    let commit = use_signal(|| "fix(mesh): riduci la latenza del relay".to_string());
    rsx! {
        Textarea {
            value: commit,
            label: Some("Messaggio di commit".to_string()),
            help_text: Some("Prima riga entro 72 caratteri.".to_string()),
            min_rows: 2,
            show_count: true,
            max_length: Some(72),
            resize_height: true,
        }
    }
}
const COUNT_CODE: &str = r#"let commit = use_signal(|| "fix(mesh): riduci la latenza del relay".to_string());

Textarea {
    value: commit,
    label: Some("Messaggio di commit".to_string()),
    help_text: Some("Prima riga entro 72 caratteri.".to_string()),
    min_rows: 2,
    show_count: true,
    max_length: Some(72),
    resize_height: true,
}"#;

#[component]
fn States() -> Element {
    let policy = use_signal(|| "Accesso consentito solo da reti mesh verificate.".to_string());
    let report = use_signal(String::new);
    rsx! {
        Stack { gap: SpacingToken::Md,
            Textarea { value: policy, label: Some("Policy (sola lettura)".to_string()), readonly: true, min_rows: 2 }
            Textarea {
                value: report,
                label: Some("Descrizione dell'incidente".to_string()),
                placeholder: Some("Cosa è successo e quando?".to_string()),
                error: if report().is_empty() { Some("La descrizione è obbligatoria".to_string()) } else { None },
                min_rows: 2,
            }
        }
    }
}
const STATES_CODE: &str = r#"let policy = use_signal(|| "Accesso consentito solo da reti mesh verificate.".to_string());
let report = use_signal(String::new);

Stack { gap: SpacingToken::Md,
    Textarea { value: policy, label: Some("Policy (sola lettura)".to_string()), readonly: true, min_rows: 2 }
    Textarea {
        value: report,
        label: Some("Descrizione dell'incidente".to_string()),
        placeholder: Some("Cosa è successo e quando?".to_string()),
        error: if report().is_empty() { Some("La descrizione è obbligatoria".to_string()) } else { None },
        min_rows: 2,
    }
}"#;

#[component]
fn InContext() -> Element {
    let title = use_signal(|| "v0.5.3 · relay più rapido".to_string());
    let notes = use_signal(|| "- Latenza del relay ridotta del 18%\n- Nuovo preset Cyan".to_string());
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Nuova release".to_string()),
            subtitle: Some("Pubblicata su tutti i nodi edge".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, "Salva bozza" }
                    Button {
                        disabled: notes().trim().is_empty(),
                        onclick: move |_| {
                            toast.success("Release pubblicata");
                        },
                        "Pubblica"
                    }
                }
            }),
            Stack { gap: SpacingToken::Md,
                TextInput { value: title, label: Some("Titolo".to_string()) }
                AutoResizeTextarea {
                    value: notes,
                    label: Some("Note di rilascio".to_string()),
                    help_text: Some("Markdown supportato. Una voce per riga.".to_string()),
                    min_rows: 3,
                    max_rows: Some(10),
                    show_count: true,
                    max_length: Some(500),
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let title = use_signal(|| "v0.5.3 · relay più rapido".to_string());
let notes = use_signal(|| "- Latenza del relay ridotta del 18%\n- Nuovo preset Cyan".to_string());
let mut toast = use_notification();

Card {
    title: Some("Nuova release".to_string()),
    subtitle: Some("Pubblicata su tutti i nodi edge".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, "Salva bozza" }
            Button {
                disabled: notes().trim().is_empty(),
                onclick: move |_| {
                    toast.success("Release pubblicata");
                },
                "Pubblica"
            }
        }
    }),
    Stack { gap: SpacingToken::Md,
        TextInput { value: title, label: Some("Titolo".to_string()) }
        AutoResizeTextarea {
            value: notes,
            label: Some("Note di rilascio".to_string()),
            help_text: Some("Markdown supportato. Una voce per riga.".to_string()),
            min_rows: 3,
            max_rows: Some(10),
            show_count: true,
            max_length: Some(500),
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("value", "Signal<String>", "-", "Testo multilinea."),
    prop("label / placeholder", "Option<String>", "None", "Etichetta collegata e segnaposto."),
    prop("help_text", "Option<String>", "None", "Testo di aiuto nel piè di pagina, collegato con aria-describedby."),
    prop("error", "Option<String>", "None", "Bordo danger, aria-invalid; sostituisce help_text nel piè di pagina."),
    prop("min_rows", "usize", "3", "Righe minime visibili."),
    prop("max_rows", "Option<usize>", "None", "Righe massime con autosize, poi scorre."),
    prop("autosize", "bool", "false", "L'altezza segue il contenuto."),
    prop("show_count", "bool", "false", "Contatore caratteri (n / max quando c'è max_length)."),
    prop("max_length", "Option<usize>", "None", "Limite di caratteri (maxlength HTML); il contatore lo evidenzia al limite."),
    prop("clearable", "bool", "false", "Pulsante di cancellazione quando il campo non è vuoto."),
    prop("resize_width / resize_height", "bool", "false", "Maniglia di ridimensionamento orizzontale e/o verticale."),
    prop("readonly / disabled", "bool", "false", "Sola lettura (selezionabile) o disabilitato."),
    prop("glow", "Option<bool>", "None", "Forza o rimuove il glow del campo."),
    prop("on_change", "Option<EventHandler<String>>", "None", "Ogni modifica del testo."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];
const AUTO_PROPS: &[PropDoc] = &[
    prop("min_rows", "usize", "2", "Righe minime (default più basso di Textarea)."),
    prop("max_rows", "Option<usize>", "None", "Oltre questo limite il campo scorre."),
    prop("value, label, error, show_count, …", "-", "-", "Tutte le altre props di Textarea, tranne autosize che è sempre attivo."),
];

pub fn story() -> Story {
    Story {
        slug: "textarea",
        name: "Textarea",
        category: Category::Form,
        features: &["textarea"],
        icon: IconKind::MessageSquare,
        tagline: "Testo su più righe, con altezza automatica, contatore di caratteri e ridimensionamento.",
        keywords: &["textarea", "testo", "multilinea", "autosize", "contatore", "commento"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Textarea",
            controls: vec![
                Control::text("label", "Etichetta", "Nota operativa").emit(Emit::Template("label: Some(\"{v}\".to_string())")),
                Control::text("placeholder", "Placeholder", "Scrivi una nota per il turno")
                    .emit(Emit::Template("placeholder: Some(\"{v}\".to_string())")),
                Control::text("help_text", "Aiuto", "").emit(Emit::Template("help_text: Some(\"{v}\".to_string())")),
                Control::number("min_rows", "Righe minime", 3.0, 1.0, 8.0, 1.0, 0),
                Control::bool("autosize", "Altezza automatica", false),
                Control::bool("show_count", "Contatore", true).emit(Emit::Template("show_count: true")),
                Control::bool("max_length", "Limite 280", true).emit(Emit::Template("max_length: Some(280)")),
                Control::bool("clearable", "Azzerabile", false),
                Control::bool("resize_height", "Ridimensiona altezza", false),
                Control::bool("readonly", "Sola lettura", false),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("error", "Errore", false)
                    .emit(Emit::Template("error: Some(\"Descrivi almeno l'impatto sui peer\".to_string())")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["value,"],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "autoresize",
                title: "AutoResizeTextarea",
                description: "Cresce con il testo tra due e sei righe.",
                code: AUTO_RESIZE_CODE,
                render: || rsx! { AutoResize {} },
                span: Span::Half,
            },
            Example {
                id: "count",
                title: "Contatore e limite",
                description: "Il contatore cambia colore al limite; altezza ridimensionabile.",
                code: COUNT_CODE,
                render: || rsx! { Count {} },
                span: Span::Half,
            },
            Example {
                id: "states",
                title: "Stati",
                description: "Sola lettura ed errore finché il campo è vuoto.",
                code: STATES_CODE,
                render: || rsx! { States {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Note di rilascio con titolo, contatore e pubblicazione.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Textarea", rows: PROPS },
            PropGroup { component: "AutoResizeTextarea", rows: AUTO_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Recessed field (language.md §3.6)",
            depth: "Well",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-inset-well", "--sd-radius-md", "--sd-accent", "--sd-glow-sm", "--sd-text-xs"],
            notes: "Stesso pozzetto di TextInput; contatore e aiuto nel piè di pagina in --sd-text-xs subtle.",
        }),
        a11y: &[
            "Elemento <textarea> nativo con <label for> collegato.",
            "help_text ed error sono collegati con aria-describedby; error imposta aria-invalid.",
            "max_length imposta maxlength: il browser blocca l'input oltre il limite.",
            "Il pulsante di cancellazione usa il testo di Locale (clear_text) come aria-label.",
        ],
        related: &["inputs", "rich-text", "form", "chat"],
    }
}
