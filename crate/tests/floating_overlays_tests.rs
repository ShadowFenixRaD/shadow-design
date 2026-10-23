#![cfg(feature = "all")]

use shadow_design::prelude::*;

#[test]
fn test_popover_position_classes() {
    assert_eq!(PopoverPosition::Top.class_name(), "sd-popover-dropdown--top");
    assert_eq!(PopoverPosition::TopStart.class_name(), "sd-popover-dropdown--top-start");
    assert_eq!(PopoverPosition::TopEnd.class_name(), "sd-popover-dropdown--top-end");
    assert_eq!(PopoverPosition::Bottom.class_name(), "sd-popover-dropdown--bottom");
    assert_eq!(PopoverPosition::BottomStart.class_name(), "sd-popover-dropdown--bottom-start");
    assert_eq!(PopoverPosition::BottomEnd.class_name(), "sd-popover-dropdown--bottom-end");
    assert_eq!(PopoverPosition::Left.class_name(), "sd-popover-dropdown--left");
    assert_eq!(PopoverPosition::LeftStart.class_name(), "sd-popover-dropdown--left-start");
    assert_eq!(PopoverPosition::LeftEnd.class_name(), "sd-popover-dropdown--left-end");
    assert_eq!(PopoverPosition::Right.class_name(), "sd-popover-dropdown--right");
    assert_eq!(PopoverPosition::RightStart.class_name(), "sd-popover-dropdown--right-start");
    assert_eq!(PopoverPosition::RightEnd.class_name(), "sd-popover-dropdown--right-end");
}

#[test]
fn test_hover_card_position_classes() {
    assert_eq!(HoverCardPosition::Bottom.class_name(), "sd-hovercard-dropdown--bottom");
    assert_eq!(HoverCardPosition::Top.class_name(), "sd-hovercard-dropdown--top");
    assert_eq!(HoverCardPosition::Left.class_name(), "sd-hovercard-dropdown--left");
    assert_eq!(HoverCardPosition::Right.class_name(), "sd-hovercard-dropdown--right");
}

#[test]
fn test_speed_dial_direction_classes() {
    assert_eq!(SpeedDialDirection::Up.class_name(), "sd-speed-dial--up");
    assert_eq!(SpeedDialDirection::Down.class_name(), "sd-speed-dial--down");
    assert_eq!(SpeedDialDirection::Left.class_name(), "sd-speed-dial--left");
    assert_eq!(SpeedDialDirection::Right.class_name(), "sd-speed-dial--right");
}

#[test]
fn test_affix_position_classes() {
    assert_eq!(AffixPosition::BottomRight.class_name(), "sd-affix--bottom-right");
    assert_eq!(AffixPosition::BottomLeft.class_name(), "sd-affix--bottom-left");
    assert_eq!(AffixPosition::TopRight.class_name(), "sd-affix--top-right");
    assert_eq!(AffixPosition::TopLeft.class_name(), "sd-affix--top-left");
}

#[test]
fn test_coachmark_defaults() {
    let pos = CoachmarkPosition::default();
    assert_eq!(pos, CoachmarkPosition::Bottom);
}
