//! Story: Tooltip (feature `tooltip`). Stateless: the playground renders it straight from the knobs.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::OpsDashboard;
use crate::story::schema::*;

const POSITIONS: [TooltipPosition; 4] =
    [TooltipPosition::Top, TooltipPosition::Bottom, TooltipPosition::Left, TooltipPosition::Right];
const POSITION_OPTS: &[ChoiceOption] = &[
    opt("Top", "TooltipPosition::Top"),
    opt("Bottom", "TooltipPosition::Bottom"),
    opt("Left", "TooltipPosition::Left"),
    opt("Right", "TooltipPosition::Right"),
];

const CHILDREN_CODE: &str = r#"Button { variant: ButtonVariant::Ghost, aria_label: Some("Copia l'indirizzo".to_string()),
    Icon { kind: IconKind::Copy, size: 16 }
}"#;

fn render(k: &Knobs) -> Element {
    rsx! {
        Tooltip {
            content: k.text("content"),
            position: k.pick("position", &POSITIONS),
            opened: k.bool("opened").then_some(true),
            glow: k.bool("glow").then_some(true),
            Button { variant: ButtonVariant::Ghost, aria_label: Some("Copia l'indirizzo".to_string()),
                Icon { kind: IconKind::Copy, size: 16 }
            }
        }
    }
}

#[component]
fn Directions() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::Center,
            Tooltip { content: "Sopra".to_string(), position: TooltipPosition::Top,
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Top" }
            }
            Tooltip { content: "Sotto".to_string(), position: TooltipPosition::Bottom,
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Bottom" }
            }
            Tooltip { content: "A sinistra".to_string(), position: TooltipPosition::Left,
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Left" }
            }
            Tooltip { content: "A destra".to_string(), position: TooltipPosition::Right,
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Right" }
            }
        }
    }
}
const DIRECTIONS_CODE: &str = r#"Group { gap: SpacingToken::Sm, justify: JustifyContent::Center,
    Tooltip { content: "Sopra".to_string(), position: TooltipPosition::Top,
        Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Top" }
    }
    Tooltip { content: "Sotto".to_string(), position: TooltipPosition::Bottom,
        Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Bottom" }
    }
    Tooltip { content: "A sinistra".to_string(), position: TooltipPosition::Left,
        Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Left" }
    }
    Tooltip { content: "A destra".to_string(), position: TooltipPosition::Right,
        Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Right" }
    }
}"#;

#[component]
fn Toolbar() -> Element {
    rsx! {
        Group { gap: SpacingToken::Xs, justify: JustifyContent::Center,
            Tooltip { content: "Scarica i log".to_string(),
                Button { variant: ButtonVariant::Ghost, aria_label: Some("Scarica i log".to_string()),
                    Icon { kind: IconKind::Download, size: 16 }
                }
            }
            Tooltip { content: "Condividi il link".to_string(),
                Button { variant: ButtonVariant::Ghost, aria_label: Some("Condividi il link".to_string()),
                    Icon { kind: IconKind::Share2, size: 16 }
                }
            }
            Tooltip { content: "Apri nel terminale".to_string(),
                Button { variant: ButtonVariant::Ghost, aria_label: Some("Apri nel terminale".to_string()),
                    Icon { kind: IconKind::Terminal, size: 16 }
                }
            }
            Tooltip { content: "Elimina il deploy".to_string(),
                Button { variant: ButtonVariant::Ghost, aria_label: Some("Elimina il deploy".to_string()),
                    Icon { kind: IconKind::Trash2, size: 16 }
                }
            }
        }
    }
}
const TOOLBAR_CODE: &str = r#"Group { gap: SpacingToken::Xs, justify: JustifyContent::Center,
    Tooltip { content: "Scarica i log".to_string(),
        Button { variant: ButtonVariant::Ghost, aria_label: Some("Scarica i log".to_string()),
            Icon { kind: IconKind::Download, size: 16 }
        }
    }
    Tooltip { content: "Condividi il link".to_string(),
        Button { variant: ButtonVariant::Ghost, aria_label: Some("Condividi il link".to_string()),
            Icon { kind: IconKind::Share2, size: 16 }
        }
    }
    Tooltip { content: "Apri nel terminale".to_string(),
        Button { variant: ButtonVariant::Ghost, aria_label: Some("Apri nel terminale".to_string()),
            Icon { kind: IconKind::Terminal, size: 16 }
        }
    }
    Tooltip { content: "Elimina il deploy".to_string(),
        Button { variant: ButtonVariant::Ghost, aria_label: Some("Elimina il deploy".to_string()),
            Icon { kind: IconKind::Trash2, size: 16 }
        }
    }
}"#;

