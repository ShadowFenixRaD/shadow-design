//! Story: NumberInput (feature `number_input`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/number_input.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "number-input",
        "NumberInput",
        Category::Form,
        &["number_input"],
        IconKind::Terminal,
        "Input numerico con stepper, decimali e limiti min/max.",
    )
}
