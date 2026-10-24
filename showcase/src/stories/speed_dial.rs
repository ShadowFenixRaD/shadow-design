//! Story: SpeedDial (feature `speed_dial`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/speed_dial.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "speed-dial",
        "SpeedDial",
        Category::Overlay,
        &["speed_dial"],
        IconKind::Plus,
        "Pulsante d'azione flottante che espande azioni correlate.",
    )
}
