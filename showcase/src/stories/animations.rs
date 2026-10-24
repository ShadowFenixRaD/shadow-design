//! Story: Transition, AnimationVariant and Animations (feature `animation`).
//! Examples replay by bumping the key of a one-item keyed list (BATCH_GUIDE §7).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;
use crate::story::sections::DemoCell;

const VARIANTS: [AnimationVariant; 14] = [
    AnimationVariant::FadeIn,
    AnimationVariant::SlideInBottom,
    AnimationVariant::SlideInTop,
    AnimationVariant::SlideInLeft,
    AnimationVariant::SlideInRight,
    AnimationVariant::ScaleIn,
    AnimationVariant::ModalEnter,
    AnimationVariant::DropdownEnter,
    AnimationVariant::ToastEnter,
    AnimationVariant::DrawerSlideLeft,
    AnimationVariant::DrawerSlideRight,
    AnimationVariant::PulseGlow,
    AnimationVariant::FadeOut,
    AnimationVariant::ToastExit,
];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("FadeIn", "AnimationVariant::FadeIn"),
    opt("SlideInBottom", "AnimationVariant::SlideInBottom"),
    opt("SlideInTop", "AnimationVariant::SlideInTop"),
    opt("SlideInLeft", "AnimationVariant::SlideInLeft"),
    opt("SlideInRight", "AnimationVariant::SlideInRight"),
    opt("ScaleIn", "AnimationVariant::ScaleIn"),
    opt("ModalEnter", "AnimationVariant::ModalEnter"),
    opt("DropdownEnter", "AnimationVariant::DropdownEnter"),
    opt("ToastEnter", "AnimationVariant::ToastEnter"),
    opt("DrawerSlideLeft", "AnimationVariant::DrawerSlideLeft"),
    opt("DrawerSlideRight", "AnimationVariant::DrawerSlideRight"),
    opt("PulseGlow", "AnimationVariant::PulseGlow"),
    opt("FadeOut", "AnimationVariant::FadeOut"),
    opt("ToastExit", "AnimationVariant::ToastExit"),
];

const CARD_CHILDREN: &str = r#"Card {
    title: Some("Nodo edge-07".to_string()),
    subtitle: Some("Francoforte · 12 ms".to_string()),
    Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
}"#;

/// Stateful demo: the "Riproduci" button bumps the key so the animation runs again.
#[component]
fn TransitionDemo(knobs: Knobs) -> Element {
    let mut replay = use_signal(|| 0usize);
    let variant = knobs.pick("variant", &VARIANTS);
    let run = format!("{}-{}", replay(), knobs.index("variant"));
    rsx! {
        Stack { gap: SpacingToken::Lg, align: AlignItems::Center,
            for run in std::iter::once(run) {
                Transition { key: "{run}", variant,
                    Card {
                        title: Some("Nodo edge-07".to_string()),
                        subtitle: Some("Francoforte · 12 ms".to_string()),
                        Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
                    }
                }
            }
            Button {
                variant: ButtonVariant::Secondary,
                size: ButtonSize::Sm,
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Sparkles, size: 14 }
                }),
                onclick: move |_| replay += 1,
                "Riproduci"
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        TransitionDemo { knobs: k.clone() }
    }
}

#[component]
fn Entrances() -> Element {
    let mut replay = use_signal(|| 0usize);
    rsx! {
        Stack { gap: SpacingToken::Md,
            for run in std::iter::once(replay()) {
                SimpleGrid { key: "{run}", cols: 2, spacing: GridSpacing::Sm,
                    Transition { variant: AnimationVariant::FadeIn,
                        DemoCell { label: Some("FadeIn".to_string()) }
                    }
                    Transition { variant: AnimationVariant::ScaleIn,
                        DemoCell { label: Some("ScaleIn".to_string()) }
                    }
                    Transition { variant: AnimationVariant::SlideInBottom,
                        DemoCell { label: Some("SlideInBottom".to_string()) }
                    }
                    Transition { variant: AnimationVariant::SlideInRight,
                        DemoCell { label: Some("SlideInRight".to_string()) }
                    }
                }
            }
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| replay += 1, "Riproduci" }
        }
    }
}
const ENTRANCES_CODE: &str = r#"let mut replay = use_signal(|| 0usize);

