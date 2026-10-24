//! Story: RichTextEditor (feature `rich_text`). Stateful: the playground goes through
//! `EditorDemo { knobs }`. `mode` is only the initial mode, so the demo remounts on change.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const MODES: [RichTextMode; 3] = [RichTextMode::Wysiwyg, RichTextMode::Markdown, RichTextMode::Split];
const MODE_OPTS: &[ChoiceOption] = &[
    opt("Visual", "RichTextMode::Wysiwyg"),
    opt("Markdown", "RichTextMode::Markdown"),
    opt("Split", "RichTextMode::Split"),
];
const HEIGHTS: [&str; 3] = ["160px", "240px", "360px"];
const HEIGHT_OPTS: &[ChoiceOption] = &[
    opt("160px", "Some(\"160px\".to_string())"),
    opt("240px", "Some(\"240px\".to_string())"),
    opt("360px", "Some(\"360px\".to_string())"),
];

const PRELUDE: &str = r####"let value = use_signal(|| "## Note di turno\n\n**edge-07** è tornato in linea alle 11:18.\n\n- Relay di Milano attivo\n- Nessuna perdita di pacchetti".to_string());"####;

#[component]
fn EditorDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| {
        "## Note di turno\n\n**edge-07** è tornato in linea alle 11:18.\n\n- Relay di Milano attivo\n- Nessuna perdita di pacchetti"
            .to_string()
    });
    let mode = knobs.index("mode");
    rsx! {
        RichTextEditor {
            key: "{mode}",
            value,
            mode: Some(MODES[mode]),
            placeholder: knobs.text("placeholder"),
            editable: knobs.bool("editable"),
            sticky_toolbar: knobs.bool("sticky_toolbar"),
            min_height: Some(knobs.pick("min_height", &HEIGHTS).to_string()),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        EditorDemo { knobs: k.clone() }
    }
}

#[component]
fn ReadOnly() -> Element {
    let policy = use_signal(|| {
        "### Policy di accesso\n\nSolo gli operatori con **chiave hardware** possono aprire una sessione.\n\n> Le eccezioni vanno approvate dal responsabile di turno.".to_string()
    });
    rsx! {
        RichTextEditor { value: policy, editable: false, min_height: Some("140px".to_string()) }
    }
}
const READ_ONLY_CODE: &str = r####"let policy = use_signal(|| {
    "### Policy di accesso\n\nSolo gli operatori con **chiave hardware** possono aprire una sessione.\n\n> Le eccezioni vanno approvate dal responsabile di turno.".to_string()
});

RichTextEditor { value: policy, editable: false, min_height: Some("140px".to_string()) }"####;

#[component]
fn Split() -> Element {
    let doc = use_signal(|| "# Changelog\n\n- `Select`: menu sopra i campi\n- *Nuovo* preset Cyan".to_string());
    rsx! {
        RichTextEditor { value: doc, mode: Some(RichTextMode::Split), min_height: Some("160px".to_string()) }
    }
}
const SPLIT_CODE: &str = r####"let doc = use_signal(|| "# Changelog\n\n- `Select`: menu sopra i campi\n- *Nuovo* preset Cyan".to_string());

RichTextEditor { value: doc, mode: Some(RichTextMode::Split), min_height: Some("160px".to_string()) }"####;

#[component]
fn InContext() -> Element {
    let title = use_signal(|| "Incidente del 24 ottobre: latenza edge-eu".to_string());
    let body = use_signal(|| {
        "## Cronologia\n\n1. 10:58 picco a 84 ms verso Francoforte\n2. 11:05 relay di Milano attivato\n3. 11:18 latenza normale\n\n## Azioni\n\n- Aggiungere un secondo relay in **eu-south**".to_string()
    });
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Post-mortem".to_string()),
            subtitle: Some("Bozza · visibile al team Ops".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, "Salva bozza" }
                    Button {
                        onclick: move |_| {
                            toast.success("Post-mortem pubblicato");
                        },
                        "Pubblica"
                    }
                }
            }),
            Stack { gap: SpacingToken::Md,
                TextInput { value: title, label: Some("Titolo".to_string()) }
                RichTextEditor { value: body, min_height: Some("220px".to_string()) }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r####"let title = use_signal(|| "Incidente del 24 ottobre: latenza edge-eu".to_string());
let body = use_signal(|| {
    "## Cronologia\n\n1. 10:58 picco a 84 ms verso Francoforte\n2. 11:05 relay di Milano attivato\n3. 11:18 latenza normale\n\n## Azioni\n\n- Aggiungere un secondo relay in **eu-south**".to_string()
});
let mut toast = use_notification();

Card {
    title: Some("Post-mortem".to_string()),
    subtitle: Some("Bozza · visibile al team Ops".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, "Salva bozza" }
            Button {
                onclick: move |_| {
                    toast.success("Post-mortem pubblicato");
                },
                "Pubblica"
            }
        }
    }),
    Stack { gap: SpacingToken::Md,
        TextInput { value: title, label: Some("Titolo".to_string()) }
        RichTextEditor { value: body, min_height: Some("220px".to_string()) }
    }
}"####;

