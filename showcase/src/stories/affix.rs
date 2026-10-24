//! Story: Affix and BackToTop (feature `affix`). Both are fixed to a corner of the window, so the
//! stage only explains where to look; the live element appears in the corner after scrolling.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const POSITIONS: [AffixPosition; 4] =
    [AffixPosition::BottomRight, AffixPosition::BottomLeft, AffixPosition::TopRight, AffixPosition::TopLeft];
const POSITION_OPTS: &[ChoiceOption] = &[
    opt("BottomRight", "AffixPosition::BottomRight"),
    opt("BottomLeft", "AffixPosition::BottomLeft"),
    opt("TopRight", "AffixPosition::TopRight"),
    opt("TopLeft", "AffixPosition::TopLeft"),
];
const CORNERS: [&str; 4] = ["in basso a destra", "in basso a sinistra", "in alto a destra", "in alto a sinistra"];

fn render(k: &Knobs) -> Element {
    let threshold = k.number("scroll_threshold") as u32;
    let label = k.text("label");
    let corner = CORNERS[k.index("position").min(3)];
    // Affix reads the threshold once when it mounts: remount on change.
    let remount = format!("{}-{threshold}", k.index("position"));
    rsx! {
        Alert { variant: AlertVariant::Info, title: Some("Guarda l'angolo della finestra".to_string()),
            "Il pulsante compare {corner} dopo {threshold} px di scroll e riporta la pagina in cima."
        }
        for key in [remount] {
            BackToTop {
                key: "{key}",
                position: k.pick("position", &POSITIONS),
                scroll_threshold: threshold,
                label: (!label.is_empty()).then_some(label.clone()),
            }
        }
    }
}

#[component]
fn InContext() -> Element {
    let mut dirty = use_signal(|| false);
    let mut toast = use_notification();
    rsx! {
        Card { title: Some("Configurazione di edge-07".to_string()), subtitle: Some("Milano · runtime 0.5".to_string()),
            Stack { gap: SpacingToken::Md,
                Switch { checked: dirty, label: Some("Simula modifiche non salvate".to_string()) }
                Text { size: TextSize::Sm, dimmed: true, "La barra resta in basso a sinistra della finestra finché salvi o annulli, anche scorrendo." }
            }
        }
        if dirty() {
            Affix { position: AffixPosition::BottomLeft,
                Card { variant: CardVariant::Elevated, shadow: CardShadow::Lg,
                    Group { gap: SpacingToken::Md,
                        Text { size: TextSize::Sm, "3 modifiche non salvate" }
                        Group { gap: SpacingToken::Sm,
                            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| dirty.set(false), "Annulla" }
                            Button {
                                size: ButtonSize::Sm,
                                onclick: move |_| {
                                    dirty.set(false);
                                    toast.success("Configurazione salvata");
                                },
                                "Salva"
                            }
                        }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut dirty = use_signal(|| false);
let mut toast = use_notification();

Card { title: Some("Configurazione di edge-07".to_string()), subtitle: Some("Milano · runtime 0.5".to_string()),
    Stack { gap: SpacingToken::Md,
        Switch { checked: dirty, label: Some("Simula modifiche non salvate".to_string()) }
        Text { size: TextSize::Sm, dimmed: true, "La barra resta in basso a sinistra della finestra finché salvi o annulli, anche scorrendo." }
    }
}
if dirty() {
    Affix { position: AffixPosition::BottomLeft,
        Card { variant: CardVariant::Elevated, shadow: CardShadow::Lg,
            Group { gap: SpacingToken::Md,
                Text { size: TextSize::Sm, "3 modifiche non salvate" }
                Group { gap: SpacingToken::Sm,
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| dirty.set(false), "Annulla" }
                    Button {
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            dirty.set(false);
                            toast.success("Configurazione salvata");
                        },
                        "Salva"
                    }
                }
            }
        }
    }
}"#;

const AFFIX_PROPS: &[PropDoc] = &[
    prop("position", "AffixPosition", "BottomRight", "Angolo della finestra: BottomRight, BottomLeft, TopRight, TopLeft."),
    prop("scroll_threshold", "Option<u32>", "None", "Px di scroll oltre cui compare; None lo mostra sempre."),
    prop("z_index", "u32", "60", "Ignorata: il livello è sempre z('affix')."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
    prop("children", "Element", "-", "Contenuto fissato: barra azioni, pulsante, avviso."),
];
const BACK_PROPS: &[PropDoc] = &[
    prop("scroll_threshold", "u32", "200", "Px di scroll oltre cui compare."),
    prop("label", "Option<String>", "None", "Testo accanto all'icona; senza, pulsante tondo a sola icona."),
    prop("icon", "IconKind", "ChevronUp", "Icona del pulsante."),
    prop("position", "AffixPosition", "BottomRight", "Angolo della finestra."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];

pub fn story() -> Story {
    Story {
        slug: "affix",
        name: "Affix",
        category: Category::Overlay,
        features: &["affix"],
        icon: IconKind::ChevronUp,
        tagline: "Fissa un elemento a un angolo della finestra, anche solo dopo un certo scroll, come il ritorno in cima.",
        keywords: &["affix", "back to top", "torna su", "sticky", "fisso", "scroll"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "BackToTop",
            controls: vec![
                Control::choice("position", "Angolo", POSITION_OPTS, 0),
                Control::number("scroll_threshold", "Soglia di scroll (px)", 200.0, 0.0, 1200.0, 50.0, 0),
                Control::text("label", "Etichetta", "Torna su").emit(Emit::Template("label: Some(\"{v}\".to_string())")),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: None,
            stage: Stage::Fill,
        }),
        examples: vec![Example {
            id: "context",
            title: "In contesto",
            description: "Barra delle modifiche non salvate fissata in basso a sinistra.",
            code: IN_CONTEXT_CODE,
            render: || rsx! { InContext {} },
            span: Span::Full,
        }],
        props: vec![
            PropGroup { component: "Affix", rows: AFFIX_PROPS },
            PropGroup { component: "BackToTop", rows: BACK_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Raised control (language.md §3.5)",
            depth: "Lit control (secondary)",
            tokens: &["--sd-surface-3", "--sd-surface-2", "--sd-border-strong", "--sd-control-md", "--sd-radius-full", "--sd-space-6"],
            notes: "BackToTop è un pill secondario a --sd-space-6 dal bordo; Affix aggiunge solo posizione, livello e dissolvenza.",
        }),
        a11y: &[
            "BackToTop è un <button> nativo con title da Locale (back_to_top).",
            "Senza label il pulsante mostra solo l'icona: il nome accessibile resta il title, quindi preferisci una label visibile.",
            "Sotto la soglia l'elemento è trasparente e senza pointer-events, così non intercetta i clic.",
            "Il livello è z('affix'): sopra i contenuti, sotto drawer, modal e notifiche, che restano sempre raggiungibili.",
        ],
        related: &["speed-dial", "notifications", "scroll-area", "reactive-hooks"],
    }
}
