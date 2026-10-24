//! Story: Dropzone (feature `dropzone`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/dropzone.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "dropzone",
        "Dropzone",
        Category::Form,
        &["dropzone"],
        IconKind::Upload,
        "Area di trascinamento file con validazione di formato e dimensione.",
    )
}
