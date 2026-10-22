use shadow_design::prelude::*;

#[test]
fn test_burger_size_classes() {
    assert_eq!(BurgerSize::Sm.class_name(), "sd-burger--sm");
    assert_eq!(BurgerSize::Md.class_name(), "sd-burger--md");
    assert_eq!(BurgerSize::Lg.class_name(), "sd-burger--lg");
}

#[test]
fn test_close_button_sizes_and_variants() {
    assert_eq!(CloseButtonSize::Sm.class_name(), "sd-close-btn--sm");
    assert_eq!(CloseButtonSize::Md.class_name(), "sd-close-btn--md");
    assert_eq!(CloseButtonSize::Lg.class_name(), "sd-close-btn--lg");

    assert_eq!(CloseButtonSize::Sm.icon_size(), 14);
    assert_eq!(CloseButtonSize::Md.icon_size(), 16);
    assert_eq!(CloseButtonSize::Lg.icon_size(), 20);

    assert_eq!(CloseButtonVariant::Subtle.class_name(), "sd-close-btn--subtle");
    assert_eq!(CloseButtonVariant::Filled.class_name(), "sd-close-btn--filled");
    assert_eq!(CloseButtonVariant::Transparent.class_name(), "sd-close-btn--transparent");
}

#[test]
fn test_indicator_positions_and_colors() {
    assert_eq!(IndicatorPosition::TopEnd.class_name(), "sd-indicator-badge--top-end");
    assert_eq!(IndicatorPosition::TopStart.class_name(), "sd-indicator-badge--top-start");
    assert_eq!(IndicatorPosition::BottomEnd.class_name(), "sd-indicator-badge--bottom-end");
    assert_eq!(IndicatorPosition::BottomStart.class_name(), "sd-indicator-badge--bottom-start");

    assert_eq!(IndicatorColor::Violet.class_name(), "sd-indicator-badge--violet");
    assert_eq!(IndicatorColor::Emerald.class_name(), "sd-indicator-badge--emerald");
    assert_eq!(IndicatorColor::Rose.class_name(), "sd-indicator-badge--rose");
    assert_eq!(IndicatorColor::Amber.class_name(), "sd-indicator-badge--amber");
    assert_eq!(IndicatorColor::Cyan.class_name(), "sd-indicator-badge--cyan");

    assert_eq!(IndicatorSize::Sm.class_name(), "sd-indicator-badge--sm");
    assert_eq!(IndicatorSize::Md.class_name(), "sd-indicator-badge--md");
    assert_eq!(IndicatorSize::Lg.class_name(), "sd-indicator-badge--lg");
}

#[test]
fn test_fieldset_variants() {
    assert_eq!(FieldsetVariant::Default.class_name(), "sd-fieldset--default");
    assert_eq!(FieldsetVariant::Filled.class_name(), "sd-fieldset--filled");
    assert_eq!(FieldsetVariant::Ungrouped.class_name(), "sd-fieldset--ungrouped");
}

#[test]
fn test_marquee_rect_geometry() {
    let rect = MarqueeRect {
        x: 10.0,
        y: 20.0,
        width: 100.0,
        height: 80.0,
    };
    assert_eq!(rect.x, 10.0);
    assert_eq!(rect.y, 20.0);
    assert_eq!(rect.width, 100.0);
    assert_eq!(rect.height, 80.0);
}
