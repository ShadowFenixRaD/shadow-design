//! Story: AlertDialog and ConfirmDialog (feature `alert_dialog`). Overlay story: the trigger
//! sits in a "danger zone" card; the playground goes through `AlertDialogDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const ICONS: [Option<IconKind>; 4] = [None, Some(IconKind::Trash2), Some(IconKind::ShieldCheck), Some(IconKind::Key)];
const ICON_OPTS: &[ChoiceOption] = &[
    opt("Predefinita", "IconKind::AlertTriangle"),
    opt("Trash2", "IconKind::Trash2"),
    opt("ShieldCheck", "IconKind::ShieldCheck"),
    opt("Key", "IconKind::Key"),
];

const PRELUDE: &str = r#"let mut opened = use_signal(|| false);
let mut toast = use_notification();"#;

#[component]
fn AlertDialogDemo(knobs: Knobs) -> Element {
    let mut opened = use_signal(|| false);
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Zona pericolosa".to_string()),
            subtitle: Some("Repository mesh-config".to_string()),
            Group { gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween,
                Stack { gap: SpacingToken::Xs, align: AlignItems::Start,
                    Text { size: TextSize::Sm, "Elimina il repository" }
                    Text { size: TextSize::Sm, dimmed: true, "Branch, tag e webhook vengono rimossi da tutti i nodi." }
                }
                Button {
                    variant: ButtonVariant::Danger,
                    left_icon: Some(rsx! {
                        Icon { kind: IconKind::Trash2, size: 15 }
                    }),
                    onclick: move |_| opened.set(true),
                    "Elimina repository"
                }
            }
        }
        AlertDialog {
            opened,
            on_confirm: move |_| {
                toast.danger("Repository eliminato");
            },
            title: knobs.text("title"),
            description: knobs.text("description"),
            confirm_label: knobs.text("confirm_label"),
            cancel_label: knobs.text("cancel_label"),
            is_destructive: knobs.bool("is_destructive"),
            icon: knobs.pick("icon", &ICONS),
            glow: knobs.bool("glow").then_some(true),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        AlertDialogDemo { knobs: k.clone() }
    }
}

#[component]
fn Confirm() -> Element {
    let mut opened = use_signal(|| false);
    let mut toast = use_notification();
    rsx! {
        Group {
            Button {
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Upload, size: 15 }
                }),
                onclick: move |_| opened.set(true),
                "Pubblica v0.5.0"
            }
        }
        ConfirmDialog {
            opened,
            title: "Pubblicare la release v0.5.0?".to_string(),
            description: "Il pacchetto va su crates.io e la documentazione viene rigenerata.".to_string(),
            confirm_label: "Pubblica".to_string(),
            cancel_label: "Non ancora".to_string(),
            icon: Some(IconKind::CheckCircle),
            on_confirm: move |_| {
                toast.success("Release v0.5.0 pubblicata");
            },
        }
    }
}
const CONFIRM_CODE: &str = r#"let mut opened = use_signal(|| false);
let mut toast = use_notification();

Group {
    Button {
        left_icon: Some(rsx! {
            Icon { kind: IconKind::Upload, size: 15 }
        }),
        onclick: move |_| opened.set(true),
        "Pubblica v0.5.0"
    }
}
ConfirmDialog {
    opened,
    title: "Pubblicare la release v0.5.0?".to_string(),
    description: "Il pacchetto va su crates.io e la documentazione viene rigenerata.".to_string(),
    confirm_label: "Pubblica".to_string(),
    cancel_label: "Non ancora".to_string(),
    icon: Some(IconKind::CheckCircle),
    on_confirm: move |_| {
        toast.success("Release v0.5.0 pubblicata");
    },
}"#;

#[component]
fn Extra() -> Element {
    let mut opened = use_signal(|| false);
    let notify = use_signal(|| true);
    let mut toast = use_notification();
    rsx! {
        Group {
            Button { variant: ButtonVariant::Danger, onclick: move |_| opened.set(true), "Revoca accesso" }
        }
        AlertDialog {
            opened,
            title: "Revocare l'accesso a Marco Bellini?".to_string(),
            description: "Le sue sessioni attive terminano subito e le chiavi personali vengono invalidate.".to_string(),
            confirm_label: "Revoca".to_string(),
            icon: Some(IconKind::Key),
            on_confirm: move |_| {
                toast.warning("Accesso revocato");
            },
            Checkbox { checked: notify, label: Some("Avvisa Marco via email".to_string()) }
        }
    }
}
const EXTRA_CODE: &str = r#"let mut opened = use_signal(|| false);
let notify = use_signal(|| true);
let mut toast = use_notification();

