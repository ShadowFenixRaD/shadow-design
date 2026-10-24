//! Story: Dropdown (feature `dropdown`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/dropdown.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "dropdown",
        "Dropdown",
        Category::Overlay,
        &["dropdown"],
        IconKind::ChevronDown,
        "Menu di azioni ancorato a un pulsante.",
    )
}
