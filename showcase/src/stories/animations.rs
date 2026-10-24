//! Story: Transition (feature `animation`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/animations.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "animations",
        "Transition",
        Category::Fondamenta,
        &["animation"],
        IconKind::Sparkles,
        "Micro-animazioni di ingresso e uscita che rispettano prefers-reduced-motion.",
    )
}
