use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Listens for click or pointerdown events occurring outside of the element identified by `element_id`.
pub fn use_click_outside(
    element_id: impl Into<String>,
    on_click_outside: impl FnMut() + 'static,
) {
    let el_id = element_id.into();
    let cb = use_hook(|| Rc::new(RefCell::new(on_click_outside)));
    let listener_key = crate::a11y::use_unique_id("sd-click-outside", None);

    let key_for_effect = listener_key.clone();
    let el_id_clone = el_id.clone();
    let cb_clone = Rc::clone(&cb);

    use_effect(move || {
        let js = format!(
            r#"
            (function() {{
                const key = "{key_for_effect}";
                window._sd_co = window._sd_co || {{}};
                if (window._sd_co[key]) {{
                    document.removeEventListener("pointerdown", window._sd_co[key], true);
                }}
                window._sd_co[key] = function(e) {{
                    const el = document.getElementById("{el_id_clone}");
                    if (el && !el.contains(e.target)) {{
                        dioxus.send(true);
                    }}
                }};
                document.addEventListener("pointerdown", window._sd_co[key], true);
            }})();
            "#
        );

        let cb_inner = Rc::clone(&cb_clone);
        spawn(async move {
            let mut eval = document::eval(&js);
            while eval.recv::<serde_json::Value>().await.is_ok() {
                if let Ok(mut f) = cb_inner.try_borrow_mut() {
                    f();
                }
            }
        });
    });

    let key_for_drop = listener_key;
    use_drop(move || {
        let _ = document::eval(&format!(
            r#"
            if (window._sd_co && window._sd_co["{key_for_drop}"]) {{
                document.removeEventListener("pointerdown", window._sd_co["{key_for_drop}"], true);
                delete window._sd_co["{key_for_drop}"];
            }}
            "#
        ));
    });
}
