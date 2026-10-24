//! Story: TransferList (feature `transfer_list`). Both columns are signals, so the playground
//! goes through `TransferDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

fn available() -> Vec<TransferItem> {
    vec![
        TransferItem::new("logs", "Leggere i log").with_description("Registro eventi di tutti i nodi"),
        TransferItem::new("restart", "Riavviare i nodi").with_description("Uno alla volta, con conferma"),
        TransferItem::new("keys", "Ruotare le chiavi").with_description("Kyber-1024 e firme dei peer"),
        TransferItem::new("export", "Esportare i report").with_description("CSV e PDF settimanali"),
        TransferItem::new("users", "Gestire gli utenti").with_description("Richiede il ruolo Admin").with_disabled(true),
    ]
}

fn granted() -> Vec<TransferItem> {
    vec![
        TransferItem::new("dashboard", "Vedere la dashboard").with_description("Metriche e grafici in sola lettura"),
        TransferItem::new("alerts", "Ricevere gli avvisi").with_description("Email e notifiche push"),
    ]
}

const PRELUDE: &str = r#"let mut toast = use_notification();
let source = use_signal(|| vec![
    TransferItem::new("logs", "Leggere i log").with_description("Registro eventi di tutti i nodi"),
    TransferItem::new("restart", "Riavviare i nodi").with_description("Uno alla volta, con conferma"),
    TransferItem::new("keys", "Ruotare le chiavi").with_description("Kyber-1024 e firme dei peer"),
    TransferItem::new("export", "Esportare i report").with_description("CSV e PDF settimanali"),
    TransferItem::new("users", "Gestire gli utenti").with_description("Richiede il ruolo Admin").with_disabled(true),
]);
let target = use_signal(|| vec![
    TransferItem::new("dashboard", "Vedere la dashboard").with_description("Metriche e grafici in sola lettura"),
    TransferItem::new("alerts", "Ricevere gli avvisi").with_description("Email e notifiche push"),
]);"#;

/// Stateful demo: owns the two column signals the playground cannot (render is a plain fn).
#[component]
fn TransferDemo(knobs: Knobs) -> Element {
    let mut toast = use_notification();
    let source = use_signal(available);
    let target = use_signal(granted);
    let notify = knobs.bool("on_change");
    rsx! {
        TransferList {
            source,
            target,
            searchable: knobs.bool("searchable"),
            height: format!("{}px", knobs.number("height")),
            titles: (knobs.text("source_title"), knobs.text("target_title")),
            on_change: move |(_, granted): (Vec<TransferItem>, Vec<TransferItem>)| {
                if notify {
                    toast.info(format!("{} permessi assegnati", granted.len()));
                }
            },
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        TransferDemo { knobs: k.clone() }
    }
}

#[component]
fn Team() -> Element {
    let source = use_signal(|| {
        vec![
            TransferItem::new("giulia", "Giulia Ferri"),
            TransferItem::new("marco", "Marco Riva"),
            TransferItem::new("luca", "Luca Neri").with_description("In ferie fino al 4 novembre").with_disabled(true),
        ]
    });
    let target = use_signal(|| vec![TransferItem::new("elena", "Elena Sarti"), TransferItem::new("anna", "Anna Conti")]);
    rsx! {
        TransferList {
            source,
            target,
            searchable: false,
            titles: ("Disponibili".to_string(), "In reperibilità".to_string()),
            height: "200px".to_string(),
        }
    }
}
const TEAM_CODE: &str = r#"let source = use_signal(|| {
    vec![
        TransferItem::new("giulia", "Giulia Ferri"),
        TransferItem::new("marco", "Marco Riva"),
        TransferItem::new("luca", "Luca Neri").with_description("In ferie fino al 4 novembre").with_disabled(true),
    ]
});
let target = use_signal(|| vec![TransferItem::new("elena", "Elena Sarti"), TransferItem::new("anna", "Anna Conti")]);

TransferList {
    source,
    target,
    searchable: false,
    titles: ("Disponibili".to_string(), "In reperibilità".to_string()),
    height: "200px".to_string(),
}"#;

#[component]
fn Regions() -> Element {
    let source = use_signal(|| {
        ["Milano", "Francoforte", "Parigi", "Madrid", "Stoccolma", "Varsavia", "Dublino", "Vienna"]
            .iter()
            .map(|c| TransferItem::new(c.to_lowercase(), *c))
            .collect::<Vec<_>>()
    });
    let target = use_signal(Vec::<TransferItem>::new);
    rsx! {
        TransferList {
            source,
            target,
            titles: ("Regioni".to_string(), "Replica attiva".to_string()),
            height: "200px".to_string(),
        }
    }
}
const REGIONS_CODE: &str = r#"let source = use_signal(|| {
    ["Milano", "Francoforte", "Parigi", "Madrid", "Stoccolma", "Varsavia", "Dublino", "Vienna"]
        .iter()
        .map(|c| TransferItem::new(c.to_lowercase(), *c))
        .collect::<Vec<_>>()
});
let target = use_signal(Vec::<TransferItem>::new);

TransferList {
    source,
    target,
    titles: ("Regioni".to_string(), "Replica attiva".to_string()),
    height: "200px".to_string(),
}"#;

