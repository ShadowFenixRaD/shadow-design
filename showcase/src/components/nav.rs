use crate::Route;
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct ComponentNavEntry {
    pub name: &'static str,
    pub route: Route,
    pub group: &'static str,
    pub icon: IconKind,
    pub keywords: &'static str,
    pub description: &'static str,
}

pub const COMPONENTS: &[ComponentNavEntry] = &[
    ComponentNavEntry {
        name: "Panoramica",
        route: Route::Home {},
        group: "Generale",
        icon: IconKind::Layers,
        keywords: "overview intro home install getting started",
        description: "Panoramica della libreria e guida introduttiva.",
    },
    ComponentNavEntry {
        name: "Theme Builder",
        route: Route::ThemeBuilderPage {},
        group: "Generale",
        icon: IconKind::Palette,
        keywords: "theme palette color mantine customization tokens",
        description: "Studio visivo per progettazione e generazione temi.",
    },
    ComponentNavEntry {
        name: "Typography",
        route: Route::TypographyPage {},
        group: "Generale",
        icon: IconKind::Code,
        keywords: "text heading kbd code geist font",
        description: "Gerarchia tipografica Geist e blocchi codice.",
    },
    ComponentNavEntry {
        name: "Button",
        route: Route::ButtonsPage {},
        group: "Componenti Core",
        icon: IconKind::Sparkles,
        keywords: "button action click trigger press",
        description: "Pulsanti tattili con varianti, loading e glow.",
    },
    ComponentNavEntry {
        name: "Card",
        route: Route::CardsPage {},
        group: "Componenti Core",
        icon: IconKind::Box,
        keywords: "card surface container panel box",
        description: "Schede contenitore con varianti ed elevazioni.",
    },
    ComponentNavEntry {
        name: "TextInput",
        route: Route::InputsPage {},
        group: "Componenti Core",
        icon: IconKind::Terminal,
        keywords: "input text field form clearable icon",
        description: "Campi di testo reattivi con supporto icone e clear button.",
    },
    ComponentNavEntry {
        name: "Select & Combobox",
        route: Route::SelectPage {},
        group: "Componenti Core",
        icon: IconKind::ChevronDown,
        keywords: "select multiselect combobox listbox dropdown search",
        description: "Menu di selezione singola, multipla e combobox editabile.",
    },
    ComponentNavEntry {
        name: "Textarea",
        route: Route::TextareaPage {},
        group: "Componenti Core",
        icon: IconKind::Code,
        keywords: "textarea multiline text input autosize",
        description: "Area di testo multilinea con autosize intelligente.",
    },
    ComponentNavEntry {
        name: "Checkbox",
        route: Route::CheckboxPage {},
        group: "Componenti Core",
        icon: IconKind::Check,
        keywords: "checkbox check toggle selection indeterminate",
        description: "Casella di controllo tattile ad alta precisione.",
    },
    ComponentNavEntry {
        name: "Badge",
        route: Route::BadgesPage {},
        group: "Componenti Core",
        icon: IconKind::CheckCircle,
        keywords: "badge status pill dot tag label",
        description: "Etichette di stato e pillole con indicatore pulsante.",
    },
    ComponentNavEntry {
        name: "Alert",
        route: Route::AlertsPage {},
        group: "Componenti Core",
        icon: IconKind::AlertTriangle,
        keywords: "alert notification banner warning error info success",
        description: "Riquadri di avviso e notifiche di sistema in-line.",
    },
    ComponentNavEntry {
        name: "Modal",
        route: Route::ModalsPage {},
        group: "Componenti Core",
        icon: IconKind::Eye,
        keywords: "modal dialog popup overlay window",
        description: "Finestre di dialogo in overlay con backdrop click-to-close.",
    },
    ComponentNavEntry {
        name: "Navbar",
        route: Route::NavbarDemoPage {},
        group: "Componenti Core",
        icon: IconKind::Menu,
        keywords: "navbar navigation header brand menu",
        description: "Barra di navigazione superiore responsive con specular highlight.",
    },
    ComponentNavEntry {
        name: "Notifications",
        route: Route::NotificationsPage {},
        group: "Interattività & Avanzati",
        icon: IconKind::Info,
        keywords: "toast notification feedback alert live-region",
        description: "Sistema globale di notifiche toast con WAI-ARIA live region.",
    },
    ComponentNavEntry {
        name: "Tabs",
        route: Route::TabsPage {},
        group: "Interattività & Avanzati",
        icon: IconKind::Layers,
        keywords: "tabs tablist tabpanel navigation switch",
        description: "Navigazione a schede orizzontali stile Mantine.",
    },
    ComponentNavEntry {
        name: "Switch",
        route: Route::SwitchPage {},
        group: "Interattività & Avanzati",
        icon: IconKind::Settings,
        keywords: "switch toggle boolean check setting",
        description: "Interruttori a scatto tattili ad alta precisione.",
    },
    ComponentNavEntry {
        name: "Tooltip",
        route: Route::TooltipPage {},
        group: "Interattività & Avanzati",
        icon: IconKind::Sparkles,
        keywords: "tooltip hint popup balloon hover focus",
        description: "Micro-etichette fluttuanti contestuali su hover e focus.",
    },
    ComponentNavEntry {
        name: "Drawer",
        route: Route::DrawerPage {},
        group: "Interattività & Avanzati",
        icon: IconKind::ChevronRight,
        keywords: "drawer sidebar offcanvas slideout panel",
        description: "Pannelli laterali a comparsa fluida da sinistra o destra.",
    },
    ComponentNavEntry {
        name: "Dropdown",
        route: Route::DropdownPage {},
        group: "Interattività & Avanzati",
        icon: IconKind::ChevronDown,
        keywords: "dropdown menu context popup actions",
        description: "Menu a comparsa ancorati contestuali con dismiss automatico.",
    },
    ComponentNavEntry {
        name: "Icons",
        route: Route::IconsPage {},
        group: "Interattività & Avanzati",
        icon: IconKind::Search,
        keywords: "icon svg vector glyph symbols",
        description: "50 icone geometriche vettoriali native ad alta nitidezza.",
    },
    ComponentNavEntry {
        name: "Animation",
        route: Route::AnimationsPage {},
        group: "Interattività & Avanzati",
        icon: IconKind::Sparkles,
        keywords: "animation transition fade slide scale spin motion",
        description: "Motore di micro-animazioni Dioxus hardware-accelerated.",
    },
    ComponentNavEntry {
        name: "Avatar",
        route: Route::AvatarsPage {},
        group: "Chat & Mesh P2P",
        icon: IconKind::Sparkles,
        keywords: "avatar profile user picture image",
        description: "Avatar utente geometrici con indicatori di stato.",
    },
    ComponentNavEntry {
        name: "Chat Bubble",
        route: Route::ChatPage {},
        group: "Chat & Mesh P2P",
        icon: IconKind::MessageSquare,
        keywords: "chat bubble message conversation peer",
        description: "Baloon di messaggistica istantanea crittografata.",
    },
    ComponentNavEntry {
        name: "AttachmentCard",
        route: Route::AttachmentCardsPage {},
        group: "Chat & Mesh P2P",
        icon: IconKind::Paperclip,
        keywords: "attachment card file media document",
        description: "Scheda multimediale per file allegati e download.",
    },
    ComponentNavEntry {
        name: "Progress",
        route: Route::ProgressPage {},
        group: "Chat & Mesh P2P",
        icon: IconKind::Spinner,
        keywords: "progress bar meter loading indicator",
        description: "Barre di avanzamento percentuale animate.",
    },
    ComponentNavEntry {
        name: "SegmentedControl",
        route: Route::SegmentedControlsPage {},
        group: "Chat & Mesh P2P",
        icon: IconKind::Layers,
        keywords: "segmented control tabs toggle picker",
        description: "Controlli segmentati lineari a selezione esclusiva.",
    },
    ComponentNavEntry {
        name: "Divider",
        route: Route::DividersPage {},
        group: "Chat & Mesh P2P",
        icon: IconKind::Code,
        keywords: "divider separator line horizontal vertical",
        description: "Separatori visivi orizzontali e verticali con etichette.",
    },
    ComponentNavEntry {
        name: "TreeView (Mesh)",
        route: Route::TreesPage {},
        group: "Chat & Mesh P2P",
        icon: IconKind::Radio,
        keywords: "tree view hierarchy nodes mesh network",
        description: "Visualizzatore gerarchico di nodi e canali di rete mesh.",
    },
    ComponentNavEntry {
        name: "Skeleton",
        route: Route::SkeletonsPage {},
        group: "Chat & Mesh P2P",
        icon: IconKind::Box,
        keywords: "skeleton placeholder loading shimmer pulse",
        description: "Segnaposto di caricamento con shimmer progressivo.",
    },
    ComponentNavEntry {
        name: "Accordion",
        route: Route::AccordionPage {},
        group: "Interattività & Avanzati",
        icon: IconKind::ChevronDown,
        keywords: "accordion collapsible collapse expand panel fold",
        description: "Pannelli espandibili a fisarmonica con animazioni fluide e chevron.",
    },
];

