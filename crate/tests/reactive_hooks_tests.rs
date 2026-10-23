#![cfg(feature = "hooks")]

use dioxus::prelude::*;
use shadow_design::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug, Default)]
struct TestFormModel {
    username: String,
    email: String,
    age: u32,
}

#[test]
fn test_form_validation_and_errors() {
    let mut dom = VirtualDom::new(|| {
        let form = use_form(
            TestFormModel {
                username: "".to_string(),
                email: "invalid-email".to_string(),
                age: 15,
            },
            Some(|values: &TestFormModel| {
                let mut errors = HashMap::new();
                if values.username.trim().is_empty() {
                    errors.insert("username".to_string(), "Username obbligatorio".to_string());
                }
                if !values.email.contains('@') {
                    errors.insert("email".to_string(), "Email non valida".to_string());
                }
                if values.age < 18 {
                    errors.insert("age".to_string(), "Devi essere maggiorenne".to_string());
                }
                errors
            }),
        );

        // Validation test
        let is_valid = form.validate();
        assert!(!is_valid, "Form should be invalid initially");
        assert_eq!(form.get_error("username").as_deref(), Some("Username obbligatorio"));
        assert_eq!(form.get_error("email").as_deref(), Some("Email non valida"));
        assert_eq!(form.get_error("age").as_deref(), Some("Devi essere maggiorenne"));

        // Manual error modification
        form.set_error("custom", "Errore manuale");
        assert_eq!(form.get_error("custom").as_deref(), Some("Errore manuale"));
        form.clear_error("custom");
        assert_eq!(form.get_error("custom"), None);

        // Update values to valid
        form.set_values(TestFormModel {
            username: "shadow_coder".to_string(),
            email: "coder@shadowfenix.net".to_string(),
            age: 25,
        });

        assert!(*form.is_dirty.read(), "Form should be dirty after set_values");
        let valid_now = form.validate();
        assert!(valid_now, "Form should be valid after updates");
        assert!(form.is_valid());

        // Submission helper test
        let mut submitted = false;
        let ok = form.submit(|vals| {
            assert_eq!(vals.username, "shadow_coder");
            submitted = true;
        });
        assert!(ok);
        assert!(submitted);

        // Reset test
        form.reset();
        assert!(!*form.is_dirty.read(), "Form should not be dirty after reset");
        assert_eq!(form.read().username, "");

        rsx! { div { "Form test passed" } }
    });

    dom.rebuild_in_place();
}

#[test]
fn test_clipboard_initial_state() {
    let mut dom = VirtualDom::new(|| {
        let cb = use_clipboard(Some(1500));
        assert!(!*cb.copied.read(), "Copied should be false initially");
        assert!(!*cb.error.read(), "Error should be false initially");

        rsx! { div { "Clipboard test passed" } }
    });

    dom.rebuild_in_place();
}

#[test]
fn test_debounced_signal_initial_value() {
    let mut dom = VirtualDom::new(|| {
        let sig = use_signal(|| "search_term".to_string());
        let debounced = use_debounced_signal(sig, 200);

        assert_eq!(*debounced.read(), "search_term");

        rsx! { div { "Debounce test passed" } }
    });

    dom.rebuild_in_place();
}

#[test]
fn test_window_scroll_initial_value() {
    let mut dom = VirtualDom::new(|| {
        let scroll = use_window_scroll();
        let (x, y) = *scroll.position.read();
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);

        rsx! { div { "Scroll test passed" } }
    });

    dom.rebuild_in_place();
}
