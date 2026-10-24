//! Story schema (ADR-0006). Data only: no hooks, no rendering here.
//!
//! Every component of the library is documented by exactly one [`Story`] value,
//! declared in `showcase/src/stories/<slug>.rs`. The template in `story/page.rs`
//! renders every story with the same anatomy.
// Shared vocabulary for all 85 stories: while stories are still stubs, some variants and
// constructors (Number, Color, Stage::Fill, ...) have no caller yet. Phase 3 removes this.
#![allow(dead_code)]

use dioxus::prelude::*;
use shadow_design::prelude::IconKind;

pub use crate::story::knobs::Knobs;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Category {
    Fondamenta,
    Form,
    Layout,
    Navigazione,
    Overlay,
    Dati,
    Contenuti,
    Utility,
}

impl Category {
    pub const ALL: [Category; 8] = [
        Category::Fondamenta,
        Category::Form,
        Category::Layout,
        Category::Navigazione,
        Category::Overlay,
        Category::Dati,
        Category::Contenuti,
        Category::Utility,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Category::Fondamenta => "Fondamenta",
            Category::Form => "Azioni & Form",
            Category::Layout => "Layout",
            Category::Navigazione => "Navigazione",
            Category::Overlay => "Overlay",
            Category::Dati => "Dati & Feedback",
            Category::Contenuti => "Contenuti & Social",
            Category::Utility => "Utility",
        }
    }

    pub fn icon(self) -> IconKind {
        match self {
            Category::Fondamenta => IconKind::Sparkles,
            Category::Form => IconKind::Terminal,
            Category::Layout => IconKind::Box,
            Category::Navigazione => IconKind::ChevronRight,
            Category::Overlay => IconKind::Layers,
            Category::Dati => IconKind::Info,
            Category::Contenuti => IconKind::MessageSquare,
            Category::Utility => IconKind::Settings,
        }
    }

    pub fn blurb(self) -> &'static str {
        match self {
            Category::Fondamenta => "Tema, token, tipografia e icone.",
            Category::Form => "Pulsanti e campi: tutto ciò che riceve input.",
            Category::Layout => "Shell, griglie, stack e contenitori.",
            Category::Navigazione => "Tab, breadcrumb, paginazione, menu e palette.",
            Category::Overlay => "Finestre, pannelli e menu flottanti.",
            Category::Dati => "Tabelle, liste, stati e grafici.",
            Category::Contenuti => "Avatar, chat, feed e schede media.",
            Category::Utility => "Micro-componenti e hook reattivi.",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Stable,
    Beta,
    Stub,
}

/// Runtime value of one knob.
#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Bool(bool),
    Choice(usize),
    Text(String),
    Number(f64),
    Color(String),
}

/// One option of a `Kind::Choice`: what the user sees and what the code sample prints.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ChoiceOption {
    pub label: &'static str,
    pub code: &'static str,
}

