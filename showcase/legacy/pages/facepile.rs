use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Counter, FeatureBanner, PageHeader, PropRow, ShowcaseTabs, Stage};

#[component]
pub fn FacepilePage() -> Element {
    let mut selected_persona = use_signal(|| "Nessuno".to_string());
    let mut add_count = use_signal(|| 0);

    let sample_personas = vec![
        PersonaProps::new("Christian").with_role("Lead").with_status(PresenceStatus::Online),
        PersonaProps::new("Elena").with_role("Designer").with_status(PresenceStatus::DirectP2P),
        PersonaProps::new("Marco").with_role("Dev").with_status(PresenceStatus::Busy),
        PersonaProps::new("Giulia").with_role("DevOps").with_status(PresenceStatus::Offline),
        PersonaProps::new("Luca").with_role("Security").with_status(PresenceStatus::Online),
        PersonaProps::new("Sara").with_role("QA").with_status(PresenceStatus::Online),
        PersonaProps::new("Davide").with_role("Research").with_status(PresenceStatus::Offline),
    ];

    let props = vec![
        PropRow {
            name: "personas",
            r#type: "Vec<PersonaProps>",
            default: "-",
            description: "Collezione di collaboratori con avatar, presenza e ruolo",
        },
        PropRow {
            name: "max_displayable",
            r#type: "usize",
            default: "4",
            description: "Numero massimo di avatar visibili prima di compattare in +N",
        },
        PropRow {
            name: "size",
            r#type: "AvatarSize",
            default: "AvatarSize::Md",
            description: "Dimensione geometrica degli avatar (Xs, Sm, Md, Lg, Xl)",
        },
        PropRow {
            name: "on_add_click",
            r#type: "Option<EventHandler<()>>",
            default: "None",
            description: "Mostra pulsante circolare tratteggiato + per invitare collaboratori",
        },
        PropRow {
            name: "on_persona_click",
            r#type: "Option<EventHandler<PersonaProps>>",
            default: "None",
            description: "Callback scatenata al click su un avatar o dal menu a tendina",
        },
    ];

    let code_snippet = r#"Facepile {
    personas: sample_personas,
    max_displayable: 4,
    size: AvatarSize::Md,
    show_presence: true,
    on_add_click: move |_| println!("Aggiungi membro"),
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Facepile".to_string(),
                description: "Cluster compatto di avatar di collaboratori in stile Microsoft Fluent UI con disposizione sovrapposta, badge di presenza e menu di overflow (+N altri).".to_string(),
            }

            FeatureBanner { feature_name: "facepile" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage { start: true,
                        Facepile {
                            personas: sample_personas,
                            max_displayable: 4,
                            size: AvatarSize::Md,
                            show_presence: true,
                            on_add_click: move |_| add_count += 1,
                            on_persona_click: move |p: PersonaProps| {
                                selected_persona.set(format!("{} ({})", p.name, p.role.unwrap_or_default()));
                            },
                        }
                        Counter { label: "Collaboratore selezionato".to_string(), value: selected_persona() }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
