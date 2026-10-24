//! Story: ResizablePanelGroup (feature `resizable`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/resizable.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "resizable",
        "ResizablePanelGroup",
        Category::Layout,
        &["resizable"],
        IconKind::GripVertical,
        "Pannelli affiancati ridimensionabili con maniglia.",
    )
}
