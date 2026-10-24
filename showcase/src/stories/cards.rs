//! Story: Card (feature `card`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/cards.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "cards",
        "Card",
        Category::Layout,
        &["card"],
        IconKind::Box,
        "Contenitore di vetro con titolo, azioni, footer e varianti.",
    )
}
