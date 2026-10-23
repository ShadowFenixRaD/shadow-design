use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn DocumentCardPage() -> Element {
    let author = PersonaProps::new("Christian Torbidone")
        .with_role("Lead Architect")
        .with_status(PresenceStatus::Online);

    let props = vec![
        PropRow {
            name: "file_name",
            r#type: "String",
            default: "-",
            description: "Nome completo del documento o file visualizzato nel titolo",
        },
        PropRow {
            name: "file_type",
            r#type: "String",
            default: "-",
            description: "Estensione o formato (es. PDF, DOCX, RS, XLSX, ZIP)",
        },
        PropRow {
            name: "preview_url",
            r#type: "Option<String>",
            default: "None",
            description: "URL opzionale per la thumbnail di anteprima visuale",
        },
        PropRow {
            name: "author",
            r#type: "PersonaProps",
            default: "-",
            description: "Dati dell'autore (nome, avatar, stato presenza, ruolo)",
        },
        PropRow {
            name: "modified_at",
            r#type: "String",
            default: "-",
            description: "Timestamp relativo o data dell'ultima modifica",
        },
    ];

    let code_snippet = r#"DocumentCard {
    file_name: "Specifiche-Architettura.pdf".to_string(),
    file_type: "PDF".to_string(),
    file_size: Some("4.2 MB".to_string()),
    author: PersonaProps::new("Christian").with_status(PresenceStatus::Online),
    modified_at: "2 ore fa".to_string(),
    views: Some(142),
    on_download: move |_| println!("Download!"),
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "DocumentCard" }
                Text { dimmed: true, size: TextSize::Sm, "Scheda ricca per la visualizzazione di documenti, allegati e file con thumbnail preview, badge cromatici di formato e barra azioni rapide in hover." }
            }

            FeatureBanner { feature_name: "document-card" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 sm:grid-cols-2 gap-6 w-full p-4",
                        DocumentCard {
                            file_name: "Architettura-Shadow-Design-v2.pdf".to_string(),
                            file_type: "PDF".to_string(),
                            file_size: Some("3.8 MB".to_string()),
                            author: author.clone(),
                            modified_at: "Oggi, 11:20".to_string(),
                            views: Some(254),
                            on_download: move |_| {},
                            on_share: move |_| {},
                            on_delete: move |_| {},
                        }

                        DocumentCard {
                            file_name: "Kernel-Engine-Mesh.rs".to_string(),
                            file_type: "RS".to_string(),
                            file_size: Some("48 KB".to_string()),
                            author: PersonaProps::new("Shadowfenix").with_status(PresenceStatus::DirectP2P),
                            modified_at: "Ieri".to_string(),
                            views: Some(89),
                            on_download: move |_| {},
                            on_share: move |_| {},
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
