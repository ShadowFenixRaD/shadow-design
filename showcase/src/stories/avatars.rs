//! Story: Avatar and AvatarGroup (feature `avatar`). Reference story pattern for a simple component.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const PHOTO: &str = "https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?auto=format&fit=crop&w=160&q=80";

const SIZES: [AvatarSize; 5] = [AvatarSize::Xs, AvatarSize::Sm, AvatarSize::Md, AvatarSize::Lg, AvatarSize::Xl];
const SIZE_OPTS: &[ChoiceOption] = &[
    opt("Xs", "AvatarSize::Xs"),
    opt("Sm", "AvatarSize::Sm"),
    opt("Md", "AvatarSize::Md"),
    opt("Lg", "AvatarSize::Lg"),
    opt("Xl", "AvatarSize::Xl"),
];
const STATUSES: [Option<PresenceStatus>; 5] = [
    Some(PresenceStatus::Online),
    Some(PresenceStatus::Busy),
    Some(PresenceStatus::Offline),
    Some(PresenceStatus::DirectP2P),
    None,
];
const STATUS_OPTS: &[ChoiceOption] = &[
    opt("Online", "Some(PresenceStatus::Online)"),
    opt("Busy", "Some(PresenceStatus::Busy)"),
    opt("Offline", "Some(PresenceStatus::Offline)"),
    opt("Direct P2P", "Some(PresenceStatus::DirectP2P)"),
    opt("Nessuno", "None"),
];

fn render(k: &Knobs) -> Element {
    rsx! {
        Avatar {
            name: Some(k.text("name")).filter(|s| !s.is_empty()),
            size: k.pick("size", &SIZES),
            status: k.pick("status", &STATUSES),
            src: k.bool("src").then(|| PHOTO.to_string()),
        }
    }
}

#[component]
fn Sizes() -> Element {
    rsx! {
        Group { gap: SpacingToken::Md, align: AlignItems::Center,
            Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Xs }
            Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm }
            Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Md }
            Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Lg }
            Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Xl }
        }
    }
}
const SIZES_CODE: &str = r#"Group { gap: SpacingToken::Md, align: AlignItems::Center,
    Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Xs }
    Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm }
    Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Md }
    Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Lg }
    Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Xl }
}"#;

#[component]
fn Presence() -> Element {
    rsx! {
        Group { gap: SpacingToken::Lg, align: AlignItems::Center,
            Avatar { name: Some("Marco Bellini".to_string()), status: Some(PresenceStatus::Online) }
            Avatar { name: Some("Sara Conti".to_string()), status: Some(PresenceStatus::Busy) }
            Avatar { name: Some("Luca Moretti".to_string()), status: Some(PresenceStatus::Offline) }
            Avatar { name: Some("relay-02".to_string()), status: Some(PresenceStatus::DirectP2P) }
            Avatar { icon: Some(IconKind::Server.into()), alt: Some("Nodo senza nome".to_string()) }
        }
    }
}
const PRESENCE_CODE: &str = r#"Group { gap: SpacingToken::Lg, align: AlignItems::Center,
    Avatar { name: Some("Marco Bellini".to_string()), status: Some(PresenceStatus::Online) }
    Avatar { name: Some("Sara Conti".to_string()), status: Some(PresenceStatus::Busy) }
    Avatar { name: Some("Luca Moretti".to_string()), status: Some(PresenceStatus::Offline) }
    Avatar { name: Some("relay-02".to_string()), status: Some(PresenceStatus::DirectP2P) }
    Avatar { icon: Some(IconKind::Server.into()), alt: Some("Nodo senza nome".to_string()) }
}"#;

#[component]
fn Stacked() -> Element {
    rsx! {
        Group { gap: SpacingToken::Md, align: AlignItems::Center,
            AvatarGroup {
                Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm }
                Avatar { name: Some("Marco Bellini".to_string()), size: AvatarSize::Sm }
                Avatar { name: Some("Sara Conti".to_string()), size: AvatarSize::Sm }
                Avatar { src: Some("https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?auto=format&fit=crop&w=160&q=80".to_string()), alt: Some("Luca Moretti".to_string()), size: AvatarSize::Sm }
            }
            Text { size: TextSize::Sm, dimmed: true, "4 revisori su questa release" }
        }
    }
}
const STACKED_CODE: &str = r#"Group { gap: SpacingToken::Md, align: AlignItems::Center,
    AvatarGroup {
        Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm }
        Avatar { name: Some("Marco Bellini".to_string()), size: AvatarSize::Sm }
        Avatar { name: Some("Sara Conti".to_string()), size: AvatarSize::Sm }
        Avatar { src: Some("https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?auto=format&fit=crop&w=160&q=80".to_string()), alt: Some("Luca Moretti".to_string()), size: AvatarSize::Sm }
    }
    Text { size: TextSize::Sm, dimmed: true, "4 revisori su questa release" }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::OpsDashboard { compact: false }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/ops_dashboard.rs (estratto): l'operatore di turno nella testata, con presenza e tooltip.
