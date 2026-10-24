//! Story: Switch (feature `switch`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/switch.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "switch",
        "Switch",
        Category::Form,
        &["switch"],
        IconKind::Settings,
        "Interruttore on/off per impostazioni che si applicano subito.",
    )
}
