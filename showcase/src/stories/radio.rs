//! Story: RadioGroup (feature `radio`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/radio.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "radio",
        "RadioGroup",
        Category::Form,
        &["radio"],
        IconKind::Radio,
        "Selezione singola esclusiva con Radio, RadioGroup e RadioCard.",
    )
}
