//! Story: Indicator (feature `indicator`). Stateless: the playground renders it directly.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::InboxPane;
use crate::story::schema::*;

const POSITIONS: [IndicatorPosition; 4] = [
    IndicatorPosition::TopEnd,
    IndicatorPosition::TopStart,
    IndicatorPosition::BottomEnd,
    IndicatorPosition::BottomStart,
];
const POSITION_OPTS: &[ChoiceOption] = &[
    opt("TopEnd", "IndicatorPosition::TopEnd"),
    opt("TopStart", "IndicatorPosition::TopStart"),
    opt("BottomEnd", "IndicatorPosition::BottomEnd"),
    opt("BottomStart", "IndicatorPosition::BottomStart"),
];
const COLORS: [IndicatorColor; 5] = [
    IndicatorColor::Violet,
    IndicatorColor::Emerald,
    IndicatorColor::Amber,
    IndicatorColor::Rose,
    IndicatorColor::Cyan,
];
const COLOR_OPTS: &[ChoiceOption] = &[
    opt("Violet (accento)", "IndicatorColor::Violet"),
    opt("Emerald (success)", "IndicatorColor::Emerald"),
    opt("Amber (warning)", "IndicatorColor::Amber"),
    opt("Rose (danger)", "IndicatorColor::Rose"),
    opt("Cyan (info)", "IndicatorColor::Cyan"),
];
const SIZES: [IndicatorSize; 3] = [IndicatorSize::Sm, IndicatorSize::Md, IndicatorSize::Lg];
const SIZE_OPTS: &[ChoiceOption] = &[
    opt("Sm", "IndicatorSize::Sm"),
    opt("Md", "IndicatorSize::Md"),
    opt("Lg", "IndicatorSize::Lg"),
];
/// The wrapped element. Its option code is printed as the block's children.
const TARGET_OPTS: &[ChoiceOption] = &[
    opt("Pulsante", "Button { variant: ButtonVariant::Secondary, left_icon: Some(rsx! { Icon { kind: IconKind::MessageSquare, size: 15 } }), \"Messaggi\" }"),
    opt("Avatar", "Avatar { name: Some(\"Giulia Ferri\".to_string()), size: AvatarSize::Lg }"),
];

fn render(k: &Knobs) -> Element {
    let label = k.text("label");
    let offset = k.number("offset") as i32;
    let target = if k.index("target") == 0 {
        rsx! {
            Button { variant: ButtonVariant::Secondary, left_icon: Some(rsx! { Icon { kind: IconKind::MessageSquare, size: 15 } }), "Messaggi" }
        }
    } else {
        rsx! {
            Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Lg }
        }
    };
    rsx! {
        Indicator {
            position: k.pick("position", &POSITIONS),
            color: k.pick("color", &COLORS),
            size: k.pick("size", &SIZES),
            label: (!label.is_empty()).then_some(label),
            processing: k.bool("processing"),
            disabled: k.bool("disabled"),
            offset: (offset != 0).then_some(offset),
            {target}
        }
    }
}

#[component]
fn Colors() -> Element {
    rsx! {
        Group { gap: SpacingToken::Lg,
            Indicator { processing: true,
                Avatar { name: Some("Sync".to_string()) }
            }
            Indicator { color: IndicatorColor::Emerald,
                Avatar { name: Some("Marco Riva".to_string()) }
            }
            Indicator { color: IndicatorColor::Amber,
                Avatar { name: Some("Elena Sarti".to_string()) }
            }
            Indicator { color: IndicatorColor::Rose, label: Some("9+".to_string()),
                Avatar { name: Some("Luca Neri".to_string()) }
            }
            Indicator { color: IndicatorColor::Cyan, label: Some("2".to_string()),
                Avatar { name: Some("Anna Conti".to_string()) }
            }
        }
    }
}
const COLORS_CODE: &str = r#"Group { gap: SpacingToken::Lg,
    Indicator { processing: true,
        Avatar { name: Some("Sync".to_string()) }
    }
    Indicator { color: IndicatorColor::Emerald,
        Avatar { name: Some("Marco Riva".to_string()) }
    }
    Indicator { color: IndicatorColor::Amber,
        Avatar { name: Some("Elena Sarti".to_string()) }
    }
    Indicator { color: IndicatorColor::Rose, label: Some("9+".to_string()),
        Avatar { name: Some("Luca Neri".to_string()) }
    }
    Indicator { color: IndicatorColor::Cyan, label: Some("2".to_string()),
        Avatar { name: Some("Anna Conti".to_string()) }
    }
}"#;

