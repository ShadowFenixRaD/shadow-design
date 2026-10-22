use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Listens for click or pointerdown events occurring outside of the element identified by `element_id`.
pub fn use_click_outside(
    element_id: &'static str,
    on_click_outside: impl FnMut() + 'static,
) {
    let cb = use_hook(|| Rc::new(RefCell::new(on_click_outside)));

    use_hook(|| {
        let js = format!(
            r#"
            (function() {{
                function handler(e) {{
                    const el = document.getElementById("{element_id}");
                    if (el && !el.contains(e.target)) {{
                        dioxus.send(true);
                    }}
                }}
                document.addEventListener("pointerdown", handler, true);
            }})();
            "#
        );

        let cb_clone = Rc::clone(&cb);
        spawn(async move {
            let mut eval = document::eval(&js);
            while let Ok(_) = eval.recv::<serde_json::Value>().await {
                if let Ok(mut f) = cb_clone.try_borrow_mut() {
                    f();
                }
            }
        });
    });
}