Stack { gap: SpacingToken::Md,
    for run in std::iter::once(replay()) {
        SimpleGrid { key: "{run}", cols: 2, spacing: GridSpacing::Sm,
            Transition { variant: AnimationVariant::FadeIn,
                DemoCell { label: Some("FadeIn".to_string()) }
            }
            Transition { variant: AnimationVariant::ScaleIn,
                DemoCell { label: Some("ScaleIn".to_string()) }
            }
            Transition { variant: AnimationVariant::SlideInBottom,
                DemoCell { label: Some("SlideInBottom".to_string()) }
            }
            Transition { variant: AnimationVariant::SlideInRight,
                DemoCell { label: Some("SlideInRight".to_string()) }
            }
        }
    }
    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| replay += 1, "Riproduci" }
}"#;

#[component]
fn FloatingLayers() -> Element {
    let mut replay = use_signal(|| 0usize);
    rsx! {
        Stack { gap: SpacingToken::Md,
            for run in std::iter::once(replay()) {
                Stack { key: "{run}", gap: SpacingToken::Sm,
                    Transition { variant: AnimationVariant::ModalEnter,
                        Card { title: Some("Scollegare edge-07?".to_string()),
                            Text { size: TextSize::Sm, dimmed: true, "I 4 peer passano su core-02." }
                        }
                    }
                    Transition { variant: AnimationVariant::ToastEnter,
                        Alert { variant: AlertVariant::Success, title: Some("Configurazione salvata".to_string()),
                            "Distribuita su 24 nodi."
                        }
                    }
                }
            }
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| replay += 1, "Riproduci" }
        }
    }
}
const FLOATING_LAYERS_CODE: &str = r#"let mut replay = use_signal(|| 0usize);

Stack { gap: SpacingToken::Md,
    for run in std::iter::once(replay()) {
        Stack { key: "{run}", gap: SpacingToken::Sm,
            Transition { variant: AnimationVariant::ModalEnter,
                Card { title: Some("Scollegare edge-07?".to_string()),
                    Text { size: TextSize::Sm, dimmed: true, "I 4 peer passano su core-02." }
                }
            }
            Transition { variant: AnimationVariant::ToastEnter,
                Alert { variant: AlertVariant::Success, title: Some("Configurazione salvata".to_string()),
                    "Distribuita su 24 nodi."
                }
            }
        }
    }
    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| replay += 1, "Riproduci" }
}"#;

#[component]
fn Loops() -> Element {
    rsx! {
        Group { gap: SpacingToken::Lg, align: AlignItems::Center,
            Transition { variant: AnimationVariant::PulseGlow,
                Badge { label: "Sincronizzazione".to_string(), variant: BadgeVariant::Primary, dot: true }
            }
            Group { gap: SpacingToken::Sm, align: AlignItems::Center,
                Transition { variant: AnimationVariant::Spin, class: "inline-flex",
                    Icon { kind: IconKind::Spinner, size: 18 }
                }
                Text { size: TextSize::Sm, dimmed: true, "Rotazione chiavi in corso" }
            }
        }
    }
}
const LOOPS_CODE: &str = r#"Group { gap: SpacingToken::Lg, align: AlignItems::Center,
    Transition { variant: AnimationVariant::PulseGlow,
        Badge { label: "Sincronizzazione".to_string(), variant: BadgeVariant::Primary, dot: true }
    }
    Group { gap: SpacingToken::Sm, align: AlignItems::Center,
        Transition { variant: AnimationVariant::Spin, class: "inline-flex",
            Icon { kind: IconKind::Spinner, size: 18 }
        }
        Text { size: TextSize::Sm, dimmed: true, "Rotazione chiavi in corso" }
    }
}"#;

#[component]
fn ClassConstants() -> Element {
    let mut replay = use_signal(|| 0usize);
    rsx! {
        Stack { gap: SpacingToken::Md,
            for run in std::iter::once(replay()) {
                Card {
                    key: "{run}",
                    class: Animations::SLIDE_IN_TOP.to_string(),
                    title: Some("Nuovo commit".to_string()),
                    subtitle: Some("main · 7 minuti fa".to_string()),
                    Text { size: TextSize::Sm, dimmed: true, "La classe va sul componente, senza contenitore in più." }
                }
            }
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| replay += 1, "Riproduci" }
        }
    }
}
const CLASS_CONSTANTS_CODE: &str = r#"let mut replay = use_signal(|| 0usize);

Stack { gap: SpacingToken::Md,
    for run in std::iter::once(replay()) {
        Card {
            key: "{run}",
            class: Animations::SLIDE_IN_TOP.to_string(),
            title: Some("Nuovo commit".to_string()),
            subtitle: Some("main · 7 minuti fa".to_string()),
            Text { size: TextSize::Sm, dimmed: true, "La classe va sul componente, senza contenitore in più." }
        }
    }
    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| replay += 1, "Riproduci" }
}"#;

