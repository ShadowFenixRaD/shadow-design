//! Story: Hooks (feature `hooks`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/reactive_hooks.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "reactive-hooks",
        "Hooks",
        Category::Utility,
        &["hooks"],
        IconKind::Sparkles,
        "Hook reattivi per hotkey, debounce, storage, media query e scroll.",
    )
}
