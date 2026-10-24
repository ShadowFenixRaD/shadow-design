//! Story: Slider (feature `slider`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/slider.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "slider",
        "Slider",
        Category::Form,
        &["slider"],
        IconKind::Layers,
        "Cursore a trascinamento con tacche e doppio intervallo.",
    )
}
