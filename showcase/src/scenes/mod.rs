//! Composed scenes (REVAMP §1.3): realistic product screens built only from library
//! components. Home shows all four; stories embed them as "In contesto" examples.
//! Read-only for batch engineers: compose them, never edit them from a story.
mod auth_card;
mod inbox_pane;
mod ops_dashboard;
mod settings_form;

pub use auth_card::AuthCard;
pub use inbox_pane::InboxPane;
pub use ops_dashboard::OpsDashboard;
pub use settings_form::SettingsForm;
