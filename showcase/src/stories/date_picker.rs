//! Story: DatePicker (feature `date_picker`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/date_picker.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "date-picker",
        "DatePicker",
        Category::Form,
        &["date_picker"],
        IconKind::Calendar,
        "Calendario per date singole e intervalli.",
    )
}
