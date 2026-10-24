//! Story: TreeView (feature `tree`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/trees.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "trees",
        "TreeView",
        Category::Dati,
        &["tree"],
        IconKind::Radio,
        "Struttura gerarchica di nodi espandibili.",
    )
}
