use crate::icon::{Icon, IconKind};
use crate::util::sleep_ms;
use dioxus::prelude::*;

/// Standalone copy-to-clipboard button with visual confirmation feedback.
#[component]
pub fn CopyButton(
    /// Value to copy to the system clipboard.
    value: String,
    /// Duration in milliseconds for the copied feedback state. Defaults to 2000ms.
    #[props(default = 2000)]
    timeout_ms: u32,
    /// Optional label displayed next to the icon before copying.
    #[props(default)]
    label: Option<String>,
    /// Optional label displayed during copied confirmation. Defaults to the locale `copied` string.
    #[props(default)]
    copied_label: String,
    /// Whether to only show an icon without text label. Defaults to false.
    #[props(default = false)]
    icon_only: bool,
    /// Accessible title / aria-label. Defaults to the locale `copy_to_clipboard` string.
    #[props(default)]
    aria_label: String,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let copied_label = if copied_label.is_empty() { locale.copied.clone() } else { copied_label };
    let aria_label = if aria_label.is_empty() { locale.copy_to_clipboard.clone() } else { aria_label };
    let mut is_copied = use_signal(|| false);

    let val_to_copy = value.clone();
    let handle_copy = move |e: MouseEvent| {
        e.stop_propagation();
        let text = val_to_copy.clone();
        spawn(async move {
            let eval = document::eval(
                r#"
                const text = await dioxus.recv();
                if (navigator.clipboard && navigator.clipboard.writeText) {
                    await navigator.clipboard.writeText(text);
                } else {
                    const el = document.createElement("textarea");
                    el.value = text;
                    document.body.appendChild(el);
                    el.select();
                    document.execCommand("copy");
                    document.body.removeChild(el);
                }
                "#,
            );
            let _ = eval.send(text);
            let _ = eval.await;
            is_copied.set(true);
            sleep_ms(timeout_ms as u64).await;
            is_copied.set(false);
        });
    };

    let copied = *is_copied.read();
    let copied_class = if copied { "sd-copy-btn--copied" } else { "" };
    let icon_only_class = if icon_only { "sd-copy-btn--icon-only" } else { "" };

    rsx! {
        button {
            r#type: "button",
            class: "sd-copy-btn {copied_class} {icon_only_class} {class}",
            title: if copied { &copied_label } else { &aria_label },
            "aria-label": if copied { &copied_label } else { &aria_label },
            onclick: handle_copy,
            if copied {
                Icon { kind: IconKind::Check, size: 15 }
                if !icon_only {
                    span { class: "sd-copy-btn-label", "{copied_label}" }
                }
            } else {
                Icon { kind: IconKind::Copy, size: 15 }
                if let Some(lbl) = label {
                    if !icon_only {
                        span { class: "sd-copy-btn-label", "{lbl}" }
                    }
                }
            }
        }
    }
}
