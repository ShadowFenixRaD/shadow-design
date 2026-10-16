use dioxus::prelude::*;
use shadow_design::prelude::*;

#[test]
fn test_spoiler_toggle_state() {
    let mut dom = VirtualDom::new(|| {
        let mut is_expanded = use_signal(|| false);

        assert!(!*is_expanded.read());
        is_expanded.set(true);
        assert!(*is_expanded.read());

        rsx! {
            Spoiler {
                max_height: 100,
                show_label: "More".to_string(),
                hide_label: "Less".to_string(),
                expanded: Some(is_expanded),
                p { "Hidden contents" }
            }
        }
    });
    dom.rebuild_in_place();
}
