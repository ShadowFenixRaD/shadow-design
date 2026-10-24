//! Story: Rating (feature `rating`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/rating.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "rating",
        "Rating",
        Category::Form,
        &["rating"],
        IconKind::Sparkles,
        "Valutazione a stelle con mezze frazioni e anteprima al passaggio.",
    )
}
