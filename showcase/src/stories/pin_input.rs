//! Story: PinInput (feature `pin_input`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/pin_input.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "pin-input",
        "PinInput",
        Category::Form,
        &["pin_input"],
        IconKind::Terminal,
        "Inserimento di codici PIN e OTP a caselle separate.",
    )
}