#[component]
fn InContext() -> Element {
    let mut source = use_signal(available);
    let mut target = use_signal(granted);
    let mut toast = use_notification();
    let count = target.read().len();
    rsx! {
        Card {
            title: Some("Ruolo Operatore".to_string()),
            subtitle: Some("Scegli cosa può fare chi ha questo ruolo".to_string()),
            buttons: Some(rsx! {
                Badge { label: format!("{count} permessi"), variant: BadgeVariant::Primary, pill: true }
            }),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button {
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| {
                            source.set(available());
                            target.set(granted());
                        },
                        "Ripristina"
                    }
                    Button {
                        onclick: move |_| {
                            toast.success(format!("Ruolo Operatore salvato con {} permessi", target.read().len()));
                        },
                        "Salva ruolo"
                    }
                }
            }),
            TransferList {
                source,
                target,
                titles: ("Permessi disponibili".to_string(), "Permessi del ruolo".to_string()),
                height: "240px".to_string(),
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut source = use_signal(available);
let mut target = use_signal(granted);
let mut toast = use_notification();
let count = target.read().len();

Card {
    title: Some("Ruolo Operatore".to_string()),
    subtitle: Some("Scegli cosa può fare chi ha questo ruolo".to_string()),
    buttons: Some(rsx! {
        Badge { label: format!("{count} permessi"), variant: BadgeVariant::Primary, pill: true }
    }),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button {
                variant: ButtonVariant::Ghost,
                onclick: move |_| {
                    source.set(available());
                    target.set(granted());
                },
                "Ripristina"
            }
            Button {
                onclick: move |_| {
                    toast.success(format!("Ruolo Operatore salvato con {} permessi", target.read().len()));
                },
                "Salva ruolo"
            }
        }
    }),
    TransferList {
        source,
        target,
        titles: ("Permessi disponibili".to_string(), "Permessi del ruolo".to_string()),
        height: "240px".to_string(),
    }
}"#;

const LIST_PROPS: &[PropDoc] = &[
    prop("source", "Signal<Vec<TransferItem>>", "-", "Colonna di sinistra; il componente la riscrive a ogni spostamento."),
    prop("target", "Signal<Vec<TransferItem>>", "-", "Colonna di destra."),
    prop("searchable", "bool", "true", "Campo di ricerca per etichetta in cima a ogni colonna."),
    prop("titles", "(String, String)", "(\"Source\", \"Target\")", "Intestazioni delle due colonne: passale sempre, il default è in inglese."),
    prop("height", "String", "\"320px\"", "Altezza CSS del corpo scorrevole di ogni colonna."),
    prop("on_change", "Option<EventHandler<(Vec<TransferItem>, Vec<TransferItem>)>>", "None", "Chiamato dopo ogni spostamento con le due colonne aggiornate."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("TransferItem::new(id, label)", "fn", "-", "Costruttore."),
    prop(".with_description(..)", "fn", "-", "Riga secondaria sotto l'etichetta."),
    prop(".with_disabled(..)", "fn(bool)", "false", "Voce visibile ma né selezionabile né spostabile, neanche con \"sposta tutto\"."),
];

pub fn story() -> Story {
    Story {
        slug: "transfer-list",
        name: "TransferList",
        category: Category::Dati,
        features: &["transfer_list"],
        icon: IconKind::Layers,
        tagline: "Due colonne per assegnare elementi da un insieme a un altro, con selezione multipla e ricerca.",
        keywords: &["transfer list", "trasferimento", "assegnazione", "permessi", "dual list", "selezione multipla"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "TransferList",
            controls: vec![
                Control::text("source_title", "Titolo sinistra", "Disponibili").emit(Emit::Skip),
                Control::text("target_title", "Titolo destra", "Assegnati").emit(Emit::Skip),
                Control::bool("searchable", "Ricerca", true),
                Control::number("height", "Altezza (px)", 320.0, 160.0, 400.0, 20.0, 0)
                    .emit(Emit::Template("height: \"{v}px\".to_string()")),
                Control::bool("on_change", "Notifica a ogni spostamento", false).emit(Emit::Template(
                    "on_change: move |(_, granted): (Vec<TransferItem>, Vec<TransferItem>)| { toast.info(format!(\"{} permessi assegnati\", granted.len())); }",
                )),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["source,", "target,"],
            children_code: Some("titles: (\"{source_title}\".to_string(), \"{target_title}\".to_string()),"),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "team",
                title: "Senza ricerca",
                description: "Pochi nomi e una persona non assegnabile con with_disabled.",
                code: TEAM_CODE,
                render: || rsx! { Team {} },
                span: Span::Half,
            },
            Example {
                id: "regions",
                title: "Destinazione vuota",
                description: "Si parte da zero: la colonna di destra mostra il testo vuoto da Locale.",
                code: REGIONS_CODE,
                render: || rsx! { Regions {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "L'editor di un ruolo: il badge conta i permessi, il footer salva o ripristina.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "TransferList", rows: LIST_PROPS },
            PropGroup { component: "TransferItem", rows: ITEM_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container (language.md §3.1) + recessed field (§3.6) + items (§3.4) + selected (§3.7)",
            depth: "Flat-lit (colonne) · Well (ricerca)",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-tint-1", "--sd-well", "--sd-item-h", "--sd-accent-soft", "--sd-accent-border"],
            notes: "Due colonne gemelle con intestazione in tinta; le voci selezionate usano sd-selected, i comandi centrali sono pulsanti ghost.",
        }),
        a11y: &[
            "Ogni colonna è un role=listbox con aria-multiselectable; le voci sono role=option con aria-selected.",
            "Le voci sono nel tab order: Invio e Spazio le selezionano; quelle disabilitate hanno aria-disabled e tabindex=-1.",
            "I quattro comandi di spostamento sono <button> con aria-label e title da Locale; quelli \"selezionati\" sono disabilitati finché non scegli una voce.",
            "Il campo di ricerca ha come nome accessibile il proprio placeholder (da Locale).",
        ],
        related: &["lists", "select", "checkbox", "trees"],
    }
}
