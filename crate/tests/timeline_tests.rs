#![cfg(feature = "timeline")]

use shadow_design::prelude::*;

#[test]
fn test_timeline_align_classes() {
    assert_eq!(TimelineAlign::Left.scss_class(), "sd-timeline--align-left");
    assert_eq!(TimelineAlign::Right.scss_class(), "sd-timeline--align-right");
}

#[test]
fn test_timeline_line_variant_classes() {
    assert_eq!(TimelineLineVariant::Solid.scss_class(), "sd-timeline--line-solid");
    assert_eq!(TimelineLineVariant::Dashed.scss_class(), "sd-timeline--line-dashed");
}

#[test]
fn test_timeline_bullet_state_classes() {
    assert_eq!(TimelineBulletState::Default.scss_class(), "sd-timeline-item--state-default");
    assert_eq!(TimelineBulletState::Active.scss_class(), "sd-timeline-item--state-active");
    assert_eq!(TimelineBulletState::Completed.scss_class(), "sd-timeline-item--state-completed");
    assert_eq!(TimelineBulletState::Pending.scss_class(), "sd-timeline-item--state-pending");
    assert_eq!(TimelineBulletState::Error.scss_class(), "sd-timeline-item--state-error");
}

#[test]
fn test_timeline_defaults() {
    assert_eq!(TimelineAlign::default(), TimelineAlign::Left);
    assert_eq!(TimelineLineVariant::default(), TimelineLineVariant::Solid);
    assert_eq!(TimelineBulletState::default(), TimelineBulletState::Default);
}
