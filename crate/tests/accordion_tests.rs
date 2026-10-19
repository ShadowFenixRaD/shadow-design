use dioxus::prelude::*;
use shadow_design::prelude::*;

#[test]
fn test_accordion_variant_classes() {
    assert_eq!(AccordionVariant::Default.as_str(), "sd-accordion--default");
    assert_eq!(AccordionVariant::Contained.as_str(), "sd-accordion--contained");
    assert_eq!(AccordionVariant::Separated.as_str(), "sd-accordion--separated");
}

#[test]
fn test_accordion_single_selection() {
    let mut dom = VirtualDom::new(|| {
        let val = use_signal(|| vec!["first".to_string()]);
        let mult = use_signal(|| false);
        let mut ctx = AccordionContext {
            multiple: mult,
            value: val,
            variant: AccordionVariant::Default,
            base_id: "test-acc".into(),
        };

        assert!(ctx.is_expanded("first"));
        assert!(!ctx.is_expanded("second"));

        // Expanding "second" closes "first"
        ctx.toggle("second");
        assert!(!ctx.is_expanded("first"));
        assert!(ctx.is_expanded("second"));

        // Collapsing "second" leaves nothing open
        ctx.toggle("second");
        assert!(!ctx.is_expanded("second"));

        rsx! { div {} }
    });
    dom.rebuild_in_place();
}

#[test]
fn test_accordion_multiple_selection() {
    let mut dom = VirtualDom::new(|| {
        let val = use_signal(|| vec!["alpha".to_string()]);
        let mult = use_signal(|| true);
        let mut ctx = AccordionContext {
            multiple: mult,
            value: val,
            variant: AccordionVariant::Separated,
            base_id: "test-multi".into(),
        };

        assert!(ctx.is_expanded("alpha"));
        assert!(!ctx.is_expanded("beta"));

        // Expanding "beta" keeps "alpha"
        ctx.toggle("beta");
        assert!(ctx.is_expanded("alpha"));
        assert!(ctx.is_expanded("beta"));

        // Collapsing "alpha" leaves "beta" open
        ctx.toggle("alpha");
        assert!(!ctx.is_expanded("alpha"));
        assert!(ctx.is_expanded("beta"));

        rsx! { div {} }
    });
    dom.rebuild_in_place();
}

#[test]
fn test_accordion_component_interaction() {
    let mut dom = VirtualDom::new(|| {
        let active = use_signal(|| vec!["item-1".to_string()]);
        rsx! {
            Accordion {
                value: Some(active),
                AccordionItem { value: "item-1".to_string(),
                    AccordionControl { "Header 1" }
                    AccordionPanel { "Content 1" }
                }
                AccordionItem { value: "item-2".to_string(),
                    AccordionControl { "Header 2" }
                    AccordionPanel { "Content 2" }
                }
            }
        }
    });
    dom.rebuild_in_place();
}