const EDITOR_PROPS: &[PropDoc] = &[
    prop("value", "Signal<String>", "-", "Contenuto in Markdown; la vista visuale lo converte in HTML e viceversa."),
    prop("mode", "Option<RichTextMode>", "None", "Modalità iniziale: Wysiwyg (default), Markdown o Split."),
    prop("placeholder", "String", "\"\"", "Segnaposto; da Locale (rich_text_placeholder) se vuoto."),
    prop("editable", "bool", "true", "false per una vista in sola lettura."),
    prop("sticky_toolbar", "bool", "true", "La toolbar resta visibile scorrendo."),
    prop("min_height", "Option<String>", "None", "Altezza minima dell'area (default 240px)."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const PARTS_PROPS: &[PropDoc] = &[
    prop("RichTextToolbar { sticky }", "component", "true", "Barra per toolbar personalizzate."),
    prop("RichTextControlsGroup", "component", "-", "Gruppo visivo di controlli."),
    prop("RichTextControl { command, active, disabled, onclick }", "component", "-", "Pulsante per un RichTextCommand."),
    prop("markdown_to_html / html_to_markdown", "fn(&str) -> String", "-", "Conversioni usate dall'editor."),
];

pub fn story() -> Story {
    Story {
        slug: "rich-text",
        name: "RichTextEditor",
        category: Category::Contenuti,
        features: &["rich_text"],
        icon: IconKind::Code,
        tagline: "Editor di testo formattato con toolbar, vista Markdown e anteprima affiancata.",
        keywords: &["rich text", "editor", "wysiwyg", "markdown", "testo formattato", "toolbar"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "RichTextEditor",
            controls: vec![
                Control::choice("mode", "Modalità", MODE_OPTS, 0).emit(Emit::Template("mode: Some({v})")),
                Control::text("placeholder", "Placeholder", "").emit(Emit::Template("placeholder: \"{v}\".to_string()")),
                Control::choice("min_height", "Altezza minima", HEIGHT_OPTS, 0).emit(Emit::Always),
                Control::bool("editable", "Modificabile", true),
                Control::bool("sticky_toolbar", "Toolbar fissa", true),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["value,"],
            children_code: None,
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "readonly",
                title: "Sola lettura",
                description: "Il Markdown reso come testo formattato, senza modifica.",
                code: READ_ONLY_CODE,
                render: || rsx! { ReadOnly {} },
                span: Span::Half,
            },
            Example {
                id: "split",
                title: "Markdown e anteprima",
                description: "Sorgente a sinistra, risultato a destra.",
                code: SPLIT_CODE,
                render: || rsx! { Split {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Scrivere un post-mortem con titolo, editor e pubblicazione.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "RichTextEditor", rows: EDITOR_PROPS },
            PropGroup { component: "Parti", rows: PARTS_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Recessed field (§3.6) + items in a track per la toolbar (§3.4)",
            depth: "Well (area di testo) · track (toolbar)",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-radius-md", "--sd-item-h", "--sd-tint-1", "--sd-accent-soft", "--sd-font-mono"],
            notes: "L'area è un pozzetto come un campo; i comandi attivi usano la tinta accento, mai il riempimento pieno.",
        }),
        a11y: &[
            "Ogni comando della toolbar è un <button> con aria-label, title e aria-pressed.",
            "I pulsanti Visual, Markdown e Split espongono aria-pressed.",
            "In modalità Markdown il testo è una <textarea> nativa; in sola lettura è readonly.",
            "Le etichette dei comandi sono fisse in inglese nella libreria (non ancora da Locale).",
        ],
        related: &["textarea", "code-highlight", "form", "chat"],
    }
}
