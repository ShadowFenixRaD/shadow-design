//! Story: TextInput (feature `input`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/inputs.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "inputs",
        "TextInput",
        Category::Form,
        &["input"],
        IconKind::Terminal,
        "Campo di testo con etichetta, icone, pulsante di cancellazione e stato di errore.",
    )
}
