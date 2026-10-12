use shadow_design::prelude::*;

#[test]
fn test_avatar_sizes_and_presence() {
    assert_eq!(AvatarSize::Xs.scss_class(), "sd-avatar--xs");
    assert_eq!(AvatarSize::Sm.scss_class(), "sd-avatar--sm");
    assert_eq!(AvatarSize::Md.scss_class(), "sd-avatar--md");
    assert_eq!(AvatarSize::Lg.scss_class(), "sd-avatar--lg");
    assert_eq!(AvatarSize::Xl.scss_class(), "sd-avatar--xl");

    assert_eq!(PresenceStatus::Online.scss_class(), "sd-avatar__status--online");
    assert_eq!(PresenceStatus::Offline.scss_class(), "sd-avatar__status--offline");
    assert_eq!(PresenceStatus::Busy.scss_class(), "sd-avatar__status--busy");
    assert_eq!(PresenceStatus::DirectP2P.scss_class(), "sd-avatar__status--direct-p2p");
}

#[test]
fn test_chat_bubble_directions_and_delivery() {
    assert_eq!(BubbleDirection::Incoming.scss_class(), "sd-chat-bubble--incoming");
    assert_eq!(BubbleDirection::Outgoing.scss_class(), "sd-chat-bubble--outgoing");

    assert_eq!(DeliveryStatus::Sending.scss_class(), "sd-chat-bubble__status--sending");
    assert_eq!(DeliveryStatus::Sent.scss_class(), "sd-chat-bubble__status--sent");
    assert_eq!(DeliveryStatus::Delivered.scss_class(), "sd-chat-bubble__status--delivered");
    assert_eq!(DeliveryStatus::Failed.scss_class(), "sd-chat-bubble__status--failed");
}

#[test]
fn test_progress_sizes_and_variants() {
    assert_eq!(ProgressSize::Sm.scss_class(), "sd-progress--sm");
    assert_eq!(ProgressSize::Md.scss_class(), "sd-progress--md");
    assert_eq!(ProgressSize::Lg.scss_class(), "sd-progress--lg");

    assert_eq!(ProgressVariant::Primary.scss_class(), "sd-progress--primary");
    assert_eq!(ProgressVariant::Success.scss_class(), "sd-progress--success");
    assert_eq!(ProgressVariant::Warning.scss_class(), "sd-progress--warning");
    assert_eq!(ProgressVariant::Danger.scss_class(), "sd-progress--danger");
}

#[test]
fn test_segmented_control_options() {
    let opt = SegmentOption::new("sqlite", "SQLite Storage")
        .with_icon(IconKind::Server.into())
        .with_disabled(false);

    assert_eq!(opt.value, "sqlite");
    assert_eq!(opt.label, "SQLite Storage");
    assert!(!opt.disabled);

    assert_eq!(SegmentedControlSize::Sm.scss_class(), "sd-segmented-control--sm");
    assert_eq!(SegmentedControlSize::Md.scss_class(), "sd-segmented-control--md");
    assert_eq!(SegmentedControlSize::Lg.scss_class(), "sd-segmented-control--lg");
}

#[test]
fn test_divider_orientations() {
    assert_eq!(Orientation::Horizontal.scss_class(), "sd-divider--horizontal");
    assert_eq!(Orientation::Vertical.scss_class(), "sd-divider--vertical");
}

#[test]
fn test_tree_node_hierarchy() {
    let root = TreeNode::new("creator-0", "Creator Node 0")
        .with_sublabel("0ms")
        .with_badge("ROOT")
        .with_icon(IconKind::Radio.into())
        .with_children(vec![
            TreeNode::new("router-1", "Router 1")
                .with_sublabel("8ms • 1 hop")
                .with_children(vec![
                    TreeNode::new("sub-1.1", "Subordinate 1.1").with_sublabel("18ms • 2 hops"),
                ]),
        ]);

    assert_eq!(root.id, "creator-0");
    assert_eq!(root.label, "Creator Node 0");
    assert_eq!(root.sublabel.as_deref(), Some("0ms"));
    assert_eq!(root.badge.as_deref(), Some("ROOT"));
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].children.len(), 1);
    assert_eq!(root.children[0].children[0].id, "sub-1.1");
}

#[test]
fn test_skeleton_shapes() {
    assert_eq!(SkeletonShape::Circle.scss_class(), "sd-skeleton--circle");
    assert_eq!(SkeletonShape::Rectangle.scss_class(), "sd-skeleton--rectangle");
    assert_eq!(SkeletonShape::Text.scss_class(), "sd-skeleton--text");
}

#[test]
fn test_new_icon_kinds() {
    let icons = vec![
        IconKind::Send,
        IconKind::MessageSquare,
        IconKind::Paperclip,
        IconKind::CheckCheck,
        IconKind::Clock,
        IconKind::Lock,
        IconKind::Unlock,
        IconKind::Shield,
        IconKind::ShieldCheck,
        IconKind::Key,
        IconKind::QrCode,
        IconKind::Fingerprint,
        IconKind::Wifi,
        IconKind::Bluetooth,
        IconKind::Radio,
        IconKind::Signal,
        IconKind::SignalHigh,
        IconKind::Server,
        IconKind::Share2,
        IconKind::Trash2,
        IconKind::Flame,
        IconKind::Download,
        IconKind::Upload,
    ];

    for kind in icons {
        let svg: SvgIcon = kind.into();
        assert_eq!(svg, SvgIcon::Kind(kind));
    }
}
