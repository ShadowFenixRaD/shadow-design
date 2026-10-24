//! Story: HoverCard (feature `hover_card`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/hover_card.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "hover-card",
        "HoverCard",
        Category::Overlay,
        &["hover_card"],
        IconKind::Eye,
        "Anteprima ricca che appare al passaggio del puntatore.",
    )
}
