//! Story registry list (ADR-0006). ONE `pub mod` and ONE `ALL` entry per story, in sidebar order.
//! Phase 1 writes this file complete; batch engineers never edit it.
use crate::story::schema::Story;

// Fondamenta
pub mod animations;
// Azioni & Form
pub mod buttons;
pub mod inputs;
pub mod number_input;
pub mod pin_input;
pub mod textarea;
pub mod select;
pub mod checkbox;
pub mod radio;
pub mod switch;
pub mod slider;
pub mod rating;
pub mod chip;
pub mod color_picker;
pub mod segmented_controls;
pub mod date_picker;
pub mod dropzone;
pub mod fieldset;
pub mod form;
// Layout
pub mod app_shell;
pub mod navbar;
pub mod grid;
pub mod group;
pub mod stack;
pub mod center;
pub mod aspect_ratio;
pub mod cards;
pub mod scroll_area;
pub mod dividers;
pub mod resizable;
pub mod masonry;
pub mod accordion;
pub mod spoiler;
// Navigazione
pub mod navigation_menu;
pub mod overflow_set;
pub mod nav_link;
pub mod tabs;
pub mod breadcrumbs;
pub mod pagination;
pub mod stepper;
pub mod bottom_navigation;
pub mod menubar;
pub mod burger;
pub mod command_palette;
// Overlay
pub mod modals;
pub mod alert_dialog;
pub mod drawer;
pub mod dropdown;
pub mod popover;
pub mod hover_card;
pub mod context_menu;
pub mod tooltip;
pub mod coachmark;
pub mod speed_dial;
pub mod affix;
pub mod keytip;
// Dati & Feedback
pub mod tables;
pub mod lists;
pub mod timelines;
pub mod badges;
pub mod alerts;
pub mod notifications;
pub mod progress;
pub mod skeletons;
pub mod trees;
pub mod indicator;
pub mod loading_overlay;
pub mod virtual_list;
pub mod transfer_list;
pub mod carousel;
pub mod charts;
// Contenuti & Social
pub mod avatars;
pub mod facepile;
pub mod chat;
pub mod activity_feed;
pub mod attachment_cards;
pub mod document_card;
pub mod media_card;
pub mod rich_text;
pub mod code_highlight;
// Utility
pub mod close_button;
pub mod copy_button;
pub mod highlight;
pub mod marquee_selection;
pub mod reactive_hooks;

pub const ALL: &[fn() -> Story] = &[
    // Fondamenta
    animations::story,
    // Azioni & Form
    buttons::story,
    inputs::story,
    number_input::story,
    pin_input::story,
    textarea::story,
    select::story,
    checkbox::story,
    radio::story,
    switch::story,
    slider::story,
    rating::story,
    chip::story,
    color_picker::story,
    segmented_controls::story,
    date_picker::story,
    dropzone::story,
    fieldset::story,
    form::story,
    // Layout
    app_shell::story,
    navbar::story,
    grid::story,
    group::story,
    stack::story,
    center::story,
    aspect_ratio::story,
    cards::story,
    scroll_area::story,
    dividers::story,
    resizable::story,
    masonry::story,
    accordion::story,
    spoiler::story,
    // Navigazione
    navigation_menu::story,
    overflow_set::story,
    nav_link::story,
    tabs::story,
    breadcrumbs::story,
    pagination::story,
    stepper::story,
    bottom_navigation::story,
    menubar::story,
    burger::story,
    command_palette::story,
    // Overlay
    modals::story,
    alert_dialog::story,
    drawer::story,
    dropdown::story,
    popover::story,
    hover_card::story,
    context_menu::story,
    tooltip::story,
    coachmark::story,
    speed_dial::story,
    affix::story,
    keytip::story,
    // Dati & Feedback
    tables::story,
    lists::story,
    timelines::story,
    badges::story,
    alerts::story,
    notifications::story,
    progress::story,
    skeletons::story,
    trees::story,
    indicator::story,
    loading_overlay::story,
    virtual_list::story,
    transfer_list::story,
    carousel::story,
    charts::story,
    // Contenuti & Social
    avatars::story,
    facepile::story,
    chat::story,
    activity_feed::story,
    attachment_cards::story,
    document_card::story,
    media_card::story,
    rich_text::story,
    code_highlight::story,
    // Utility
    close_button::story,
    copy_button::story,
    highlight::story,
    marquee_selection::story,
    reactive_hooks::story,
];