#[component]
pub fn SearchPalette(mut is_open: Signal<bool>) -> Element {
    let query = use_signal(String::new);
    let nav = use_navigator();

    let filtered: Vec<&ComponentNavEntry> = {
        let q = query().to_lowercase();
        if q.is_empty() {
            COMPONENTS.iter().take(8).collect()
        } else {
            COMPONENTS
                .iter()
                .filter(|c| {
                    c.name.to_lowercase().contains(&q)
                        || c.group.to_lowercase().contains(&q)
                        || c.keywords.to_lowercase().contains(&q)
                })
                .collect()
        }
    };

    rsx! {
        Modal {
            is_open: is_open,
            title: Some("Cerca componenti e documentazione".to_string()),
            div { class: "flex flex-col gap-4",
                TextInput {
                    value: query,
                    placeholder: "Digita per cercare... (es. Button, Select, Theme)".to_string(),
                    left_icon: SvgIcon::Kind(IconKind::Search),
                }

                div { class: "flex flex-col gap-1 max-h-80 overflow-y-auto pr-1",
                    if filtered.is_empty() {
                        div { class: "p-6 text-center text-xs text-zinc-500 font-mono",
                            "Nessun componente corrispondente a \"{query}\""
                        }
                    } else {
                        for comp in filtered {
                            button {
                                key: "{comp.name}",
                                r#type: "button",
                                class: "flex items-center justify-between p-3 rounded-xl border border-white/[0.04] bg-white/[0.02] hover:bg-white/[0.06] hover:border-purple-500/30 transition-all cursor-pointer text-left group",
                                onclick: {
                                    let r = comp.route.clone();
                                    move |_| {
                                        is_open.set(false);
                                        nav.push(r.clone());
                                    }
                                },
                                div { class: "flex items-center gap-3",
                                    span { class: "text-zinc-400 group-hover:text-purple-300 transition-colors",
                                        Icon { kind: comp.icon, size: 16 }
                                    }
                                    div { class: "flex flex-col",
                                        span { class: "text-xs font-semibold text-zinc-200 group-hover:text-white transition-colors",
                                            "{comp.name}"
                                        }
                                        span { class: "text-[11px] text-zinc-400 truncate max-w-sm",
                                            "{comp.description}"
                                        }
                                    }
                                }
                                span { class: "text-[10px] font-mono px-2 py-0.5 rounded bg-white/[0.04] text-zinc-400 border border-white/[0.06]",
                                    "{comp.group}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