Group {
    Button { variant: ButtonVariant::Danger, onclick: move |_| opened.set(true), "Revoca accesso" }
}
AlertDialog {
    opened,
    title: "Revocare l'accesso a Marco Bellini?".to_string(),
    description: "Le sue sessioni attive terminano subito e le chiavi personali vengono invalidate.".to_string(),
    confirm_label: "Revoca".to_string(),
    icon: Some(IconKind::Key),
    on_confirm: move |_| {
        toast.warning("Accesso revocato");
    },
    Checkbox { checked: notify, label: Some("Avvisa Marco via email".to_string()) }
}"#;

#[component]
fn InContext() -> Element {
    let mut nodes = use_signal(|| vec![("edge-07", "Milano"), ("edge-12", "Francoforte"), ("relay-03", "Parigi")]);
    let mut target = use_signal(|| None::<&'static str>);
    let mut opened = use_signal(|| false);
    let mut toast = use_notification();
    let name = target().unwrap_or_default();
    rsx! {
        Card { title: Some("Nodi collegati".to_string()), subtitle: Some("Cluster eu-west".to_string()),
            Stack { gap: SpacingToken::Sm,
                for (node, region) in nodes() {
                    Group { key: "{node}", gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween,
                        Stack { gap: SpacingToken::None, align: AlignItems::Start,
                            Text { size: TextSize::Sm, "{node}" }
                            Text { size: TextSize::Xs, dimmed: true, "{region}" }
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                target.set(Some(node));
                                opened.set(true);
                            },
                            "Scollega"
                        }
                    }
                }
                if nodes().is_empty() {
                    Text { size: TextSize::Sm, dimmed: true, "Nessun nodo collegato." }
                }
            }
        }
        AlertDialog {
            opened,
            title: format!("Scollegare {name}?"),
            description: "Il traffico del nodo passa ai peer vicini; potrai ricollegarlo dalla console.".to_string(),
            confirm_label: "Scollega".to_string(),
            on_confirm: move |_| {
                nodes.write().retain(|(n, _)| Some(*n) != target());
                toast.danger(format!("{name} scollegato"));
            },
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut nodes = use_signal(|| vec![("edge-07", "Milano"), ("edge-12", "Francoforte"), ("relay-03", "Parigi")]);
let mut target = use_signal(|| None::<&'static str>);
let mut opened = use_signal(|| false);
let mut toast = use_notification();
let name = target().unwrap_or_default();

Card { title: Some("Nodi collegati".to_string()), subtitle: Some("Cluster eu-west".to_string()),
    Stack { gap: SpacingToken::Sm,
        for (node, region) in nodes() {
            Group { gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween,
                Stack { gap: SpacingToken::None, align: AlignItems::Start,
                    Text { size: TextSize::Sm, "{node}" }
                    Text { size: TextSize::Xs, dimmed: true, "{region}" }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        target.set(Some(node));
                        opened.set(true);
                    },
                    "Scollega"
                }
            }
        }
        if nodes().is_empty() {
            Text { size: TextSize::Sm, dimmed: true, "Nessun nodo collegato." }
        }
    }
}
AlertDialog {
    opened,
    title: format!("Scollegare {name}?"),
    description: "Il traffico del nodo passa ai peer vicini; potrai ricollegarlo dalla console.".to_string(),
    confirm_label: "Scollega".to_string(),
    on_confirm: move |_| {
        nodes.write().retain(|(n, _)| Some(*n) != target());
        toast.danger(format!("{name} scollegato"));
    },
}"#;

const PROPS: &[PropDoc] = &[
    prop("opened", "Signal<bool>", "-", "Visibilità; conferma, annulla e clic sul backdrop la riportano a false."),
    prop("title", "String", "-", "Domanda di conferma, in forma diretta."),
    prop("description", "String", "-", "Conseguenze dell'azione."),
    prop("confirm_label / cancel_label", "String", "\"\"", "Testi dei pulsanti; vuoti usano Locale (confirm, cancel)."),
    prop("is_destructive", "bool", "true", "Pulsante Danger e icona danger; false usa Primary."),
    prop("icon", "Option<IconKind>", "AlertTriangle", "Icona nel badge in testa."),
    prop("on_confirm", "Option<EventHandler<()>>", "None", "Chiamato dopo la chiusura per conferma."),
    prop("on_cancel", "Option<EventHandler<()>>", "None", "Chiamato su Annulla o clic sul backdrop."),
    prop("glow", "Option<bool>", "None", "Glow perimetrale opzionale."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul pannello."),
    prop("children", "Option<Element>", "None", "Contenuto extra tra descrizione e azioni (checkbox, riepilogo)."),
];
const CONFIRM_PROPS: &[PropDoc] = &[
    prop("is_destructive", "bool", "false", "Unica differenza da AlertDialog: parte non distruttivo."),
    prop("…", "-", "-", "Tutte le altre prop sono identiche ad AlertDialog."),
];

pub fn story() -> Story {
    Story {
        slug: "alert-dialog",
        name: "AlertDialog",
        category: Category::Overlay,
        features: &["alert_dialog"],
        icon: IconKind::AlertTriangle,
        tagline: "Dialogo bloccante che chiede una conferma esplicita prima di un'azione critica o irreversibile.",
        keywords: &["alert dialog", "confirm", "conferma", "dialogo", "distruttivo", "elimina"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "AlertDialog",
            controls: vec![
                Control::text("title", "Titolo", "Eliminare mesh-config?").emit(Emit::Template("title: \"{v}\".to_string()")),
                Control::text("description", "Descrizione", "L'azione è irreversibile: branch, tag e webhook vengono rimossi.")
                    .emit(Emit::Template("description: \"{v}\".to_string()")),
                Control::text("confirm_label", "Conferma", "Elimina definitivamente")
                    .emit(Emit::Template("confirm_label: \"{v}\".to_string()")),
                Control::text("cancel_label", "Annulla", "").emit(Emit::Template("cancel_label: \"{v}\".to_string()")),
                Control::choice("icon", "Icona", ICON_OPTS, 0).emit(Emit::Template("icon: Some({v})")),
                Control::bool("is_destructive", "Distruttivo", true),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["opened,", "on_confirm: move |_| {", "    toast.danger(\"Repository eliminato\");", "},"],
            children_code: None,
            stage: Stage::Fill,
        }),
        examples: vec![
            Example {
                id: "confirm",
                title: "ConfirmDialog",
                description: "Variante non distruttiva per una conferma ordinaria.",
                code: CONFIRM_CODE,
                render: || rsx! { Confirm {} },
                span: Span::Half,
            },
            Example {
                id: "extra",
                title: "Contenuto extra",
                description: "Una checkbox tra descrizione e azioni.",
                code: EXTRA_CODE,
                render: || rsx! { Extra {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Scollegare un nodo dal cluster: il titolo nomina la riga scelta.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "AlertDialog", rows: PROPS },
            PropGroup { component: "ConfirmDialog", rows: CONFIRM_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Floating panel (language.md §3.2) + backdrop (§3.3)",
            depth: "Raised-max",
            tokens: &["--sd-surface-2", "--sd-elev-3", "--sd-radius-xl", "--sd-scrim", "--sd-danger-soft", "--sd-danger-strong", "--sd-accent-soft"],
            notes: "L'icona è una tinta soft (§3.10); l'azione di conferma è l'unico pulsante pieno: Danger o Primary.",
        }),
        a11y: &[
            "Pannello con role=alertdialog e aria-modal=true: gli screen reader lo annunciano come interruzione.",
            "Titolo (h3) e descrizione sono testo visibile, letti in ordine prima delle azioni.",
            "Annulla e Conferma sono pulsanti nativi, raggiungibili con Tab e attivabili con Invio o Spazio.",
            "Esc non chiude il dialogo: la scelta resta esplicita; il clic sul backdrop vale come Annulla.",
        ],
        related: &["modals", "drawer", "alerts", "notifications"],
    }
}
