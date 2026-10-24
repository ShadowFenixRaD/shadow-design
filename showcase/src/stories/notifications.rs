//! Story: Notification (feature `notification`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/notifications.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "notifications",
        "Notification",
        Category::Dati,
        &["notification"],
        IconKind::Info,
        "Notifiche toast globali annunciate agli screen reader.",
    )
}
