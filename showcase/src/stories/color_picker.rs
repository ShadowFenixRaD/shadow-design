//! Story: ColorPicker and ColorInput (feature `color_picker`). Stateful: the playground goes
//! through `PickerDemo { knobs }`. Colour values come from presets, never from hex literals.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const FORMATS: [ColorFormat; 3] = [ColorFormat::Hex, ColorFormat::Rgb, ColorFormat::Hsl];
const FORMAT_OPTS: &[ChoiceOption] = &[opt("Hex", "ColorFormat::Hex"), opt("Rgb", "ColorFormat::Rgb"), opt("Hsl", "ColorFormat::Hsl")];
const SIZES: [ControlSize; 3] = [ControlSize::Sm, ControlSize::Md, ControlSize::Lg];
const SIZE_OPTS: &[ChoiceOption] = &[opt("Sm", "ControlSize::Sm"), opt("Md", "ControlSize::Md"), opt("Lg", "ControlSize::Lg")];

const PRELUDE: &str = "let value = use_signal(|| PrimaryColor::Violet.hex().to_string());";

#[component]
fn PickerDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| PrimaryColor::Violet.hex().to_string());
    rsx! {
        ColorPicker {
            value,
            format: knobs.pick("format", &FORMATS),
            with_alpha: knobs.bool("with_alpha"),
            size: knobs.pick("size", &SIZES),
            disabled: knobs.bool("disabled"),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        PickerDemo { knobs: k.clone() }
    }
}

#[component]
fn Field() -> Element {
    let brand = use_signal(|| PrimaryColor::Cyan.hex().to_string());
    let alert = use_signal(String::new);
    rsx! {
        Stack { gap: SpacingToken::Md,
            ColorInput {
                value: brand,
                label: Some("Colore del brand".to_string()),
                description: Some("Usato per grafici e badge del team.".to_string()),
                clearable: true,
            }
            ColorInput {
                value: alert,
                label: Some("Colore degli avvisi".to_string()),
                placeholder: Some("Scegli un colore".to_string()),
                error: if alert().is_empty() { Some("Obbligatorio".to_string()) } else { None },
            }
        }
    }
}
const FIELD_CODE: &str = r#"let brand = use_signal(|| PrimaryColor::Cyan.hex().to_string());
let alert = use_signal(String::new);

Stack { gap: SpacingToken::Md,
    ColorInput {
        value: brand,
        label: Some("Colore del brand".to_string()),
        description: Some("Usato per grafici e badge del team.".to_string()),
        clearable: true,
    }
    ColorInput {
        value: alert,
        label: Some("Colore degli avvisi".to_string()),
        placeholder: Some("Scegli un colore".to_string()),
        error: if alert().is_empty() { Some("Obbligatorio".to_string()) } else { None },
    }
}"#;

#[component]
fn Swatches() -> Element {
    let accent = use_signal(|| PrimaryColor::Emerald.hex().to_string());
    rsx! {
        ColorPicker {
            value: accent,
            size: ControlSize::Sm,
            swatches: vec![
                PrimaryColor::Violet.hex().to_string(),
                PrimaryColor::Obsidian.hex().to_string(),
                PrimaryColor::Crimson.hex().to_string(),
                PrimaryColor::Cyan.hex().to_string(),
                PrimaryColor::Emerald.hex().to_string(),
            ],
        }
    }
}
const SWATCHES_CODE: &str = r#"let accent = use_signal(|| PrimaryColor::Emerald.hex().to_string());

ColorPicker {
    value: accent,
    size: ControlSize::Sm,
    swatches: vec![
        PrimaryColor::Violet.hex().to_string(),
        PrimaryColor::Obsidian.hex().to_string(),
        PrimaryColor::Crimson.hex().to_string(),
        PrimaryColor::Cyan.hex().to_string(),
        PrimaryColor::Emerald.hex().to_string(),
    ],
}"#;

#[component]
fn OneHex() -> Element {
    let global = use_theme();
    let value = use_signal(|| PrimaryColor::Crimson.hex().to_string());
    let mut local = use_signal(|| global.peek().clone());
    let relay = use_signal(|| true);
    use_effect(move || {
        let hex = value();
        local.write().apply_primary_color(PrimaryColor::custom("Personalizzato", hex));
    });
    rsx! {
        SimpleGrid { cols: 1, md: Some(2), spacing: GridSpacing::Lg,
            ColorPicker { value }
            ThemeProvider { theme: Some(local),
                Card {
                    title: Some("Anteprima del tema".to_string()),
                    subtitle: Some("Un solo hex ricolora ogni componente".to_string()),
                    Stack { gap: SpacingToken::Md,
                        Group { gap: SpacingToken::Sm, align: AlignItems::Center,
                            Button { "Salva" }
                            Button { variant: ButtonVariant::Outline, "Anteprima" }
                            Badge { label: "Attivo".to_string(), variant: BadgeVariant::Primary, pill: true, dot: true }
                        }
                        Switch { checked: relay, label: Some("Relay automatico".to_string()) }
                        ProgressBar { value: 64.0 }
                    }
                }
            }
        }
    }
}
const ONE_HEX_CODE: &str = r#"let global = use_theme();
let value = use_signal(|| PrimaryColor::Crimson.hex().to_string());
let mut local = use_signal(|| global.peek().clone());
let relay = use_signal(|| true);
use_effect(move || {
    let hex = value();
    local.write().apply_primary_color(PrimaryColor::custom("Personalizzato", hex));
});

