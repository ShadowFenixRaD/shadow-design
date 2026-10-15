use dioxus::prelude::*;
use crate::a11y::ControlSize;
use crate::icon::IconKind;

/// Visual variant styles for [`Chip`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChipVariant {
    /// Solid filled accent background with micro-glow when active.
    #[default]
    Filled,
    /// Translucent surface with illuminated active border.
    Outline,
    /// Subtle accent background tint.
    Light,
    /// Minimalist ghost styling without default borders.
    Subtle,
}

/// Shared context provided by a parent [`ChipGroup`].
#[derive(Clone, Copy, PartialEq)]
pub struct ChipGroupContext {
    /// Reactive list of currently selected string values.
    pub values: Signal<Vec<String>>,
    /// Whether multiple chips can be selected concurrently.
    pub multiple: bool,
    /// Shared size modifier inherited by child chips.
    pub size: Option<ControlSize>,
    /// Shared variant inherited by child chips.
    pub variant: Option<ChipVariant>,
}

/// Properties for configuring an individual [`Chip`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ChipProps {
    /// Reactive boolean signal binding the checked state of a standalone chip.
    #[props(default)]
    pub checked: Option<Signal<bool>>,
    /// Default checked state if no reactive signal is provided.
    #[props(default = false)]
    pub default_checked: bool,
    /// String identifier for this chip when used within a [`ChipGroup`].
    #[props(default)]
    pub value: Option<String>,
    /// Visual style variant (Filled, Outline, Light, Subtle).
    #[props(default = ChipVariant::Filled)]
    pub variant: ChipVariant,
    /// Size scaling (Sm, Md, Lg).
    #[props(default = ControlSize::Md)]
    pub size: ControlSize,
    /// Optional leading icon (SVG native).
    #[props(default)]
    pub icon: Option<IconKind>,
    /// Whether to show a checkmark icon when the chip is active.
    #[props(default = true)]
    pub show_check: bool,
    /// Whether the chip is disabled and non-interactive.
    #[props(default = false)]
    pub disabled: bool,
    /// Optional additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
    /// Children elements (usually label text).
    pub children: Element,
    /// Click event handler.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    /// Toggle state change event handler.
    #[props(default)]
    pub on_toggle: Option<EventHandler<bool>>,
}

/// Properties for configuring a [`ChipGroup`] container.
#[derive(Props, Clone, PartialEq)]
pub struct ChipGroupProps {
    /// Reactive signal containing the list of currently selected chip values.
    pub value: Signal<Vec<String>>,
    /// Whether multiple chips can be selected simultaneously. If false, single-select mode.
    #[props(default = false)]
    pub multiple: bool,
    /// Shared size applied to all child chips if not overridden.
    #[props(default)]
    pub size: Option<ControlSize>,
    /// Shared variant applied to all child chips if not overridden.
    #[props(default)]
    pub variant: Option<ChipVariant>,
    /// Optional additional CSS classes for the group container.
    #[props(default)]
    pub class: Option<String>,
    /// Group children containing [`Chip`] items.
    pub children: Element,
    /// Callback triggered whenever the selection changes.
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<String>>>,
}
