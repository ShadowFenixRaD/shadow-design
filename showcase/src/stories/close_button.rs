//! Story: CloseButton (feature `close_button`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/close_button.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "close-button",
        "CloseButton",
        Category::Utility,
        &["close_button"],
        IconKind::X,
        "Pulsante di chiusura con icona X per overlay e avvisi.",
    )
}
