//! Story: Modal (feature `modal`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/modals.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "modals",
        "Modal",
        Category::Overlay,
        &["modal"],
        IconKind::Eye,
        "Finestra di dialogo sopra la pagina, con backdrop e chiusura.",
    )
}
