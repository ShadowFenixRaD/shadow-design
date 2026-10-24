//! Story: Form (feature `form`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/form.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "form",
        "Form",
        Category::Form,
        &["form"],
        IconKind::CheckCircle,
        "Stato e validazione dei moduli con use_form e regole tipizzate.",
    )
}
