//! Story: Burger (feature `burger`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/burger.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "burger",
        "Burger",
        Category::Navigazione,
        &["burger"],
        IconKind::Menu,
        "Pulsante hamburger che apre e chiude la navigazione mobile.",
    )
}
