//! Story: MarqueeSelection (feature `marquee_selection`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/marquee_selection.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "marquee-selection",
        "MarqueeSelection",
        Category::Utility,
        &["marquee_selection"],
        IconKind::Layers,
        "Selezione di più elementi trascinando un rettangolo.",
    )
}