#[component]
fn Placement() -> Element {
    rsx! {
        Group { gap: SpacingToken::Lg, align: AlignItems::Center,
            Indicator { size: IndicatorSize::Sm,
                Button { variant: ButtonVariant::Ghost, aria_label: Some("Cronologia, nuove voci".to_string()),
                    Icon { kind: IconKind::Clock, size: 16 }
                }
            }
            Indicator { position: IndicatorPosition::BottomEnd, color: IndicatorColor::Emerald, size: IndicatorSize::Lg, offset: Some(6),
                Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Lg }
            }
            Indicator { position: IndicatorPosition::TopStart, color: IndicatorColor::Amber, label: Some("Beta".to_string()),
                Button { variant: ButtonVariant::Secondary, "Grafici" }
            }
            Indicator { disabled: true, label: Some("4".to_string()),
                Button { variant: ButtonVariant::Secondary, "Archivio" }
            }
        }
    }
}
const PLACEMENT_CODE: &str = r#"Group { gap: SpacingToken::Lg, align: AlignItems::Center,
    Indicator { size: IndicatorSize::Sm,
        Button { variant: ButtonVariant::Ghost, aria_label: Some("Cronologia, nuove voci".to_string()),
            Icon { kind: IconKind::Clock, size: 16 }
        }
    }
    Indicator { position: IndicatorPosition::BottomEnd, color: IndicatorColor::Emerald, size: IndicatorSize::Lg, offset: Some(6),
        Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Lg }
    }
    Indicator { position: IndicatorPosition::TopStart, color: IndicatorColor::Amber, label: Some("Beta".to_string()),
        Button { variant: ButtonVariant::Secondary, "Grafici" }
    }
    Indicator { disabled: true, label: Some("4".to_string()),
        Button { variant: ButtonVariant::Secondary, "Archivio" }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        InboxPane {}
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/inbox_pane.rs (estratto)
// MessageRow: il punto accento segnala i messaggi non letti sull'avatar del mittente.
let avatar = rsx! {
    Avatar { name: Some(m.from.to_string()), size: AvatarSize::Sm, status: Some(m.status) }
};
ListItem {
    icon: Some(rsx! {
        if m.unread {
            Indicator { size: IndicatorSize::Sm, label: None, {avatar} }
        } else {
            {avatar}
        }
    }),
    // mittente, ora, oggetto, badge e anteprima
}"#;

const PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "L'elemento a cui si ancora il segno (avatar, pulsante, icona)."),
    prop("position", "IndicatorPosition", "TopEnd", "Angolo: TopEnd, TopStart, BottomEnd, BottomStart, centrato sul vertice."),
    prop("color", "IndicatorColor", "Violet", "Violet (accento), Emerald (success), Amber (warning), Rose (danger), Cyan (info)."),
    prop("size", "IndicatorSize", "Md", "Punto da 8, 10 o 14 px; con label diventa una pillola più alta."),
    prop("label", "Option<String>", "None", "Contatore o parola breve (\"3\", \"9+\", \"Beta\")."),
    prop("processing", "bool", "false", "Onda pulsante attorno al segno, per un'attività in corso."),
    prop("disabled", "bool", "false", "Nasconde il segno senza smontare il contenuto."),
    prop("offset", "Option<i32>", "None", "Distanza in px dall'angolo, per contenuti rotondi come gli avatar."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul wrapper."),
];

pub fn story() -> Story {
    Story {
        slug: "indicator",
        name: "Indicator",
        category: Category::Dati,
        features: &["indicator"],
        icon: IconKind::Radio,
        tagline: "Punto o contatore ancorato all'angolo di un avatar o di un pulsante, per segnalare novità e stato.",
        keywords: &["indicator", "indicatore", "dot", "punto", "contatore", "badge", "non letti", "notifica"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Indicator",
            controls: vec![
                Control::choice("position", "Posizione", POSITION_OPTS, 0),
                Control::choice("color", "Colore", COLOR_OPTS, 3).emit(Emit::Always),
                Control::choice("size", "Dimensione", SIZE_OPTS, 1),
                Control::choice("target", "Elemento", TARGET_OPTS, 0).emit(Emit::Skip),
                Control::text("label", "Etichetta", "3").emit(Emit::Template("label: Some(\"{v}\".to_string())")),
                Control::number("offset", "Offset (px)", 0.0, 0.0, 12.0, 1.0, 0)
                    .emit(Emit::Template("offset: Some({v})"))
                    .hint("Utile sugli avatar: sposta il segno verso il centro."),
                Control::bool("processing", "Onda pulsante", false),
                Control::bool("disabled", "Nascosto", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some("{target}"),
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "colors",
                title: "Colori e stati",
                description: "Accento per l'attività, tinte semantiche per presenza, attesa ed errori.",
                code: COLORS_CODE,
                render: || rsx! { Colors {} },
                span: Span::Half,
            },
            Example {
                id: "placement",
                title: "Posizioni e dimensioni",
                description: "Angoli diversi, offset sull'avatar e un segno nascosto con disabled.",
                code: PLACEMENT_CODE,
                render: || rsx! { Placement {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "La casella di posta segna i messaggi non letti sull'avatar del mittente.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Indicator", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Semantic small mark (language.md §3.10)",
            depth: "Segno pieno sopra il contenuto",
            tokens: &["--sd-accent", "--sd-accent-fg", "--sd-success", "--sd-warning", "--sd-danger", "--sd-info", "--sd-surface-1", "--sd-radius-full"],
            notes: "Uno dei pochi colori pieni ammessi: è un segno piccolo. L'anello di 2px nel colore della superficie lo stacca dal contenuto.",
        }),
        a11y: &[
            "Il segno è un elemento decorativo senza ruolo e con pointer-events: none: non intercetta clic né focus.",
            "Un punto senza label non viene annunciato: scrivi lo stato anche nell'aria_label o nel testo del contenuto.",
            "La label è testo nel DOM, letto subito dopo il contenuto avvolto.",
            "L'onda di processing usa sd-animate e viene rimossa con prefers-reduced-motion.",
        ],
        related: &["badges", "avatars", "notifications", "lists"],
    }
}
