//! Story: RichTextEditor (feature `rich_text`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/rich_text.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "rich-text",
        "RichTextEditor",
        Category::Contenuti,
        &["rich_text"],
        IconKind::Code,
        "Editor di testo ricco con toolbar e modalità Markdown.",
    )
}
