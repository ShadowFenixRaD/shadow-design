use shadow_design::code_highlight::{tokenize_line, SyntaxTokenKind};
use shadow_design::command_palette::CommandAction;
use shadow_design::date_picker::Date;
use shadow_design::dropzone::FileMetadata;
use shadow_design::icon::IconKind;
use shadow_design::resizable::ResizableDirection;

#[test]
fn test_date_leap_year_calculation() {
    assert!(Date::is_leap_year(2024));
    assert!(Date::is_leap_year(2000));
    assert!(!Date::is_leap_year(2026));
    assert!(!Date::is_leap_year(1900));
    assert!(!Date::is_leap_year(2023));
}

#[test]
fn test_date_days_in_month() {
    assert_eq!(Date::days_in_month(2026, 1), 31);
    assert_eq!(Date::days_in_month(2026, 2), 28);
    assert_eq!(Date::days_in_month(2024, 2), 29);
    assert_eq!(Date::days_in_month(2026, 4), 30);
    assert_eq!(Date::days_in_month(2026, 12), 31);
}

#[test]
fn test_date_day_of_week() {
    // 2026-10-22 è un Giovedì (0=Lun, 1=Mar, 2=Mer, 3=Gio, 4=Ven, 5=Sab, 6=Dom)
    let d = Date::new(2026, 10, 22);
    assert_eq!(d.day_of_week(), 3);

    // 2026-10-19 è Lunedì (0)
    let d_mon = Date::new(2026, 10, 19);
    assert_eq!(d_mon.day_of_week(), 0);

    // 2026-10-25 è Domenica (6)
    let d_sun = Date::new(2026, 10, 25);
    assert_eq!(d_sun.day_of_week(), 6);
}

#[test]
fn test_date_formatting_and_parsing() {
    let d = Date::new(2026, 5, 8);
    assert_eq!(d.format_iso(), "2026-05-08");
    assert_eq!(d.format_it(), "08/05/2026");

    let parsed = Date::parse_iso("2026-05-08");
    assert_eq!(parsed, Some(d));

    assert_eq!(Date::parse_iso("invalid-date"), None);
    assert_eq!(Date::parse_iso("2026-13-01"), None);
}

#[test]
fn test_date_prev_and_next_month_transitions() {
    let jan = Date::new(2026, 1, 15);
    let prev = jan.prev_month();
    assert_eq!(prev.year, 2025);
    assert_eq!(prev.month, 12);

    let dec = Date::new(2025, 12, 10);
    let next = dec.next_month();
    assert_eq!(next.year, 2026);
    assert_eq!(next.month, 1);
}

#[test]
fn test_command_action_builder() {
    let action = CommandAction::new("settings", "Impostazioni di Sistema")
        .with_description("Configura parametri utente e workspace")
        .with_category("Sistema")
        .with_shortcut("Ctrl+,")
        .with_icon(IconKind::Settings)
        .with_keywords(["opzioni", "preferenze"]);

    assert_eq!(action.id, "settings");
    assert_eq!(action.title, "Impostazioni di Sistema");
    assert_eq!(action.description.as_deref(), Some("Configura parametri utente e workspace"));
    assert_eq!(action.category.as_deref(), Some("Sistema"));
    assert_eq!(action.shortcut.as_deref(), Some("Ctrl+,"));
    assert_eq!(action.icon, Some(IconKind::Settings));
    assert_eq!(action.keywords.len(), 2);
}

#[test]
fn test_file_metadata_format_size() {
    let f1 = FileMetadata {
        name: "small.txt".to_string(),
        size_bytes: 512,
        mime_type: Some("text/plain".to_string()),
    };
    assert_eq!(f1.format_size(), "512 B");

    let f2 = FileMetadata {
        name: "image.png".to_string(),
        size_bytes: 1024 * 350,
        mime_type: Some("image/png".to_string()),
    };
    assert_eq!(f2.format_size(), "350.0 KB");

    let f3 = FileMetadata {
        name: "video.mp4".to_string(),
        size_bytes: 1024 * 1024 * 15,
        mime_type: Some("video/mp4".to_string()),
    };
    assert_eq!(f3.format_size(), "15.0 MB");
}

#[test]
fn test_code_highlight_tokenizer() {
    let tokens = tokenize_line("pub fn test_run() -> bool {", "rust");
    assert!(!tokens.is_empty());
    assert_eq!(tokens[0].kind, SyntaxTokenKind::Keyword);
    assert_eq!(tokens[0].text, "pub");

    let comment_tokens = tokenize_line("// Questo è un commento", "rust");
    assert_eq!(comment_tokens.len(), 1);
    assert_eq!(comment_tokens[0].kind, SyntaxTokenKind::Comment);
}

#[test]
fn test_resizable_direction_default() {
    assert_eq!(ResizableDirection::default(), ResizableDirection::Horizontal);
}

#[test]
fn test_calendar_view_mode_defaults() {
    use shadow_design::date_picker::CalendarViewMode;
    assert_eq!(CalendarViewMode::default(), CalendarViewMode::Day);
    assert_ne!(CalendarViewMode::Month, CalendarViewMode::Year);
}
