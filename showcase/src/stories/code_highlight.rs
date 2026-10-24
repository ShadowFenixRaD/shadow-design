//! Story: CodeHighlight (feature `code_highlight`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/code_highlight.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "code-highlight",
        "CodeHighlight",
        Category::Contenuti,
        &["code_highlight"],
        IconKind::Code,
        "Blocco di codice con evidenziazione sintattica e copia.",
    )
}