const TRANSITION_PROPS: &[PropDoc] = &[
    prop("variant", "AnimationVariant", "FadeIn", "Animazione applicata al contenitore <div>."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore (es. inline-flex per Spin)."),
    prop("children", "Element", "-", "Contenuto animato."),
];
const VARIANT_PROPS: &[PropDoc] = &[
    prop("FadeIn / FadeOut", "AnimationVariant", "-", "Opacità in entrata o in uscita (l'uscita resta invisibile)."),
    prop("SlideInTop / Bottom / Left / Right", "AnimationVariant", "-", "Scorrimento breve dal lato indicato."),
    prop("ScaleIn / ModalEnter / DropdownEnter", "AnimationVariant", "-", "Pop-in dei livelli flottanti (sd-pop-in)."),
    prop("ToastEnter / ToastExit", "AnimationVariant", "-", "Entrata dal basso e dissolvenza dei toast."),
    prop("DrawerSlideLeft / DrawerSlideRight", "AnimationVariant", "-", "Entrata laterale dei drawer."),
    prop("PulseGlow", "AnimationVariant", "-", "Pulsazione decorativa infinita."),
    prop("Spin / Indeterminate", "AnimationVariant", "-", "Cicli essenziali: rallentano con reduced motion."),
    prop("TactilePress", "AnimationVariant", "-", "Pressione a scala 0.97 su :active."),
    prop(".classes()", "fn -> &'static str", "-", "Classe CSS della variante, per usarla senza Transition."),
];
const CONST_PROPS: &[PropDoc] = &[
    prop("Animations::FADE_IN … INDETERMINATE", "&'static str", "-", "Le stesse classi come costanti, da passare a class."),
    prop("Animations::TACTILE", "&'static str", "\"tactile-press\"", "Press tattile per elementi cliccabili personalizzati."),
];

pub fn story() -> Story {
    Story {
        slug: "animations",
        name: "Transition",
        category: Category::Fondamenta,
        features: &["animation"],
        icon: IconKind::Sparkles,
        tagline: "Micro-animazioni di ingresso, uscita e attesa che rispettano prefers-reduced-motion.",
        keywords: &["animation", "animazione", "transition", "transizione", "motion", "fade", "slide", "reduced motion"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Transition",
            controls: vec![Control::choice("variant", "Animazione", VARIANT_OPTS, 0)
                .hint("Riproduci per rivedere l'animazione.")],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(CARD_CHILDREN),
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "entrances",
                title: "Ingressi",
                description: "Quattro entrate a confronto, riprodotte insieme.",
                code: ENTRANCES_CODE,
                render: || rsx! { Entrances {} },
                span: Span::Half,
            },
            Example {
                id: "floating",
                title: "Livelli flottanti",
                description: "ModalEnter per i dialoghi, ToastEnter per le conferme.",
                code: FLOATING_LAYERS_CODE,
                render: || rsx! { FloatingLayers {} },
                span: Span::Half,
            },
            Example {
                id: "loops",
                title: "Cicli",
                description: "PulseGlow decorativo e Spin essenziale.",
                code: LOOPS_CODE,
                render: || rsx! { Loops {} },
                span: Span::Half,
            },
            Example {
                id: "constants",
                title: "Costanti Animations",
                description: "La classe direttamente sul componente, senza Transition.",
                code: CLASS_CONSTANTS_CODE,
                render: || rsx! { ClassConstants {} },
                span: Span::Half,
            },
        ],
        props: vec![
            PropGroup { component: "Transition", rows: TRANSITION_PROPS },
            PropGroup { component: "AnimationVariant", rows: VARIANT_PROPS },
            PropGroup { component: "Animations", rows: CONST_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Motion (language.md §5): sd-animate e sd-animate-essential",
            depth: "Nessuna: anima il contenuto senza cambiarne il livello",
            tokens: &["--sd-dur", "--sd-dur-fast", "--sd-dur-slow", "--sd-ease"],
            notes: "Keyframes sd-* da _animations.scss; gli ingressi durano --sd-dur, i livelli flottanti usano sd-pop-in.",
        }),
        a11y: &[
            "Con prefers-reduced-motion: reduce le animazioni decorative non partono: il contenuto appare subito.",
            "Spin e Indeterminate restano attivi ma tre volte più lenti, perché comunicano attesa.",
            "Transition è un <div> senza ruolo: non cambia la semantica né l'ordine del focus.",
            "FadeOut e ToastExit nascondono solo visivamente: rimuovi l'elemento dal DOM quando non serve più.",
        ],
        related: &["notifications", "modals", "drawer", "skeletons", "progress"],
    }
}
