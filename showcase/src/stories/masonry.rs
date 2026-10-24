//! Story: MasonryGrid (feature `masonry`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/masonry.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "masonry",
        "MasonryGrid",
        Category::Layout,
        &["masonry"],
        IconKind::Layers,
        "Griglia a cascata per gallerie con lightbox.",
    )
}
