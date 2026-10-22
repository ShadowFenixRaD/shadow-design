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
    // --- Generale ---
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
        name: "Icons",
        route: Route::IconsPage {},
        group: "Generale",
        icon: IconKind::Search,
        keywords: "icon svg vector glyph symbols",
        description: "50 icone geometriche vettoriali native ad alta nitidezza.",
    },
    ComponentNavEntry {
        name: "Animation",
        route: Route::AnimationsPage {},
        group: "Generale",
        icon: IconKind::Sparkles,
        keywords: "animation transition fade slide scale spin motion",
        description: "Motore di micro-animazioni Dioxus hardware-accelerated.",
    },

    // --- Layout & Struttura ---
    ComponentNavEntry {
        name: "AppShell",
        route: Route::AppShellPage {},
        group: "Layout & Struttura",
        icon: IconKind::Box,
        keywords: "appshell layout header navbar aside main footer master",
        description: "Layout master responsive a 5 zone con collasso fluido e z-index standard.",
    },
    ComponentNavEntry {
        name: "Navbar",
        route: Route::NavbarDemoPage {},
        group: "Layout & Struttura",
        icon: IconKind::Menu,
        keywords: "navbar navigation header brand menu",
        description: "Barra di navigazione superiore responsive con specular highlight.",
    },
    ComponentNavEntry {
        name: "Card",
        route: Route::CardsPage {},
        group: "Layout & Struttura",
        icon: IconKind::Box,
        keywords: "card surface container panel box",
        description: "Schede contenitore con varianti ed elevazioni.",
    },
    ComponentNavEntry {
        name: "Grid",
        route: Route::GridPage {},
        group: "Layout & Struttura",
        icon: IconKind::Layers,
        keywords: "grid col column 12 span responsive offset gutter layout",
        description: "Sistema a griglia responsive a 12 colonne con span e offset.",
    },
    ComponentNavEntry {
        name: "SimpleGrid",
        route: Route::GridPage {},
        group: "Layout & Struttura",
        icon: IconKind::Box,
        keywords: "simplegrid simple grid equal columns auto layout spacing",
        description: "Griglia automatica a colonne di larghezza equivalente con gutter.",
    },
    ComponentNavEntry {
        name: "Group",
        route: Route::GroupPage {},
        group: "Layout & Struttura",
        icon: IconKind::Layers,
        keywords: "group flex row wrap align justify gap flexbox layout",
        description: "Disposizione orizzontale flessibile con wrapping, allineamento e gap.",
    },
    ComponentNavEntry {
        name: "Stack",
        route: Route::StackPage {},
        group: "Layout & Struttura",
        icon: IconKind::Layers,
        keywords: "stack flex column vertical align justify gap spacing flexbox layout",
        description: "Disposizione verticale a colonna con spaziatura tokenizzata uniforme.",
    },
    ComponentNavEntry {
        name: "AspectRatio",
        route: Route::AspectRatioPage {},
        group: "Layout & Struttura",
        icon: IconKind::Box,
        keywords: "aspect ratio container media video canvas iframe 16:9 4:3 1:1 cls",
        description: "Contenitore per vincolare il rapporto di forma media azzerando il CLS.",
    },
    ComponentNavEntry {
        name: "Center",
        route: Route::CenterPage {},
        group: "Layout & Struttura",
        icon: IconKind::Box,
        keywords: "center centering horizontal vertical inline flex alignment",
        description: "Contenitore per centrare elementi in orizzontale e verticale.",
    },
    ComponentNavEntry {
        name: "ScrollArea",
        route: Route::ScrollAreaPage {},
        group: "Layout & Struttura",
        icon: IconKind::Layers,
        keywords: "scroll area scrollbar overflow virtual fade edges",
        description: "Area con micro-scrollbar Shadow, fade edges e auto-hide.",
    },
    ComponentNavEntry {
        name: "Divider",
        route: Route::DividersPage {},
        group: "Layout & Struttura",
        icon: IconKind::Code,
        keywords: "divider separator line horizontal vertical",
        description: "Separatori visivi orizzontali e verticali con etichette.",
    },
    ComponentNavEntry {
        name: "Accordion",
        route: Route::AccordionPage {},
        group: "Layout & Struttura",
        icon: IconKind::ChevronDown,
        keywords: "accordion collapsible fold collapse expand panel",
        description: "Pannelli espandibili a fisarmonica conformi a WAI-ARIA.",
    },
    ComponentNavEntry {
        name: "Spoiler",
        route: Route::SpoilerPage {},
        group: "Layout & Struttura",
        icon: IconKind::ChevronDown,
        keywords: "spoiler hide show reveal expand collapse truncate",
        description: "Contenitore con dissolvenza inferiore e pulsante mostra di più / mostra meno.",
    },

    // --- Navigazione ---
    ComponentNavEntry {
        name: "NavLink",
        route: Route::NavLinkPage {},
        group: "Navigazione",
        icon: IconKind::Menu,
        keywords: "nav link sidebar menu item neon active submenu",
        description: "Voce di menu per barre laterali con neon active e sottomenu.",
    },
    ComponentNavEntry {
        name: "Tabs",
        route: Route::TabsPage {},
        group: "Navigazione",
        icon: IconKind::Layers,
        keywords: "tabs tablist tabpanel navigation switch",
        description: "Navigazione a schede orizzontali stile Mantine.",
    },
    ComponentNavEntry {
        name: "Breadcrumbs",
        route: Route::BreadcrumbsPage {},
        group: "Navigazione",
        icon: IconKind::ChevronRight,
        keywords: "breadcrumbs nav path trail wayfinding hierarchy",
        description: "Navigazione a briciole di pane con ellissi e separatori.",
    },
    ComponentNavEntry {
        name: "Pagination",
        route: Route::PaginationPage {},
        group: "Navigazione",
        icon: IconKind::Layers,
        keywords: "pagination pages numbers table navigation pager",
        description: "Barra di navigazione a pagine numerata con salti ellittici.",
    },
    ComponentNavEntry {
        name: "Stepper",
        route: Route::StepperPage {},
        group: "Navigazione",
        icon: IconKind::CheckCircle,
        keywords: "stepper step wizard progress multi step workflow",
        description: "Guida procedurale a passi con orientamento orizzontale e verticale.",
    },
    ComponentNavEntry {
        name: "BottomNavigation",
        route: Route::BottomNavigationPage {},
        group: "Navigazione",
        icon: IconKind::Menu,
        keywords: "bottom navigation bar mobile tab footer safe area",
        description: "Barra di navigazione inferiore per mobile con safe area e micro-glow.",
    },
    ComponentNavEntry {
        name: "Menubar",
        route: Route::MenubarPage {},
        group: "Navigazione",
        icon: IconKind::Menu,
        keywords: "menubar menu command bar desktop navigation shortcut",
        description: "Barra dei menu orizzontale in stile app desktop con shortcut e tendine.",
    },

    // --- Inserimento & Form ---
    ComponentNavEntry {
        name: "Button",
        route: Route::ButtonsPage {},
        group: "Inserimento & Form",
        icon: IconKind::Sparkles,
        keywords: "button action click trigger press",
        description: "Pulsanti tattili con varianti, loading e glow.",
    },
    ComponentNavEntry {
        name: "TextInput",
        route: Route::InputsPage {},
        group: "Inserimento & Form",
        icon: IconKind::Terminal,
        keywords: "input text field form clearable icon",
        description: "Campi di testo reattivi con supporto icone e clear button.",
    },
    ComponentNavEntry {
        name: "NumberInput",
        route: Route::NumberInputPage {},
        group: "Inserimento & Form",
        icon: IconKind::Terminal,
        keywords: "number input spinbutton stepper numeric counter",
        description: "Input numerico con stepper su/giù, decimali e limiti min/max.",
    },
    ComponentNavEntry {
        name: "PinInput / OTP",
        route: Route::PinInputPage {},
        group: "Inserimento & Form",
        icon: IconKind::Terminal,
        keywords: "pin input otp code 2fa token password mask",
        description: "Inserimento tattile di codici PIN e token OTP a caselle.",
    },
    ComponentNavEntry {
        name: "Textarea",
        route: Route::TextareaPage {},
        group: "Inserimento & Form",
        icon: IconKind::Code,
        keywords: "textarea multiline text input autosize",
        description: "Area di testo multilinea con autosize intelligente.",
    },
    ComponentNavEntry {
        name: "Select & Combobox",
        route: Route::SelectPage {},
        group: "Inserimento & Form",
        icon: IconKind::ChevronDown,
        keywords: "select multiselect combobox listbox dropdown search",
        description: "Menu di selezione singola, multipla e combobox editabile.",
    },
    ComponentNavEntry {
        name: "Checkbox",
        route: Route::CheckboxPage {},
        group: "Inserimento & Form",
        icon: IconKind::Check,
        keywords: "checkbox check toggle selection indeterminate",
        description: "Casella di controllo tattile ad alta precisione.",
    },
    ComponentNavEntry {
        name: "Radio & RadioGroup",
        route: Route::RadioPage {},
        group: "Inserimento & Form",
        icon: IconKind::Radio,
        keywords: "radio radiogroup radiocard choice select single option",
        description: "Selezione singola esclusiva con Radio, RadioGroup e RadioCard.",
    },
    ComponentNavEntry {
        name: "Switch",
        route: Route::SwitchPage {},
        group: "Inserimento & Form",
        icon: IconKind::Settings,
        keywords: "switch toggle boolean check setting",
        description: "Interruttori a scatto tattili ad alta precisione.",
    },
    ComponentNavEntry {
        name: "Slider & RangeSlider",
        route: Route::SliderPage {},
        group: "Inserimento & Form",
        icon: IconKind::Layers,
        keywords: "slider rangeslider track thumb range value meter",
        description: "Cursore graduato a trascinamento con tacche e supporto doppio range.",
    },
    ComponentNavEntry {
        name: "Rating",
        route: Route::RatingPage {},
        group: "Inserimento & Form",
        icon: IconKind::Sparkles,
        keywords: "rating stars score evaluation feedback half fractions review",
        description: "Valutazione a stelle o icone con supporto per mezze valutazioni e anteprima hover.",
    },
    ComponentNavEntry {
        name: "Chip & ChipGroup",
        route: Route::ChipPage {},
        group: "Inserimento & Form",
        icon: IconKind::Layers,
        keywords: "chip chipgroup tag filter badge toggle pill",
        description: "Capsule interattive per filtri, tag e selezioni multiple o singole.",
    },
    ComponentNavEntry {
        name: "ColorPicker & ColorInput",
        route: Route::ColorPickerPage {},
        group: "Inserimento & Form",
        icon: IconKind::Sparkles,
        keywords: "color picker input hex rgb hsl alpha swatch",
        description: "Selettore cromatico avanzato con slider HSL, alpha e palette swatch.",
    },
    ComponentNavEntry {
        name: "SegmentedControl",
        route: Route::SegmentedControlsPage {},
        group: "Inserimento & Form",
        icon: IconKind::Layers,
        keywords: "segmented control tabs toggle picker",
        description: "Controlli segmentati lineari a selezione esclusiva.",
    },

    // --- Dati & Feedback ---
    ComponentNavEntry {
        name: "Table",
        route: Route::TablesPage {},
        group: "Dati & Feedback",
        icon: IconKind::Layers,
        keywords: "table datatable grid cells sorting pagination rows",
        description: "Tabelle dati semantiche con ordinamento, paginazione e hover.",
    },
    ComponentNavEntry {
        name: "List",
        route: Route::ListsPage {},
        group: "Dati & Feedback",
        icon: IconKind::Layers,
        keywords: "list listitem ordered unordered bullet ol ul items",
        description: "Elenchi ordinati, non ordinati e con icone personalizzate.",
    },
    ComponentNavEntry {
        name: "Timeline",
        route: Route::TimelinesPage {},
        group: "Dati & Feedback",
        icon: IconKind::Clock,
        keywords: "timeline events history steps audit tracking status",
        description: "Linea guida temporale e cronologia eventi con bullet di stato.",
    },
    ComponentNavEntry {
        name: "Badge",
        route: Route::BadgesPage {},
        group: "Dati & Feedback",
        icon: IconKind::CheckCircle,
        keywords: "badge status pill dot tag label",
        description: "Etichette di stato e pillole con indicatore pulsante.",
    },
    ComponentNavEntry {
        name: "Alert",
        route: Route::AlertsPage {},
        group: "Dati & Feedback",
        icon: IconKind::AlertTriangle,
        keywords: "alert notification banner warning error info success",
        description: "Riquadri di avviso e notifiche di sistema in-line.",
    },
    ComponentNavEntry {
        name: "Notifications",
        route: Route::NotificationsPage {},
        group: "Dati & Feedback",
        icon: IconKind::Info,
        keywords: "toast notification feedback alert live-region",
        description: "Sistema globale di notifiche toast con WAI-ARIA live region.",
    },
    ComponentNavEntry {
        name: "Progress",
        route: Route::ProgressPage {},
        group: "Dati & Feedback",
        icon: IconKind::Spinner,
        keywords: "progress bar meter loading indicator",
        description: "Barre di avanzamento percentuale animate.",
    },
    ComponentNavEntry {
        name: "Skeleton",
        route: Route::SkeletonsPage {},
        group: "Dati & Feedback",
        icon: IconKind::Box,
        keywords: "skeleton placeholder loading shimmer pulse",
        description: "Segnaposto di caricamento con shimmer progressivo.",
    },

    // --- Overlay & Finestre ---
    ComponentNavEntry {
        name: "Modal",
        route: Route::ModalsPage {},
        group: "Overlay & Finestre",
        icon: IconKind::Eye,
        keywords: "modal dialog popup overlay window",
        description: "Finestre di dialogo in overlay con backdrop click-to-close.",
    },
    ComponentNavEntry {
        name: "Drawer",
        route: Route::DrawerPage {},
        group: "Overlay & Finestre",
        icon: IconKind::ChevronRight,
        keywords: "drawer sidebar offcanvas slideout panel",
        description: "Pannelli laterali a comparsa fluida da sinistra o destra.",
    },
    ComponentNavEntry {
        name: "Dropdown",
        route: Route::DropdownPage {},
        group: "Overlay & Finestre",
        icon: IconKind::ChevronDown,
        keywords: "dropdown menu context popup actions",
        description: "Menu a comparsa ancorati contestuali con dismiss automatico.",
    },
    ComponentNavEntry {
        name: "Tooltip",
        route: Route::TooltipPage {},
        group: "Overlay & Finestre",
        icon: IconKind::Sparkles,
        keywords: "tooltip hint popup balloon hover focus",
        description: "Micro-etichette fluttuanti contestuali su hover e focus.",
    },

    // --- Chat & P2P Mesh ---
    ComponentNavEntry {
        name: "Avatar",
        route: Route::AvatarsPage {},
        group: "Chat & P2P Mesh",
        icon: IconKind::Sparkles,
        keywords: "avatar profile user picture image",
        description: "Avatar utente geometrici con indicatori di stato.",
    },
    ComponentNavEntry {
        name: "Chat Bubble",
        route: Route::ChatPage {},
        group: "Chat & P2P Mesh",
        icon: IconKind::MessageSquare,
        keywords: "chat bubble message conversation peer",
        description: "Baloon di messaggistica istantanea crittografata.",
    },
    ComponentNavEntry {
        name: "AttachmentCard",
        route: Route::AttachmentCardsPage {},
        group: "Chat & P2P Mesh",
        icon: IconKind::Paperclip,
        keywords: "attachment card file media document contact",
        description: "Scheda multimediale per file allegati, documenti e contatti.",
    },
    ComponentNavEntry {
        name: "TreeView (Mesh)",
        route: Route::TreesPage {},
        group: "Chat & P2P Mesh",
        icon: IconKind::Radio,
        keywords: "tree view hierarchy nodes mesh network",
        description: "Visualizzatore gerarchico di nodi e canali di rete mesh.",
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
            size: ModalSize::Xl,
            class: "max-w-4xl w-full".to_string(),
            title: Some("Cerca componenti e documentazione".to_string()),
            div { class: "flex flex-col gap-4 w-full min-w-0",
                TextInput {
                    value: query,
                    placeholder: "Digita per cercare... (es. Button, Select, Theme)".to_string(),
                    left_icon: SvgIcon::Kind(IconKind::Search),
                }

                div { class: "flex flex-col gap-1 max-h-96 overflow-y-auto overflow-x-hidden pr-1 w-full min-w-0",
                    if filtered.is_empty() {
                        div { class: "p-6 text-center text-xs text-zinc-500 font-mono w-full",
                            "Nessun componente corrispondente a \"{query}\""
                        }
                    } else {
                        for comp in filtered {
                            button {
                                key: "{comp.name}",
                                r#type: "button",
                                class: "w-full flex items-center justify-between p-3.5 rounded-xl border border-white/[0.04] bg-white/[0.02] hover:bg-white/[0.06] hover:border-purple-500/30 transition-all cursor-pointer text-left group gap-4 min-w-0 box-border",
                                onclick: {
                                    let r = comp.route.clone();
                                    move |_| {
                                        is_open.set(false);
                                        nav.push(r.clone());
                                    }
                                },
                                div { class: "flex items-center gap-3.5 min-w-0 flex-1",
                                    span { class: "text-zinc-400 group-hover:text-purple-300 transition-colors shrink-0 p-2 rounded-lg bg-white/[0.03] border border-white/[0.04] flex items-center justify-center",
                                        Icon { kind: comp.icon, size: 18 }
                                    }
                                    div { class: "flex flex-col min-w-0 flex-1",
                                        span { class: "text-sm font-semibold text-zinc-100 group-hover:text-white transition-colors truncate",
                                            "{comp.name}"
                                        }
                                        span { class: "text-xs text-zinc-400 truncate w-full",
                                            "{comp.description}"
                                        }
                                    }
                                }
                                span { class: "text-[11px] font-mono px-2.5 py-1 rounded-md bg-white/[0.04] text-zinc-400 border border-white/[0.06] shrink-0 whitespace-nowrap",
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
