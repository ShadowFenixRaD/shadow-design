use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Modal alert dialog for critical confirmations and destructive actions.
#[component]
pub fn AlertDialog(
    /// Signal controlling the visibility of the dialog.
    opened: Signal<bool>,
    /// Dialog title.
    title: String,
    /// Detailed description or confirmation message.
    description: String,
    /// Label for the confirmation button. Defaults to the locale `confirm` string.
    #[props(default)]
    confirm_label: String,
    /// Label for the cancellation button. Defaults to the locale `cancel` string.
    #[props(default)]
    cancel_label: String,
    /// Whether the action is destructive (renders danger styling and crimson glow). Defaults to true.
    #[props(default = true)]
    is_destructive: bool,
    /// Optional custom icon. Defaults to [`IconKind::AlertTriangle`].
    #[props(default)]
    icon: Option<IconKind>,
    /// Callback triggered when the confirm button is pressed.
    #[props(default)]
    on_confirm: Option<EventHandler<()>>,
    /// Callback triggered when the cancel button or backdrop is clicked.
    #[props(default)]
    on_cancel: Option<EventHandler<()>>,
    /// Perimetric optical glow.
    #[props(default)]
    glow: Option<bool>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Optional supplementary content rendered between description and actions.
    #[props(default)]
    children: Option<Element>,
) -> Element {
    let locale = crate::locale::use_locale();
    let confirm_label = if confirm_label.is_empty() { locale.confirm.clone() } else { confirm_label };
    let cancel_label = if cancel_label.is_empty() { locale.cancel.clone() } else { cancel_label };
    let is_open = *opened.read();

    if !is_open {
        return rsx! {};
    }

    let dialog_icon = icon.unwrap_or(IconKind::AlertTriangle);

    let glow_class = if glow == Some(true) || (glow.is_none() && is_destructive) {
        "sd-alert-dialog--glow"
    } else {
        ""
    };

    let icon_variant_class = if is_destructive {
        "sd-alert-dialog-icon--destructive"
    } else {
        "sd-alert-dialog-icon--primary"
    };

    let handle_confirm = move |_| {
        *opened.write() = false;
        if let Some(h) = on_confirm {
            h.call(());
        }
    };

    let handle_cancel = move |_| {
        *opened.write() = false;
        if let Some(h) = on_cancel {
            h.call(());
        }
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("alert_dialog").unwrap_or("");
    #[cfg(not(feature = "theme"))]
    let base_override = "";

    rsx! {
        div {
            class: "sd-alert-dialog-backdrop",
            onclick: handle_cancel,
            div {
                role: "alertdialog",
                "aria-modal": "true",
                class: "sd-alert-dialog {glow_class} {base_override} {class}",
                onclick: move |e| e.stop_propagation(),
                div { class: "sd-alert-dialog-icon {icon_variant_class}",
                    Icon { kind: dialog_icon, size: 24 }
                }
                h3 { class: "sd-alert-dialog-title", "{title}" }
                p { class: "sd-alert-dialog-desc", "{description}" }

                if let Some(ch) = children {
                    div { class: "mb-6", {ch} }
                }

                div { class: "sd-alert-dialog-actions",
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        onclick: handle_cancel,
                        "{cancel_label}"
                    }
                    Button {
                        variant: if is_destructive { ButtonVariant::Danger } else { ButtonVariant::Primary },
                        size: ButtonSize::Sm,
                        onclick: handle_confirm,
                        "{confirm_label}"
                    }
                }
            }
        }
    }
}

/// Convenience alias for [`AlertDialog`].
#[component]
pub fn ConfirmDialog(
    opened: Signal<bool>,
    title: String,
    description: String,
    #[props(default)] confirm_label: String,
    #[props(default)] cancel_label: String,
    #[props(default = false)] is_destructive: bool,
    #[props(default)] icon: Option<IconKind>,
    #[props(default)] on_confirm: Option<EventHandler<()>>,
    #[props(default)] on_cancel: Option<EventHandler<()>>,
    #[props(default)] glow: Option<bool>,
    #[props(default)] class: String,
    #[props(default)] children: Option<Element>,
) -> Element {
    rsx! {
        AlertDialog {
            opened,
            title,
            description,
            confirm_label,
            cancel_label,
            is_destructive,
            icon,
            on_confirm,
            on_cancel,
            glow,
            class,
            children,
        }
    }
}
