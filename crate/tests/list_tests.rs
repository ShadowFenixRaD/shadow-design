use shadow_design::prelude::*;

#[test]
fn test_list_marker_classes() {
    assert_eq!(ListMarker::Disc.scss_class(), "sd-list--marker-disc");
    assert_eq!(ListMarker::Circle.scss_class(), "sd-list--marker-circle");
    assert_eq!(ListMarker::Square.scss_class(), "sd-list--marker-square");
    assert_eq!(ListMarker::Decimal.scss_class(), "sd-list--marker-decimal");
    assert_eq!(
        ListMarker::LowerAlpha.scss_class(),
        "sd-list--marker-lower-alpha"
    );
    assert_eq!(
        ListMarker::UpperAlpha.scss_class(),
        "sd-list--marker-upper-alpha"
    );
    assert_eq!(
        ListMarker::LowerRoman.scss_class(),
        "sd-list--marker-lower-roman"
    );
    assert_eq!(
        ListMarker::UpperRoman.scss_class(),
        "sd-list--marker-upper-roman"
    );
    assert_eq!(ListMarker::None.scss_class(), "sd-list--marker-none");
    assert_eq!(ListMarker::Custom.scss_class(), "sd-list--marker-custom");
}

#[test]
fn test_list_spacing_classes() {
    assert_eq!(ListSpacing::Xs.scss_class(), "sd-list--spacing-xs");
    assert_eq!(ListSpacing::Sm.scss_class(), "sd-list--spacing-sm");
    assert_eq!(ListSpacing::Md.scss_class(), "sd-list--spacing-md");
    assert_eq!(ListSpacing::Lg.scss_class(), "sd-list--spacing-lg");
    assert_eq!(ListSpacing::Xl.scss_class(), "sd-list--spacing-xl");
}

#[test]
fn test_list_size_classes() {
    assert_eq!(ListSize::Xs.scss_class(), "sd-list--size-xs");
    assert_eq!(ListSize::Sm.scss_class(), "sd-list--size-sm");
    assert_eq!(ListSize::Md.scss_class(), "sd-list--size-md");
    assert_eq!(ListSize::Lg.scss_class(), "sd-list--size-lg");
    assert_eq!(ListSize::Xl.scss_class(), "sd-list--size-xl");
}

#[test]
fn test_list_orientation_classes() {
    assert_eq!(ListOrientation::Vertical.scss_class(), "sd-list--vertical");
    assert_eq!(
        ListOrientation::Horizontal.scss_class(),
        "sd-list--horizontal"
    );
}

#[test]
fn test_list_defaults() {
    assert_eq!(ListType::default(), ListType::Unordered);
    assert_eq!(ListMarker::default(), ListMarker::Disc);
    assert_eq!(ListSpacing::default(), ListSpacing::Sm);
    assert_eq!(ListSize::default(), ListSize::Md);
    assert_eq!(ListOrientation::default(), ListOrientation::Vertical);
}