pub const fn opt(label: &'static str, code: &'static str) -> ChoiceOption {
    ChoiceOption { label, code }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    Bool,
    Choice(&'static [ChoiceOption]),
    Text,
    /// `decimals` controls the printed literal: 0 → `20`, 1 → `1.0`.
    Number { min: f64, max: f64, step: f64, decimals: u8 },
    Color,
}

/// How a knob appears in the generated code.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Emit {
    /// `key: <value>,` on every render.
    Always,
    /// `key: <value>,` only when the value differs from the default.
    IfChanged,
    /// A template line, emitted while the knob is "on": a `Bool` when true, a `Text`/`Color`
    /// when non-empty, a `Choice`/`Number` when it differs from the default. Use it for
    /// `Option<_>` props and icons. `{v}` is replaced by the raw value (text unquoted,
    /// choice code, number, bool).
    Template(&'static str),
    /// Never printed: the knob only shapes the demo (stage width, sample text...).
    Skip,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Control {
    pub key: &'static str,
    pub label: &'static str,
    pub kind: Kind,
    pub default: Value,
    pub emit: Emit,
    pub hint: Option<&'static str>,
}

impl Control {
    pub fn bool(key: &'static str, label: &'static str, default: bool) -> Self {
        Self { key, label, kind: Kind::Bool, default: Value::Bool(default), emit: Emit::IfChanged, hint: None }
    }

    pub fn choice(key: &'static str, label: &'static str, options: &'static [ChoiceOption], default: usize) -> Self {
        Self { key, label, kind: Kind::Choice(options), default: Value::Choice(default), emit: Emit::IfChanged, hint: None }
    }

    pub fn text(key: &'static str, label: &'static str, default: &str) -> Self {
        Self { key, label, kind: Kind::Text, default: Value::Text(default.to_string()), emit: Emit::IfChanged, hint: None }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn number(key: &'static str, label: &'static str, default: f64, min: f64, max: f64, step: f64, decimals: u8) -> Self {
        Self {
            key,
            label,
            kind: Kind::Number { min, max, step, decimals },
            default: Value::Number(default),
            emit: Emit::IfChanged,
            hint: None,
        }
    }

    pub fn color(key: &'static str, label: &'static str, default: &str) -> Self {
        Self { key, label, kind: Kind::Color, default: Value::Color(default.to_string()), emit: Emit::IfChanged, hint: None }
    }

    #[must_use]
    pub fn emit(mut self, emit: Emit) -> Self {
        self.emit = emit;
        self
    }

    #[must_use]
    pub fn hint(mut self, hint: &'static str) -> Self {
        self.hint = Some(hint);
        self
    }
}

/// How the live component sits in the stage.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Stage {
    /// Centred, content-sized (buttons, badges, switches).
    #[default]
    Center,
    /// Top-left, full width (lists, tables, navigation).
    Start,
    /// A centred 24rem column (fields, selects, cards).
    Form,
    /// Full width and at least 20rem tall (layout and overlay demos).
    Fill,
}

/// Live component plus its knobs. `render` is a plain function, not a component:
/// stateful demos go through a private `#[component] fn XDemo(knobs: Knobs)` (REVAMP §3.2).
#[derive(Clone)]
pub struct Playground {
    /// Rust component name printed as the block head: `Button {`.
    pub component: &'static str,
    pub controls: Vec<Control>,
    pub render: fn(&Knobs) -> Element,
    /// Lines printed before the block (signals, data). Verbatim.
    pub prelude_code: Option<&'static str>,
    /// Lines printed first inside the block, always (e.g. `data: data.clone(),`, `value,`).
    pub fixed_code: &'static [&'static str],
    /// Children printed last inside the block. `{key}` placeholders take the raw knob value.
    pub children_code: Option<&'static str>,
    pub stage: Stage,
}

// Stories are static data: two playgrounds are equal when they describe the same
// component and code. Function pointers are compared by address, never by value.
impl PartialEq for Playground {
    fn eq(&self, other: &Self) -> bool {
        self.component == other.component
            && self.controls == other.controls
            && std::ptr::fn_addr_eq(self.render, other.render)
            && self.prelude_code == other.prelude_code
            && self.fixed_code == other.fixed_code
            && self.children_code == other.children_code
            && self.stage == other.stage
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Span {
    #[default]
    Half,
    Full,
}

#[derive(Clone)]
pub struct Example {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    /// Must be the exact RSX of `render`'s body (migration checklist §5, item 6).
    pub code: &'static str,
    pub render: fn() -> Element,
    pub span: Span,
}

impl PartialEq for Example {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.title == other.title
            && self.description == other.description
            && self.code == other.code
            && std::ptr::fn_addr_eq(self.render, other.render)
            && self.span == other.span
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PropDoc {
    pub name: &'static str,
    pub ty: &'static str,
    pub default: &'static str,
    pub doc: &'static str,
}

pub const fn prop(name: &'static str, ty: &'static str, default: &'static str, doc: &'static str) -> PropDoc {
    PropDoc { name, ty, default, doc }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PropGroup {
    pub component: &'static str,
    pub rows: &'static [PropDoc],
}

/// Which Lit Glass recipe the component implements (docs/design/language.md §3).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Recipe {
    /// e.g. "Raised control (§3.5)"
    pub recipe: &'static str,
    /// e.g. "Lit control" (depth ladder §2)
    pub depth: &'static str,
    pub tokens: &'static [&'static str],
    pub notes: &'static str,
}

#[derive(Clone, PartialEq)]
pub struct Story {
    /// URL segment, kebab-case, equal to the old route path.
    pub slug: &'static str,
    /// Rust component name.
    pub name: &'static str,
    pub category: Category,
    pub features: &'static [&'static str],
    pub icon: IconKind,
    /// One Italian sentence.
    pub tagline: &'static str,
    pub keywords: &'static [&'static str],
    pub status: Status,
    pub playground: Option<Playground>,
    pub examples: Vec<Example>,
    pub props: Vec<PropGroup>,
    pub recipe: Option<Recipe>,
    pub a11y: &'static [&'static str],
    /// Slugs of related stories.
    pub related: &'static [&'static str],
}

impl Story {
    /// Phase-1 placeholder. Renders head + "Storia in migrazione".
    pub fn stub(
        slug: &'static str,
        name: &'static str,
        category: Category,
        features: &'static [&'static str],
        icon: IconKind,
        tagline: &'static str,
    ) -> Self {
        Self {
            slug,
            name,
            category,
            features,
            icon,
            tagline,
            keywords: &[],
            status: Status::Stub,
            playground: None,
            examples: Vec::new(),
            props: Vec::new(),
            recipe: None,
            a11y: &[],
            related: &[],
        }
    }

    /// The Cargo dependency line for exactly this story's features.
    pub fn cargo_line(&self) -> String {
        let feats = self.features.iter().map(|f| format!("\"{f}\"")).collect::<Vec<_>>().join(", ");
        format!(
            "shadow-design = {{ version = \"{}\", default-features = false, features = [{feats}] }}",
            shadow_design::VERSION
        )
    }
}
