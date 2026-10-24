//! Story: AspectRatio (feature `aspect_ratio`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/aspect_ratio.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "aspect-ratio",
        "AspectRatio",
        Category::Layout,
        &["aspect_ratio"],
        IconKind::Box,
        "Vincola i media a un rapporto di forma fisso.",
    )
}
