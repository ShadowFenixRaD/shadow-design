//! Story: Textarea (feature `textarea`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/textarea.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "textarea",
        "Textarea",
        Category::Form,
        &["textarea"],
        IconKind::Code,
        "Area di testo multilinea con autosize e conteggio caratteri.",
    )
}