Tooltip { content: "Giulia Ferri · in linea".to_string(), position: TooltipPosition::Bottom,
    Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm, status: Some(PresenceStatus::Online) }
}"#;

const AVATAR_PROPS: &[PropDoc] = &[
    prop("name", "Option<String>", "None", "Nome completo: genera le iniziali e una tinta stabile per persona."),
    prop("src", "Option<String>", "None", "URL dell'immagine; ha la precedenza sulle iniziali."),
    prop("alt", "Option<String>", "None", "Testo alternativo; di default il nome."),
    prop("icon", "Option<SvgIcon>", "None", "Icona quando non ci sono né immagine né nome."),
    prop("size", "AvatarSize", "Md", "Xs 24, Sm 32, Md 40, Lg 48, Xl 64 px."),
    prop("status", "Option<PresenceStatus>", "None", "Punto di presenza: Online, Busy, Offline, DirectP2P."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const GROUP_PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Avatar sovrapposti con spaziatura negativa."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const PERSONA_PROPS: &[PropDoc] = &[
    prop("PersonaProps::new(name)", "fn", "-", "Persona usata da Facepile, ActivityFeed e DocumentCard."),
    prop(".with_avatar(url) / .with_status(s) / .with_role(r)", "fn", "-", "Immagine, presenza e ruolo."),
];

pub fn story() -> Story {
    Story {
        slug: "avatars",
        name: "Avatar",
        category: Category::Contenuti,
        features: &["avatar"],
        icon: IconKind::Fingerprint,
        tagline: "Immagine o iniziali dell'utente, con stato di presenza e gruppi sovrapposti.",
        keywords: &["avatar", "utente", "profilo", "iniziali", "presenza", "avatar group"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Avatar",
            controls: vec![
                Control::text("name", "Nome", "Giulia Ferri").emit(Emit::Template("name: Some(\"{v}\".to_string())")),
                Control::choice("size", "Dimensione", SIZE_OPTS, 3).emit(Emit::Always),
                Control::choice("status", "Presenza", STATUS_OPTS, 0).emit(Emit::Always),
                Control::bool("src", "Foto", false).emit(Emit::Template("src: Some(\"https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?auto=format&fit=crop&w=160&q=80\".to_string())")),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "sizes",
                title: "Dimensioni",
                description: "Da Xs a Xl; le iniziali scalano con il cerchio.",
                code: SIZES_CODE,
                render: || rsx! { Sizes {} },
                span: Span::Half,
            },
            Example {
                id: "presence",
                title: "Presenza e fallback",
                description: "Quattro stati e un'icona quando manca il nome.",
                code: PRESENCE_CODE,
                render: || rsx! { Presence {} },
                span: Span::Half,
            },
            Example {
                id: "group",
                title: "AvatarGroup",
                description: "Avatar sovrapposti accanto a un riepilogo.",
                code: STACKED_CODE,
                render: || rsx! { Stacked {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "L'operatore di turno nella testata della dashboard.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Avatar", rows: AVATAR_PROPS },
            PropGroup { component: "AvatarGroup", rows: GROUP_PROPS },
            PropGroup { component: "PersonaProps", rows: PERSONA_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Pill shape (language.md §2) su superficie flat-lit",
            depth: "Flat-lit",
            tokens: &["--sd-radius-full", "--sd-surface-2", "--sd-border", "--sd-success", "--sd-danger", "--sd-text-subtle"],
            notes: "Il cerchio resta tale con ogni scala di raggio; la presenza è un piccolo punto semantico, mai un alone.",
        }),
        a11y: &[
            "L'immagine ha alt uguale ad alt o al nome; il contenitore riporta lo stesso testo in title.",
            "Il punto di presenza ha un title con lo stato (Online, Busy, Offline, Direct P2P).",
            "Le iniziali sono testo: senza immagine il nome resta leggibile da title.",
            "Avatar non è interattivo: per un profilo cliccabile avvolgilo in un elemento attivabile.",
        ],
        related: &["facepile", "chat", "activity-feed", "indicator", "badges"],
    }
}