SimpleGrid { cols: 1, md: Some(2), spacing: GridSpacing::Lg,
    ColorPicker { value }
    ThemeProvider { theme: Some(local),
        Card {
            title: Some("Anteprima del tema".to_string()),
            subtitle: Some("Un solo hex ricolora ogni componente".to_string()),
            Stack { gap: SpacingToken::Md,
                Group { gap: SpacingToken::Sm, align: AlignItems::Center,
                    Button { "Salva" }
                    Button { variant: ButtonVariant::Outline, "Anteprima" }
                    Badge { label: "Attivo".to_string(), variant: BadgeVariant::Primary, pill: true, dot: true }
                }
                Switch { checked: relay, label: Some("Relay automatico".to_string()) }
                ProgressBar { value: 64.0 }
            }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::SettingsForm { compact: false }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/settings_form.rs (estratto): ColorInput nel Fieldset "Identità".
let label_color = use_signal(|| PrimaryColor::Cyan.hex().to_string());

ColorInput {
    value: label_color,
    label: Some("Colore etichetta".to_string()),
    description: Some("Usato nelle mappe di rete.".to_string()),
}"#;

const PICKER_PROPS: &[PropDoc] = &[
    prop("value", "Signal<String>", "-", "Colore nel formato scelto (hex, rgb() o hsl())."),
    prop("format", "ColorFormat", "Hex", "Hex, Rgb o Hsl; il selettore interno permette di cambiarlo."),
    prop("with_alpha", "bool", "false", "Cursore di opacità e valori con alpha."),
    prop("swatches", "Vec<String>", "default_color_swatches()", "Colori rapidi sotto i cursori."),
    prop("size", "ControlSize", "Md", "Sm, Md o Lg."),
    prop("disabled", "bool", "false", "Selettore inattivo."),
    prop("on_change", "Option<EventHandler<String>>", "None", "Nuovo colore."),
    prop("class", "Option<String>", "None", "Classi aggiuntive."),
];
const INPUT_PROPS: &[PropDoc] = &[
    prop("value", "Signal<String>", "-", "Colore; vuoto se non scelto."),
    prop("label / description / placeholder", "Option<String>", "None", "Etichetta, aiuto e segnaposto del campo."),
    prop("error", "Option<String>", "None", "Bordo danger e messaggio."),
    prop("clearable", "bool", "false", "Pulsante per svuotare il campo."),
    prop("format / with_alpha / swatches / disabled", "-", "-", "Come ColorPicker, passati al pannello."),
    prop("on_change", "Option<EventHandler<String>>", "None", "Nuovo colore."),
    prop("class", "Option<String>", "None", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "color-picker",
        name: "ColorPicker",
        category: Category::Form,
        features: &["color_picker"],
        icon: IconKind::Palette,
        tagline: "Scelta di un colore con area di saturazione, tinta, alpha e campioni, inline o in un campo.",
        keywords: &["color picker", "colore", "hex", "rgb", "hsl", "color input", "palette"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "ColorPicker",
            controls: vec![
                Control::choice("format", "Formato", FORMAT_OPTS, 0),
                Control::choice("size", "Dimensione", SIZE_OPTS, 1),
                Control::bool("with_alpha", "Alpha", false),
                Control::bool("disabled", "Disabilitato", false),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["value,"],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "field",
                title: "ColorInput",
                description: "Campo compatto che apre il selettore in un pannello.",
                code: FIELD_CODE,
                render: || rsx! { Field {} },
                span: Span::Half,
            },
            Example {
                id: "swatches",
                title: "Campioni dei preset",
                description: "I cinque accenti della libreria come scelte rapide.",
                code: SWATCHES_CODE,
                render: || rsx! { Swatches {} },
                span: Span::Half,
            },
            Example {
                id: "one-hex",
                title: "Un hex, tutto il tema",
                description: "Il colore scelto diventa l'accento di un ThemeProvider annidato.",
                code: ONE_HEX_CODE,
                render: || rsx! { OneHex {} },
                span: Span::Full,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il colore dell'etichetta del nodo nel form Impostazioni.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "ColorPicker", rows: PICKER_PROPS },
            PropGroup { component: "ColorInput", rows: INPUT_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Floating panel (§3.2) + recessed field (§3.6)",
            depth: "Flat-lit (pannello inline) · Raised (popup di ColorInput) · Well (campi)",
            tokens: &["--sd-surface-1", "--sd-surface-2", "--sd-elev-2", "--sd-well", "--sd-border-strong", "--sd-radius-lg", "--sd-radius-full"],
            notes: "Il colore scelto è un valore, non uno stile: l'interfaccia del selettore resta nei token del tema.",
        }),
        a11y: &[
            "L'area di saturazione ha role=slider con aria-valuetext; tinta, canali e alpha sono input range nativi con aria-label.",
            "Il campo del codice accetta hex, rgb() e hsl() digitati.",
            "ColorInput apre il pannello con aria-haspopup=dialog e aria-expanded.",
            "I campioni sono pulsanti con aria-label che riporta il colore.",
        ],
        related: &["inputs", "segmented-controls", "slider", "form"],
    }
}
