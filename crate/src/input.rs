use dioxus::prelude::*;

#[component]
pub fn TextInput(
    value: Signal<String>,
    #[props(default)] label: Option<String>,
    #[props(default)] placeholder: Option<String>,
    #[props(default)] help_text: Option<String>,
    #[props(default)] error: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] input_type: Option<String>,
    #[props(default)] class: String,
) -> Element {
    let itype = input_type.unwrap_or_else(|| "text".to_string());
    let border_color = if error.is_some() {
        "border-rose-500 focus:border-rose-400 focus:ring-rose-500/20"
    } else {
        "border-zinc-700 focus:border-blue-500 focus:ring-blue-500/20"
    };

    rsx! {
        div {
            class: "flex flex-col gap-1.5 {class}",
            if let Some(lbl) = label {
                label { class: "text-sm font-medium text-zinc-300", "{lbl}" }
            }
            input {
                r#type: "{itype}",
                value: "{value}",
                placeholder: placeholder.as_deref().unwrap_or_default(),
                disabled,
                class: "px-3.5 py-2 bg-zinc-900 text-zinc-100 rounded-lg border {border_color} focus:outline-none focus:ring-2 transition-all placeholder:text-zinc-500 disabled:opacity-50 disabled:cursor-not-allowed text-sm shadow-inner",
                oninput: move |e| {
                    *value.write() = e.value();
                },
            }
            if let Some(err) = error {
                span { class: "text-xs text-rose-400 mt-0.5", "{err}" }
            } else if let Some(help) = help_text {
                span { class: "text-xs text-zinc-500 mt-0.5", "{help}" }
            }
        }
    }
}
