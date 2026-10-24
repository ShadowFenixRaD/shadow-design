//! Story: ActivityFeed (feature `activity_feed`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/activity_feed.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "activity-feed",
        "ActivityFeed",
        Category::Contenuti,
        &["activity_feed"],
        IconKind::Clock,
        "Feed di eventi collaborativi con commenti e risposte.",
    )
}
