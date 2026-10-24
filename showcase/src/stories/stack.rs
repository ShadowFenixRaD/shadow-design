//! Story: Stack (feature `stack`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/stack.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "stack",
        "Stack",
        Category::Layout,
        &["stack"],
        IconKind::Layers,
        "Disposizione verticale con spaziatura uniforme a token.",
    )
}