#[component]
fn Controlled() -> Element {
    let show = use_signal(|| true);
    rsx! {
        Stack { gap: SpacingToken::Lg, align: AlignItems::Center,
            Tooltip { content: "Nuovo: esporta in CSV".to_string(), position: TooltipPosition::Bottom, opened: Some(show()),
                Button { variant: ButtonVariant::Secondary, "Esporta" }
            }
            Switch { checked: show, label: Some("Mostra il suggerimento".to_string()) }
        }
    }
}
const CONTROLLED_CODE: &str = r#"let show = use_signal(|| true);

Stack { gap: SpacingToken::Lg, align: AlignItems::Center,
    Tooltip { content: "Nuovo: esporta in CSV".to_string(), position: TooltipPosition::Bottom, opened: Some(show()),
        Button { variant: ButtonVariant::Secondary, "Esporta" }
    }
    Switch { checked: show, label: Some("Mostra il suggerimento".to_string()) }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        OpsDashboard {}
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/ops_dashboard.rs (estratto): il tooltip dice chi è l'avatar in testata.
Tooltip { content: "Giulia Ferri · in linea".to_string(), position: TooltipPosition::Bottom,
    Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm, status: Some(PresenceStatus::Online) }
}"#;

const PROPS: &[PropDoc] = &[
    prop("content", "String", "-", "Testo breve del suggerimento, senza markup."),
    prop("position", "TooltipPosition", "Top", "Top, Bottom, Left, Right."),
    prop("opened", "Option<bool>", "None", "Controllo esterno: Some(true) lo tiene aperto; None segue hover e focus."),
    prop("glow", "Option<bool>", "None", "Glow opzionale sulla bolla."),
    prop("id", "Option<String>", "None", "Id della bolla; generato se assente."),
    prop("describes_id", "Option<String>", "None", "Id da usare in aria-describedby al posto di quello della bolla."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul wrapper."),
    prop("children", "Element", "-", "Elemento descritto, di solito un pulsante."),
];

pub fn story() -> Story {
    Story {
        slug: "tooltip",
        name: "Tooltip",
        category: Category::Overlay,
        features: &["tooltip"],
        icon: IconKind::Info,
        tagline: "Etichetta breve che descrive un controllo al passaggio del puntatore o al focus.",
        keywords: &["tooltip", "suggerimento", "hint", "hover", "etichetta", "icon button"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Tooltip",
            controls: vec![
                Control::text("content", "Testo", "Copia l'indirizzo").emit(Emit::Template("content: \"{v}\".to_string()")),
                Control::choice("position", "Posizione", POSITION_OPTS, 0),
                Control::bool("opened", "Sempre aperto", false).emit(Emit::Template("opened: Some(true)")),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(CHILDREN_CODE),
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "directions",
                title: "Quattro direzioni",
                description: "La bolla si apre sul lato scelto.",
                code: DIRECTIONS_CODE,
                render: || rsx! { Directions {} },
                span: Span::Half,
            },
            Example {
                id: "toolbar",
                title: "Toolbar di icone",
                description: "Il caso tipico: pulsanti ghost a sola icona.",
                code: TOOLBAR_CODE,
                render: || rsx! { Toolbar {} },
                span: Span::Half,
            },
            Example {
                id: "controlled",
                title: "Controllato",
                description: "opened lo mostra da codice, per esempio per annunciare una novità.",
                code: CONTROLLED_CODE,
                render: || rsx! { Controlled {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il presidio in testata alla dashboard operativa.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Tooltip", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Floating panel (language.md §3.2)",
            depth: "Raised",
            tokens: &["--sd-surface-2", "--sd-border-strong", "--sd-elev-2", "--sd-radius-md", "--sd-text-xs", "--sd-dur-fast"],
            notes: "Unico pannello flottante con --sd-radius-md; nessuna interazione al suo interno.",
        }),
        a11y: &[
            "La bolla ha role=tooltip e il trigger la referenzia con aria-describedby.",
            "Si apre al passaggio del puntatore e al focus da tastiera, si chiude all'uscita o alla perdita del focus.",
            "Esc la nasconde senza spostare il focus.",
            "Non sostituisce l'etichetta: un pulsante a sola icona ha comunque bisogno di aria_label.",
        ],
        related: &["popover", "hover-card", "keytip", "buttons"],
    }
}
