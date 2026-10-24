//! Scene: node settings form (REVAMP §1.3). Fields, selects, toggles, a slider and one primary action.
use dioxus::prelude::*;
use shadow_design::prelude::*;

fn regions() -> Vec<SelectItem> {
    vec![
        SelectItem::new("mil", "Milano").with_group("Europa"),
        SelectItem::new("fra", "Francoforte").with_group("Europa"),
        SelectItem::new("ams", "Amsterdam").with_group("Europa"),
        SelectItem::new("nyc", "New York").with_group("America"),
        SelectItem::new("sin", "Singapore").with_group("Asia"),
    ]
}

fn protocols() -> Vec<SelectItem> {
    vec![
        SelectItem::new("quic", "QUIC"),
        SelectItem::new("webrtc", "WebRTC"),
        SelectItem::new("tcp", "TCP"),
        SelectItem::new("ws", "WebSocket"),
    ]
}

#[component]
pub fn SettingsForm(#[props(default)] compact: bool) -> Element {
    let name = use_signal(|| "edge-07".to_string());
    let region = use_signal(|| Some("mil".to_string()));
    let protocols_on = use_signal(|| vec!["quic".to_string(), "webrtc".to_string()]);
    let cipher = use_signal(|| "kyber1024".to_string());
    let bandwidth = use_signal(|| 64.0);
    let relay = use_signal(|| true);
    let telemetry = use_signal(|| false);
    // A field value, not styling: start from the Cyan preset hex (no colour literal in the showcase).
    let label_color = use_signal(|| PrimaryColor::Cyan.hex().to_string());
    let mut toast = use_notification();
    let grid = if compact { "sc-settings" } else { "sc-settings sc-settings--split" };

    rsx! {
        div { class: "sc-scene-body",
            div { class: "sc-scene__head",
                div { class: "sc-scene__title-group",
                    span { class: "sc-eyebrow", "Impostazioni" }
                    span { class: "sc-scene__title", "Nodo edge-07" }
                }
            }
            div { class: "{grid}",
                Fieldset { legend: "Identità".to_string(), description: Some("Come il nodo appare nella mesh.".to_string()),
                    Stack { gap: SpacingToken::Md,
                        TextInput { value: name, label: Some("Nome del nodo".to_string()), clearable: true }
                        Select {
                            data: regions(),
                            value: region,
                            label: Some("Regione".to_string()),
                            searchable: true,
                        }
                        ColorInput {
                            value: label_color,
                            label: Some("Colore etichetta".to_string()),
                            description: Some("Usato nelle mappe di rete.".to_string()),
                        }
                    }
                }
                Fieldset { legend: "Rete".to_string(), description: Some("Trasporti e cifratura dei peer.".to_string()),
                    Stack { gap: SpacingToken::Md,
                        MultiSelect {
                            data: protocols(),
                            values: protocols_on,
                            label: Some("Protocolli".to_string()),
                            clearable: true,
                        }
                        RadioGroup { value: cipher, label: Some("Cifratura".to_string()), orientation: RadioOrientation::Horizontal,
                            Radio { value: "kyber768", checked: Some(cipher), label: Some("Kyber-768".to_string()) }
                            Radio { value: "kyber1024", checked: Some(cipher), label: Some("Kyber-1024".to_string()) }
                        }
                        Slider { value: bandwidth, min: 8.0, max: 256.0, step: 8.0, label: Some("Banda massima (Mbit/s)".to_string()) }
                        Switch {
                            checked: relay,
                            label: Some("Relay automatico".to_string()),
                            description: Some("Inoltra il traffico dei peer dietro NAT.".to_string()),
                        }
                        Checkbox { checked: telemetry, label: Some("Invia telemetria anonima".to_string()) }
                    }
                }
            }
            Alert { variant: AlertVariant::Info, title: Some("Riavvio non necessario".to_string()),
                "Le modifiche si applicano ai nuovi peer; le sessioni aperte restano attive."
            }
            Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                Button { variant: ButtonVariant::Ghost, "Annulla" }
                Button {
                    left_icon: Some(rsx! {
                        Icon { kind: IconKind::Check, size: 15 }
                    }),
                    onclick: move |_| {
                        toast.success("Impostazioni salvate");
                    },
                    "Salva impostazioni"
                }
            }
        }
    }
}
