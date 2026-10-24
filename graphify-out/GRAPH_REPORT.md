# Graph Report - shadow-design  (2026-10-24)

## Corpus Check
- 292 files · ~247,246 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 107 file(s) not represented in the graph (top: .scss 94, (none) 7, .toml 2)

## Summary
- 3770 nodes · 6359 edges · 230 communities (205 shown, 25 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 167 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `86614fb4`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Route
- lib.rs
- IconKind
- AvatarSize
- notification.rs
- Animations
- AnimationVariant
- list.rs
- src/rich_text.rs
- SpacingToken
- table.rs
- GridSpacing
- SvgIcon
- menubar/mod.rs
- stepper.rs
- app_shell/mod.rs
- icons.rs
- src/color_picker.rs
- src/overflow_set.rs
- accordion/mod.rs
- BadgeVariant
- avatars.rs
- PropRow
- ControlSize
- src/slider.rs
- Date
- IndicatorColor
- ComponentSpacing
- Theme
- timeline.rs
- stories/affix.rs
- AGENTS.md — Shadow Design Workspace Guide
- primitives.rs
- theme.rs
- PopoverPosition
- src/select.rs
- Changelog — Shadow Design
- AlertVariant
- breadcrumbs/mod.rs
- CardShadow
- ChartSeries
- src/chip.rs
- Changelog — Shadow Design
- AGENTS.md — Shadow Design Workspace Guide
- super
- PrimaryColor
- CloseButton
- src/radio.rs
- stories/chat.rs
- UseForm<T>
- NavigationLinkItem
- stories/progress.rs
- ComponentOverride
- debounce.rs
- area.rs
- src/pin_input.rs
- get_series_color
- ThemeBuilder
- stories/code_highlight.rs
- bottom_navigation/mod.rs
- stories/hover_card.rs
- JustifyContent
- stories/speed_dial.rs
- pie.rs
- TreeNode
- stories/context_menu.rs
- src/drawer.rs
- modal.rs
- scroll_area.rs
- 4. Snippets
- bottom_navigation.rs
- AlignItems
- Burger
- stories/checkbox.rs
- src/context_menu.rs
- Locale
- src/number_input.rs
- RadiusScale
- lint_design.py
- 📦 Libreria UI (`crate/`)
- BarChart
- stories/radio.rs
- heatmap.rs
- stories/fieldset.rs
- use_theme
- skeletons.rs
- ElevationStyle
- stories/transfer_list.rs
- theme_menu.rs
- components/controls.rs
- Shadow Design (`shadow-design`)
- PersonaProps
- dividers.rs
- src/rating.rs
- src/textarea.rs
- 3. Recipes
- registry.rs
- src/tabs.rs
- stories/select.rs
- stories/rating.rs
- ContactCard
- DataTableColumn<T>
- breadcrumbs.rs
- stories/date_picker.rs
- stories/stack.rs
- reactive_hooks_tests.rs
- adr/README.md
- Shadow Design — Showcase App
- Signal
- pages/typography.rs
- ButtonSize
- home.rs
- spoiler/mod.rs
- sections.rs
- Showcase revamp: batch migration guide (phase 2)
- alerts.rs
- Knobs
- components/layout.rs
- badges.rs
- buttons.rs
- stories/facepile.rs
- stories/activity_feed.rs
- nav_link.rs
- SegmentedControl
- app/layout.rs
- CommandAction
- ColorFormat
- ListMarker
- schema.rs
- click_outside.rs
- ADR-0005: Three type roles: Kanit, Onest, Source Code Pro
- ADR-0006: The showcase is data-driven documentation built from the library
- 7. Regole di Sviluppo
- Switch
- ADR-0002: One design language: Lit Glass
- Element
- not_found.rs
- ToolbarAction
- stories/copy_button.rs
- accordion.rs
- ButtonVariant
- use_notification
- animations.rs
- app_shell.rs
- $schema
- attachment_cards.rs
- stories/burger.rs
- step.rs
- stories/carousel.rs
- RadiusScale
- PropDoc
- avatar.rs
- PresenceStatus
- stories/chip.rs
- stories/close_button.rs
- stories/coachmark.rs
- use_theme
- stories/color_picker.rs
- stories/command_palette.rs
- stories/document_card.rs
- stories/drawer.rs
- stories/dropdown.rs
- stories/dropzone.rs
- src/coachmark.rs
- SyntaxTokenKind
- stories/grid.rs
- stories/group.rs
- stories/highlight.rs
- ChoiceOption
- Lit Glass: the Shadow Design language
- Token reference
- inputs.rs
- stories/keytip.rs
- TooltipPosition
- lists.rs
- shadow-design
- notifications.rs
- prelude
- Component checklist
- ADR-0001: Record decisions in ADRs and bind agents to them
- ADR-0004: Component styling contract
- 7. Regole di Sviluppo
- stories/loading_overlay.rs
- stories/marquee_selection.rs
- src/code_highlight.rs
- stories/media_card.rs
- ADR-0003: Runtime CSS tokens are the only theming mechanism
- menubar.rs
- design-system.md
- modals.rs
- stories/navbar.rs
- stories/navigation_menu.rs
- stories/number_input.rs
- stories/overflow_set.rs
- pagination.rs
- stories/pin_input.rs
- src/grid.rs
- OpsDashboard
- reactive_hooks.rs
- stories/resizable.rs
- GridAlign
- ListSize
- segmented_controls.rs
- stories/slider.rs
- spoiler.rs
- ListSpacing
- stories/switch.rs
- tables.rs
- stories/tabs.rs
- stories/textarea.rs
- stories/tooltip.rs
- GridJustify
- trees.rs
- stories/virtual_list.rs
- hooks/mod.rs
- sidebar.rs
- src/alert_dialog.rs
- CalendarViewMode
- src/loading_overlay.rs

## God Nodes (most connected - your core abstractions)
1. `PropDoc` - 178 edges
2. `Knobs` - 151 edges
3. `IconKind` - 145 edges
4. `ChoiceOption` - 95 edges
5. `Signal` - 92 edges
6. `$schema` - 86 edges
7. `use_notification()` - 79 edges
8. `Theme` - 50 edges
9. `Icon()` - 49 edges
10. `use_theme()` - 37 edges

## Surprising Connections (you probably didn't know these)
- `SIZES` --references--> `ControlSize`  [EXTRACTED]
  showcase/src/stories/pagination.rs → crate/src/a11y.rs
- `VARIANTS` --references--> `AlertVariant`  [EXTRACTED]
  showcase/src/stories/alerts.rs → crate/src/alert.rs
- `VARIANTS` --references--> `AnimationVariant`  [EXTRACTED]
  showcase/src/stories/animations.rs → crate/src/animation.rs
- `SIZES` --references--> `AvatarSize`  [EXTRACTED]
  showcase/src/stories/facepile.rs → crate/src/avatar.rs
- `status_badge()` --references--> `BadgeVariant`  [EXTRACTED]
  showcase/src/scenes/ops_dashboard.rs → crate/src/badge.rs

## Import Cycles
- 1-file cycle: `crate/src/lib.rs -> crate/src/lib.rs`
- 1-file cycle: `showcase/src/main.rs -> showcase/src/main.rs`
- 2-file cycle: `showcase/src/story/knobs.rs -> showcase/src/story/schema.rs -> showcase/src/story/knobs.rs`
- 2-file cycle: `crate/src/stepper/mod.rs -> crate/src/stepper/step.rs -> crate/src/stepper/mod.rs`
- 2-file cycle: `crate/src/stepper/completed.rs -> crate/src/stepper/mod.rs -> crate/src/stepper/completed.rs`
- 2-file cycle: `crate/src/menubar/item.rs -> crate/src/menubar/mod.rs -> crate/src/menubar/item.rs`
- 2-file cycle: `crate/src/menubar/menu.rs -> crate/src/menubar/mod.rs -> crate/src/menubar/menu.rs`
- 2-file cycle: `crate/src/app_shell/mod.rs -> crate/src/app_shell/navbar.rs -> crate/src/app_shell/mod.rs`
- 2-file cycle: `crate/src/app_shell/aside.rs -> crate/src/app_shell/mod.rs -> crate/src/app_shell/aside.rs`
- 2-file cycle: `crate/src/accordion/mod.rs -> crate/src/accordion/panel.rs -> crate/src/accordion/mod.rs`

## Communities (230 total, 25 thin omitted)

### Community 0 - "Route"
Cohesion: 0.15
Nodes (17): VERSION, App(), FAVICON, HEADER_SVG, Route, Home, IconsPage, NotFound (+9 more)

### Community 2 - "IconKind"
Cohesion: 0.03
Nodes (56): AccordionControl(), IconKind, AlertCircle, AlertTriangle, .ALL, Bluetooth, Box, Calendar (+48 more)

### Community 3 - "AvatarSize"
Cohesion: 0.15
Nodes (8): AvatarSize, Lg, Md, Sm, Xl, Xs, Facepile(), SIZES

### Community 4 - "notification.rs"
Cohesion: 0.12
Nodes (12): NotificationContainer(), NotificationHandle, NotificationItem, NotificationProvider(), NotificationToast(), NotificationVariant, Danger, Info (+4 more)

### Community 5 - "Animations"
Cohesion: 0.11
Nodes (18): Animations, .DRAWER_LEFT, .DRAWER_RIGHT, .DROPDOWN_ENTER, .FADE_IN, .FADE_OUT, .INDETERMINATE, .MODAL_ENTER (+10 more)

### Community 6 - "AnimationVariant"
Cohesion: 0.10
Nodes (19): AnimationVariant, DrawerSlideLeft, DrawerSlideRight, DropdownEnter, FadeIn, FadeOut, Indeterminate, ModalEnter (+11 more)

### Community 7 - "list.rs"
Cohesion: 0.24
Nodes (10): List(), ListContext, ListItem(), ListOrientation, Horizontal, Vertical, ListType, Ordered (+2 more)

### Community 8 - "src/rich_text.rs"
Cohesion: 0.05
Nodes (44): format_inline(), html_escape(), html_to_markdown(), markdown_to_html(), RichTextCommand, Blockquote, Bold, BulletList (+36 more)

### Community 9 - "SpacingToken"
Cohesion: 0.16
Nodes (8): Group(), SpacingToken, Lg, Md, None, Sm, Xl, Xs

### Community 10 - "table.rs"
Cohesion: 0.17
Nodes (14): DataTable(), DataTableColumn, SortDirection, Ascending, Descending, None, Table(), TableBody() (+6 more)

### Community 12 - "GridSpacing"
Cohesion: 0.22
Nodes (9): GridSpacing, Lg, Md, None, Sm, Xl, Xs, SimpleGrid() (+1 more)

### Community 13 - "SvgIcon"
Cohesion: 0.10
Nodes (12): AttachmentCard(), BottomNavItem(), BottomNavItemProps, Icon(), render_icon_paths(), RenderSvgIcon(), SvgIcon, Custom (+4 more)

### Community 14 - "menubar/mod.rs"
Cohesion: 0.18
Nodes (14): MenubarContent(), MenubarContentProps, MenubarMenu(), MenubarMenuProps, MenubarTrigger(), MenubarTriggerProps, SingleMenuCtx, CommandBar() (+6 more)

### Community 15 - "stepper.rs"
Cohesion: 0.08
Nodes (25): StepperCompleted(), StepperCompletedProps, Stepper(), StepperCtx, StepperOrientation, .ALL, Horizontal, Vertical (+17 more)

### Community 16 - "app_shell/mod.rs"
Cohesion: 0.14
Nodes (13): AppShellAside(), AppShellAsideProps, AppShellFooter(), AppShellFooterProps, AppShellHeader(), AppShellHeaderProps, AppShellMain(), AppShellMainProps (+5 more)

### Community 18 - "src/color_picker.rs"
Cohesion: 0.16
Nodes (23): use_unique_id(), ColorInput(), ColorPicker(), default_color_swatches(), format_color(), hsl_to_rgba(), Hsla, hsv_to_rgb() (+15 more)

### Community 19 - "src/overflow_set.rs"
Cohesion: 0.17
Nodes (10): Dropdown(), DropdownCtx, DropdownDivider(), DropdownItem(), DropdownPlacement, BottomLeft, BottomRight, TopLeft (+2 more)

### Community 20 - "accordion/mod.rs"
Cohesion: 0.11
Nodes (13): AccordionItem(), Accordion(), AccordionContext, AccordionItemContext, AccordionVariant, Contained, Default, Separated (+5 more)

### Community 21 - "BadgeVariant"
Cohesion: 0.11
Nodes (13): Badge(), BadgeVariant, Danger, Default, Outline, Primary, Success, Warning (+5 more)

### Community 22 - "avatars.rs"
Cohesion: 0.14
Nodes (15): AVATAR_PROPS, GROUP_PROPS, IN_CONTEXT_CODE, InContext(), PERSONA_PROPS, PHOTO, Presence(), PRESENCE_CODE (+7 more)

### Community 23 - "PropRow"
Cohesion: 0.32
Nodes (3): PropRow, PropsTable(), ShowcaseTabs()

### Community 24 - "ControlSize"
Cohesion: 0.12
Nodes (18): ControlSize, Lg, Md, Sm, calculate_pagination_range(), PaginationControl(), PaginationControlProps, PaginationItem (+10 more)

### Community 25 - "src/slider.rs"
Cohesion: 0.23
Nodes (7): calc_percentage(), RangeSlider(), Slider(), SliderMark, snap_to_step(), test_slider_marks(), bandwidth_marks()

### Community 26 - "Date"
Cohesion: 0.26
Nodes (4): Calendar(), Date, DatePicker(), DateRangePicker()

### Community 27 - "IndicatorColor"
Cohesion: 0.10
Nodes (19): Indicator(), IndicatorColor, Amber, Cyan, Emerald, Rose, Violet, IndicatorPosition (+11 more)

### Community 28 - "ComponentSpacing"
Cohesion: 0.06
Nodes (24): ComponentSpacing, Lg, Md, None, Sm, Xl, Xs, ImageList() (+16 more)

### Community 29 - "Theme"
Cohesion: 0.09
Nodes (4): test_css_variables_share_one_source(), test_presets_emit_identical_classes(), Theme, ThemeProvider()

### Community 30 - "timeline.rs"
Cohesion: 0.07
Nodes (29): Timeline(), TimelineAlign, Left, Right, TimelineBulletState, Active, Completed, Default (+21 more)

### Community 31 - "stories/affix.rs"
Cohesion: 0.06
Nodes (32): Affix(), AffixPosition, BottomLeft, BottomRight, TopLeft, TopRight, BackToTop(), Code() (+24 more)

### Community 32 - "AGENTS.md — Shadow Design Workspace Guide"
Cohesion: 0.12
Nodes (16): 0. Navigazione del Codice con graphify, 1. Panoramica del Progetto, 2. Struttura del Workspace Cargo, 3. Feature Flags, 4. Sistema di Stile (100% SCSS), 5. Theme Engine (`crate/src/theme.rs`, ADR-0003), 6. Showcase (`showcase/`), 8. Comandi (+8 more)

### Community 33 - "primitives.rs"
Cohesion: 0.17
Nodes (16): CurveType, Linear, Monotone, Step, DEFAULT_CHART_COLORS, generate_stacked_area_band_path(), generate_svg_area_path(), generate_svg_path() (+8 more)

### Community 34 - "theme.rs"
Cohesion: 0.11
Nodes (11): AccentTokens, DEFAULT_ACCENT, DEFAULT_SURFACES, mix(), mix_channel(), relative_luminance(), safe_hex(), test_accent_derivation_ultraviolet() (+3 more)

### Community 35 - "PopoverPosition"
Cohesion: 0.07
Nodes (32): Popover(), PopoverContext, PopoverDropdown(), PopoverPosition, Bottom, BottomEnd, BottomStart, Left (+24 more)

### Community 37 - "src/select.rs"
Cohesion: 0.20
Nodes (12): Combobox(), display_label(), filter_items(), MultiSelect(), Select(), SelectItem, SelectOption(), test_display_label() (+4 more)

### Community 38 - "Changelog — Shadow Design"
Cohesion: 0.06
Nodes (34): [0.1.0] — 2026-10-09, [0.2.0] — 2026-10-11, [0.3.0] — 2026-10-14, [0.4.0] — 2026-10-20, [0.5.0] — 2026-10-24, 📦 Architettura del Crate & Metadati, ⚠️ Breaking changes, ⚠️ Breaking — fondamenta Lit Glass (ADR-0002, ADR-0003, ADR-0004) (+26 more)

### Community 39 - "AlertVariant"
Cohesion: 0.18
Nodes (6): Alert(), AlertVariant, Danger, Info, Success, Warning

### Community 40 - "breadcrumbs/mod.rs"
Cohesion: 0.15
Nodes (11): BreadcrumbItem(), BreadcrumbItemProps, BreadcrumbData, Breadcrumbs(), BreadcrumbsProps, calculate_collapse_bounds(), test_breadcrumb_data_builder(), test_collapse_bounds_custom_overrides() (+3 more)

### Community 41 - "CardShadow"
Cohesion: 0.05
Nodes (33): AspectRatio(), Card(), CardShadow, Lg, Md, None, Sm, Xl (+25 more)

### Community 42 - "ChartSeries"
Cohesion: 0.14
Nodes (6): AreaChart(), LineChart(), ChartDataPoint, ChartSeries, compute_min_max(), test_point_builder()

### Community 43 - "src/chip.rs"
Cohesion: 0.16
Nodes (12): Chip(), ChipGroup(), ChipGroupContext, ChipGroupProps, ChipProps, ChipVariant, Filled, Light (+4 more)

### Community 44 - "Changelog — Shadow Design"
Cohesion: 0.07
Nodes (29): [0.1.0] — 2026-10-09, [0.2.0] — 2026-10-11, [0.3.0] — 2026-10-14, [0.4.0] — 2026-10-20, [0.5.0] — 2026-10-24, 📦 Architettura del Crate & Metadati, Changelog — Shadow Design, 🐛 Correzioni & Miglioramenti (+21 more)

### Community 45 - "AGENTS.md — Shadow Design Workspace Guide"
Cohesion: 0.12
Nodes (16): 0. Navigazione del Codice con graphify, 1. Panoramica del Progetto, 2. Struttura del Workspace Cargo, 3. Feature Flags, 4. Sistema di Stile (100% SCSS), 5. Theme Engine (`crate/src/theme.rs`, ADR-0003), 6. Showcase (`showcase/`), 8. Comandi (+8 more)

### Community 46 - "super"
Cohesion: 0.33
Nodes (3): AriaHeading(), NEXT_ID, use_body_scroll_lock()

### Community 47 - "PrimaryColor"
Cohesion: 0.16
Nodes (8): PrimaryColor, Crimson, Cyan, Emerald, Obsidian, Violet, sanitize_css_prefix(), test_custom_color_any_prefix_uses_generic_classes()

### Community 48 - "CloseButton"
Cohesion: 0.13
Nodes (11): CloseButton(), CloseButtonSize, Lg, Md, Sm, CloseButtonVariant, Filled, Subtle (+3 more)

### Community 49 - "src/radio.rs"
Cohesion: 0.20
Nodes (8): Radio(), RadioCard(), RadioContext, RadioGroup(), RadioOrientation, Horizontal, Vertical, ORIENTATIONS

### Community 50 - "stories/chat.rs"
Cohesion: 0.09
Nodes (21): BubbleDirection, Incoming, Outgoing, DeliveryStatus, Delivered, Failed, Sending, Sent (+13 more)

### Community 51 - "UseForm<T>"
Cohesion: 0.08
Nodes (21): FormOptions, FormValidator, use_form(), UseForm, UseForm<T>, test_debounced_signal_initial_value(), test_form_validation_and_errors(), TestFormModel (+13 more)

### Community 52 - "NavigationLinkItem"
Cohesion: 0.21
Nodes (5): NavigationLinkItem, NavigationMenu(), NavigationMenuItem, NavigationMenuSection, NavigationPreviewCard

### Community 53 - "stories/progress.rs"
Cohesion: 0.08
Nodes (22): ProgressBar(), ProgressSize, Lg, Md, Sm, ProgressVariant, Danger, Primary (+14 more)

### Community 54 - "ComponentOverride"
Cohesion: 0.20
Nodes (4): ComponentOverride, test_builder_preserves_overrides_when_changing_color(), test_explicit_glow_always_yields_class(), test_radius_scale_is_provider_class_only()

### Community 55 - "debounce.rs"
Cohesion: 0.20
Nodes (4): use_clipboard(), UseClipboard, test_clipboard_initial_state(), Clipboard()

### Community 56 - "area.rs"
Cohesion: 0.21
Nodes (8): ChartContainer(), ChartGrid(), ChartLegend(), ChartTooltip(), get_series_color(), series_line_style(), RadarChart(), RadarDataPoint

### Community 57 - "src/pin_input.rs"
Cohesion: 0.16
Nodes (7): PinInput(), PinInputProps, PinInputSize, PinInputType, Alphanumeric, Numeric, sanitize_pin_value()

### Community 58 - "get_series_color"
Cohesion: 0.18
Nodes (6): BubbleChart(), ScaleType, Linear, Log, ScatterChart(), ScatterPoint

### Community 59 - "ThemeBuilder"
Cohesion: 0.24
Nodes (4): Custom, test_invalid_hex_is_sanitised(), ThemeBuilder, is_valid_hex_color()

### Community 60 - "stories/code_highlight.rs"
Cohesion: 0.14
Nodes (15): HIGHLIGHTS, IN_CONTEXT_CODE, InContext(), rust_sample!, Minimal(), MINIMAL_CODE, PARTS_PROPS, PRELUDE (+7 more)

### Community 61 - "bottom_navigation/mod.rs"
Cohesion: 0.19
Nodes (7): BottomNavigation(), BottomNavigationProps, BottomNavPosition, Absolute, .ALL, Fixed, Relative

### Community 62 - "stories/hover_card.rs"
Cohesion: 0.09
Nodes (24): HoverCard(), HoverCardContext, HoverCardDropdown(), HoverCardPosition, Bottom, Left, Right, Top (+16 more)

### Community 63 - "JustifyContent"
Cohesion: 0.18
Nodes (9): JustifyContent, Center, End, SpaceAround, SpaceBetween, SpaceEvenly, Start, JUSTIFIES (+1 more)

### Community 64 - "stories/speed_dial.rs"
Cohesion: 0.08
Nodes (26): Fab(), SpeedDial(), SpeedDialAction(), SpeedDialContext, SpeedDialDirection, Down, Left, Right (+18 more)

### Community 65 - "pie.rs"
Cohesion: 0.23
Nodes (5): GaugeChart(), RadialBar(), DonutChart(), PieChart(), PieSegment

### Community 66 - "TreeNode"
Cohesion: 0.21
Nodes (4): TreeNode, TreeNodeRow(), TreeView(), mesh()

### Community 67 - "stories/context_menu.rs"
Cohesion: 0.14
Nodes (15): CHECK_PROPS, CHILDREN_CODE, CONTENT_PROPS, ContextMenuDemo(), IN_CONTEXT_CODE, InContext(), ITEM_PROPS, MENU_PROPS (+7 more)

### Community 68 - "src/drawer.rs"
Cohesion: 0.19
Nodes (10): Drawer(), DrawerPosition, Left, Right, DrawerSize, Lg, Md, Sm (+2 more)

### Community 69 - "modal.rs"
Cohesion: 0.20
Nodes (8): Modal(), ModalSize, Lg, Md, Sm, Xl, Xxl, SIZES

### Community 70 - "scroll_area.rs"
Cohesion: 0.10
Nodes (18): ScrollArea(), ScrollAreaProps, ScrollType, Always, Auto, Hover, Never, CHILDREN (+10 more)

### Community 71 - "4. Snippets"
Cohesion: 0.06
Nodes (34): 1.1 Hero concept: "Un hex, tutto il tema", 1.2 Presets, radius and glow demonstrated live, 1.3 Composed scenes versus isolated widgets, 1.4 Density, rhythm, type, 1.5 Motion moments, 1.6 Mobile (390), 1. Visual direction, 2.1 Navigation (+26 more)

### Community 73 - "bottom_navigation.rs"
Cohesion: 0.15
Nodes (14): BADGE_OPTS, BADGES, BottomNavDemo(), CHILDREN, Flat(), FLAT_CODE, IN_CONTEXT_CODE, InContext() (+6 more)

### Community 74 - "AlignItems"
Cohesion: 0.14
Nodes (9): AlignItems, Baseline, Center, End, Start, Stretch, Stack(), ALIGNS (+1 more)

### Community 75 - "Burger"
Cohesion: 0.22
Nodes (6): Burger(), BurgerSize, Lg, Md, Sm, SIZES

### Community 76 - "stories/checkbox.rs"
Cohesion: 0.10
Nodes (17): Checkbox(), CheckboxSize, CheckboxSizeExt, CheckboxDemo(), IN_CONTEXT_CODE, InContext(), PRELUDE, PROPS (+9 more)

### Community 77 - "src/context_menu.rs"
Cohesion: 0.33
Nodes (7): ContextMenu(), ContextMenuCheckboxItem(), ContextMenuContent(), ContextMenuContext, ContextMenuItem(), ContextMenuSeparator(), ContextMenuTrigger()

### Community 78 - "Locale"
Cohesion: 0.21
Nodes (3): Locale, LocaleProvider(), use_locale()

### Community 79 - "src/number_input.rs"
Cohesion: 0.31
Nodes (6): format_and_clamp_number(), format_number_string(), NumberInput(), test_format_and_clamp_over_max(), test_format_and_clamp_under_min(), test_format_and_clamp_within_bounds()

### Community 80 - "RadiusScale"
Cohesion: 0.16
Nodes (12): GlowMode, Expressive, Off, Subtle, GlowTile(), RadiusTile(), ScopedTheme(), SurfaceBox() (+4 more)

### Community 81 - "lint_design.py"
Cohesion: 0.17
Nodes (15): compile_scss(), extract_selectors(), main(), sass_command(), scan_source_classes(), compiled_classes(), lint_missing_classes(), lint_rust() (+7 more)

### Community 82 - "📦 Libreria UI (`crate/`)"
Cohesion: 0.20
Nodes (9): Avviare l'Espositore, 🖼️ Espositore (`showcase/`), Feature disponibili, Includere la libreria nei tuoi siti web, 🎨 Integrazione Asset SCSS (`STYLES`), 📦 Libreria UI (`crate/`), Shadow Design, 🎭 Sistema Temi Mantine-Style & ThemeBuilder (+1 more)

### Community 83 - "BarChart"
Cohesion: 0.22
Nodes (8): BarChart(), BarChartType, Default, Percent, Stacked, ChartOrientation, Horizontal, Vertical

### Community 84 - "stories/radio.rs"
Cohesion: 0.15
Nodes (14): CARD_PROPS, Cards(), CARDS_CODE, CHILDREN, GROUP_PROPS, Horizontal(), HORIZONTAL_CODE, IN_CONTEXT_CODE (+6 more)

### Community 85 - "heatmap.rs"
Cohesion: 0.36
Nodes (3): CalendarHeatmap(), Heatmap(), HeatmapCell

### Community 86 - "stories/fieldset.rs"
Cohesion: 0.11
Nodes (18): Fieldset(), FieldsetVariant, Default, Filled, Ungrouped, CHILDREN, FieldsetDemo(), IN_CONTEXT_CODE (+10 more)

### Community 87 - "use_theme"
Cohesion: 0.29
Nodes (7): ResizableDirection, Horizontal, Vertical, ResizableHandle(), ResizablePanel(), ResizablePanelGroup(), DIRECTIONS

### Community 88 - "skeletons.rs"
Cohesion: 0.12
Nodes (15): Skeleton(), SkeletonShape, Circle, Rectangle, Text, IN_CONTEXT_CODE, InContext(), PROPS (+7 more)

### Community 89 - "ElevationStyle"
Cohesion: 0.33
Nodes (4): ElevationStyle, Flat, NeonGlow, test_glow_strength_and_flat()

### Community 90 - "stories/transfer_list.rs"
Cohesion: 0.10
Nodes (16): TransferColumn(), TransferItem, TransferList(), available(), granted(), IN_CONTEXT_CODE, InContext(), ITEM_PROPS (+8 more)

### Community 91 - "theme_menu.rs"
Cohesion: 0.16
Nodes (12): actions(), SearchPalette(), THEME_CATEGORY, glow_from_key(), glow_key(), PRESETS, radius_from_key(), radius_key() (+4 more)

### Community 92 - "components/controls.rs"
Cohesion: 0.21
Nodes (13): CheckControl(), ChoicePicker(), ControlGroup(), ControlRow(), Controls(), Counter(), DemoCell(), OptionPicker() (+5 more)

### Community 93 - "Shadow Design (`shadow-design`)"
Cohesion: 0.29
Nodes (6): 💻 Esempio d'Uso, 🧩 Feature Flags Disponibili, 📦 Installazione in Cargo.toml, 🎨 Integrazione Diretta Asset SCSS in Dioxus 0.7, Shadow Design (`shadow-design`), 🎨 Sistema del Tema Mantine (`theme`)

### Community 94 - "PersonaProps"
Cohesion: 0.14
Nodes (7): ActivityComment, ActivityEvent, ActivityFeed(), ActivityItem(), PersonaProps, events(), team()

### Community 95 - "dividers.rs"
Cohesion: 0.12
Nodes (16): Divider(), Orientation, Horizontal, Vertical, IN_CONTEXT_CODE, InContext(), Labelled(), LABELLED_CODE (+8 more)

### Community 99 - "3. Recipes"
Cohesion: 0.14
Nodes (14): 3.10 Semantic surface (alert, toast, badge, status pill, callout): `@include sd-semantic(<sem>)`, 3.11 Value fill (progress, slider range, switch on, gauge, rating), 3.12 Text and labels, 3.13 Data visualisation, 3.1 Container: `@include sd-surface(1)`, 3.2 Floating panel: `@include sd-surface(2)` or `sd-surface(3)`, 3.3 Backdrop, 3.4 Item (menu item, list option, tab, segmented item, pagination page, tree node, nav link): `@include sd-item` (+6 more)

### Community 100 - "registry.rs"
Cohesion: 0.07
Nodes (22): ALL, all(), by_category(), CRATE_MANIFEST, find(), matches(), neighbours(), STORIES (+14 more)

### Community 101 - "src/tabs.rs"
Cohesion: 0.43
Nodes (5): Tab(), TabList(), TabListCtx, TabPanel(), Tabs()

### Community 105 - "stories/select.rs"
Cohesion: 0.13
Nodes (16): Combo(), COMBO_CODE, COMBO_PROPS, frameworks(), Grouped(), GROUPED_CODE, IN_CONTEXT_CODE, InContext() (+8 more)

### Community 106 - "stories/rating.rs"
Cohesion: 0.15
Nodes (14): COUNT_OPTS, COUNTS, FRACTION_OPTS, FRACTIONS, Icons(), ICONS_CODE, IN_CONTEXT_CODE, InContext() (+6 more)

### Community 108 - "DataTableColumn<T>"
Cohesion: 0.14
Nodes (5): DataTableColumn<T>, TableAlign, Center, Left, Right

### Community 109 - "breadcrumbs.rs"
Cohesion: 0.15
Nodes (13): BREADCRUMBS_PROPS, Collapsed(), COLLAPSED_CODE, Composed(), COMPOSED_CODE, DATA_PROPS, IN_CONTEXT_CODE, InContext() (+5 more)

### Community 110 - "stories/date_picker.rs"
Cohesion: 0.16
Nodes (13): CALENDAR_PROPS, DATE_PROPS, DateDemo(), IN_CONTEXT_CODE, InContext(), Inline(), INLINE_CODE, PICKER_PROPS (+5 more)

### Community 112 - "stories/stack.rs"
Cohesion: 0.15
Nodes (13): ALIGN_OPTS, Centered(), CENTERED_CODE, CHILDREN, GAP_OPTS, Gaps(), GAPS_CODE, GAPS (+5 more)

### Community 113 - "reactive_hooks_tests.rs"
Cohesion: 0.38
Nodes (4): use_window_scroll(), UseWindowScroll, test_window_scroll_initial_value(), WindowScroll()

### Community 114 - "adr/README.md"
Cohesion: 0.17
Nodes (6): Architecture Decision Records, Rules, Template, Shadow Design: design documentation, Note specifiche per Showcase, Showcase — Istruzioni Agenti

### Community 115 - "Shadow Design — Showcase App"
Cohesion: 0.50
Nodes (3): 🎨 Architettura Stili, Shadow Design — Showcase App, 🚀 Sviluppo Locale (Integrazione Diretta Dioxus 0.7)

### Community 116 - "Signal"
Cohesion: 0.16
Nodes (16): Signal, BoolKnob(), ChoiceKnob(), ColorKnob(), Knob(), KnobPanel(), NumberKnob(), SEGMENT_MAX (+8 more)

### Community 117 - "pages/typography.rs"
Cohesion: 0.29
Nodes (6): HEADINGS_CODE, INLINE_CODE, Role, ROLES, TEXT_CODE, TypographyPage()

### Community 118 - "ButtonSize"
Cohesion: 0.16
Nodes (9): Button(), ButtonSize, Lg, Md, Sm, Xl, Xs, SIZES (+1 more)

### Community 119 - "home.rs"
Cohesion: 0.15
Nodes (11): Home(), PRESETS, PresetTile(), Stat(), AuthCard(), DEMO_CODE, InboxPane(), Message (+3 more)

### Community 121 - "sections.rs"
Cohesion: 0.16
Nodes (19): LivePlayground(), StoryHead(), StoryPage(), StoryView(), Example, PropGroup, Recipe, Span (+11 more)

### Community 122 - "Showcase revamp: batch migration guide (phase 2)"
Cohesion: 0.12
Nodes (15): 1. Where things are, 2. The stub contract, 3. Steps for one story, 4.1 Controls and code generation, 4.2 Gotchas the compiler or the tests will catch, 4.3 Shared vocabulary you may use, 4.4 Trivial components, 4. Engine cheat sheet (+7 more)

### Community 123 - "alerts.rs"
Cohesion: 0.17
Nodes (12): AlertDemo(), IN_CONTEXT_CODE, InContext(), PRELUDE, PROPS, render(), VARIANT_OPTS, Variants() (+4 more)

### Community 124 - "Knobs"
Cohesion: 0.13
Nodes (20): bool_template_on_by_default_is_printed(), changed_knob_is_printed_and_highlighted(), children_line_is_highlighted_when_its_knob_changes(), defaults_print_only_children(), generate(), Generated, Knobs, literal() (+12 more)

### Community 125 - "components/layout.rs"
Cohesion: 0.15
Nodes (13): ColorSwatchButton(), glow_key(), radius_key(), SIDEBAR_SCROLL_JS, SidebarLink(), SidebarNav(), ThemeToolbar(), CATEGORIES (+5 more)

### Community 126 - "badges.rs"
Cohesion: 0.17
Nodes (12): IN_CONTEXT_CODE, InContext(), Inline(), INLINE_CODE, PROPS, render(), STATUS_CODE, StatusPills() (+4 more)

### Community 127 - "buttons.rs"
Cohesion: 0.17
Nodes (12): IN_CONTEXT_CODE, InContext(), PROPS, render(), SIZE_OPTS, Sizes(), SIZES_CODE, States() (+4 more)

### Community 128 - "stories/facepile.rs"
Cohesion: 0.17
Nodes (12): Clickable(), CLICKABLE_CODE, FacepileDemo(), IN_CONTEXT_CODE, InContext(), Overflow(), OVERFLOW_CODE, PRELUDE (+4 more)

### Community 129 - "stories/activity_feed.rs"
Cohesion: 0.18
Nodes (11): Compact(), COMPACT_CODE, Empty(), EMPTY_CODE, EVENT_PROPS, FEED_PROPS, IN_CONTEXT_CODE, InContext() (+3 more)

### Community 130 - "nav_link.rs"
Cohesion: 0.18
Nodes (11): BADGE_VARIANT_OPTS, BADGE_VARIANTS, Groups(), GROUPS_CODE, IN_CONTEXT_CODE, InContext(), LINK_PROPS, MENU_PROPS (+3 more)

### Community 131 - "SegmentedControl"
Cohesion: 0.15
Nodes (8): SegmentedControl(), SegmentedControlSize, Lg, Md, Sm, SegmentOption, ranges(), SIZES

### Community 132 - "app/layout.rs"
Cohesion: 0.25
Nodes (3): Header(), Layout(), SCROLL_ACTIVE_JS

### Community 133 - "CommandAction"
Cohesion: 0.17
Nodes (3): CommandAction, CommandPalette(), console_actions()

### Community 134 - "ColorFormat"
Cohesion: 0.29
Nodes (7): ColorFormat, Hex, Hsl, Rgb, ColorInputProps, ColorPickerProps, FORMATS

### Community 135 - "ListMarker"
Cohesion: 0.15
Nodes (12): ListMarker, Circle, Custom, Decimal, Disc, LowerAlpha, LowerRoman, None (+4 more)

### Community 136 - "schema.rs"
Cohesion: 0.14
Nodes (12): Emit, Always, IfChanged, Skip, Template, opt(), prop(), Stage (+4 more)

### Community 137 - "click_outside.rs"
Cohesion: 0.29
Nodes (4): use_click_outside(), use_hotkey(), Layout(), Hotkey()

### Community 138 - "ADR-0005: Three type roles: Kanit, Onest, Source Code Pro"
Cohesion: 0.40
Nodes (4): ADR-0005: Three type roles: Kanit, Onest, Source Code Pro, Consequences, Context, Decision

### Community 139 - "ADR-0006: The showcase is data-driven documentation built from the library"
Cohesion: 0.40
Nodes (4): ADR-0006: The showcase is data-driven documentation built from the library, Consequences, Context, Decision

### Community 140 - "7. Regole di Sviluppo"
Cohesion: 0.40
Nodes (5): 7. Regole di Sviluppo, Aggiungere o modificare un componente, Concorrenza (`clippy.toml`), Dioxus 0.7, Lint

### Community 142 - "ADR-0002: One design language: Lit Glass"
Cohesion: 0.50
Nodes (4): ADR-0002: One design language: Lit Glass, Consequences, Context, Decision

### Community 143 - "Element"
Cohesion: 0.17
Nodes (8): use_idle(), use_intersection_observer(), use_media_query(), ClickOutside(), example(), Idle(), Intersection(), MediaQuery()

### Community 145 - "ToolbarAction"
Cohesion: 0.23
Nodes (4): OverflowSet(), ResponsiveToolbar(), ToolbarAction, actions()

### Community 146 - "stories/copy_button.rs"
Cohesion: 0.21
Nodes (9): IN_CONTEXT_CODE, InContext(), Inline(), INLINE_CODE, KEYS, Labels(), LABELS_CODE, PROPS (+1 more)

### Community 147 - "accordion.rs"
Cohesion: 0.13
Nodes (16): ACCORDION_PROPS, CHILDREN, COLLAPSIBLE_CODE, COLLAPSIBLE_PROPS, CollapsibleDemo(), CONTROL_PROPS, Controlled(), CONTROLLED_CODE (+8 more)

### Community 148 - "ButtonVariant"
Cohesion: 0.18
Nodes (8): ButtonVariant, Danger, Ghost, Outline, Primary, Secondary, Success, VARIANTS

### Community 149 - "use_notification"
Cohesion: 0.17
Nodes (14): use_notification(), AlertDialogDemo(), Confirm(), CONFIRM_CODE, CONFIRM_PROPS, Extra(), EXTRA_CODE, ICON_OPTS (+6 more)

### Community 150 - "animations.rs"
Cohesion: 0.13
Nodes (16): CARD_CHILDREN, CLASS_CONSTANTS_CODE, ClassConstants(), CONST_PROPS, Entrances(), ENTRANCES_CODE, FLOATING_LAYERS_CODE, FloatingLayers() (+8 more)

### Community 151 - "app_shell.rs"
Cohesion: 0.11
Nodes (19): ASIDE_CODE, CHILDREN, FOOTER_CODE, FOOTER_PROPS, HEADER_CODE, HEADER_ONLY_CODE, HEADER_PROPS, HeaderOnly() (+11 more)

### Community 152 - "$schema"
Cohesion: 0.10
Nodes (18): $schema, Custom(), CUSTOM_CODE, IN_CONTEXT_CODE, InContext(), PROPS, RATIO_OPTS, RATIOS (+10 more)

### Community 153 - "attachment_cards.rs"
Cohesion: 0.14
Nodes (15): ATTACHMENT_PROPS, AttachmentDemo(), Contact(), CONTACT_CODE, CONTACT_PROPS, ICON_OPTS, ICONS, IN_CONTEXT_CODE (+7 more)

### Community 154 - "stories/burger.rs"
Cohesion: 0.19
Nodes (11): BurgerDemo(), COLORS, MOBILE_HEADER_CODE, MobileHeader(), PRELUDE, PROPS, render(), Sizes() (+3 more)

### Community 155 - "step.rs"
Cohesion: 0.25
Nodes (8): StepperStep(), StepperStepProps, StepState, Active, Completed, Error, Upcoming, STATES

### Community 156 - "stories/carousel.rs"
Cohesion: 0.15
Nodes (14): CAROUSEL_PROPS, CarouselDemo(), Charts(), CHARTS_CODE, hours(), IN_CONTEXT_CODE, InContext(), NO_LOOP_CODE (+6 more)

### Community 157 - "RadiusScale"
Cohesion: 0.20
Nodes (4): RadiusScale, Medium, Sharp, Soft

### Community 158 - "PropDoc"
Cohesion: 0.09
Nodes (30): Area(), AREA_CODE, Bar(), BAR_CODE, CARTESIAN_PROPS, DATA_PROPS, example(), GAUGE_PROPS (+22 more)

### Community 160 - "PresenceStatus"
Cohesion: 0.22
Nodes (7): Avatar(), PresenceStatus, Busy, DirectP2P, Offline, Online, STATUSES

### Community 161 - "stories/chip.rs"
Cohesion: 0.14
Nodes (15): CHIP_PROPS, ChipDemo(), GROUP_PROPS, IN_CONTEXT_CODE, InContext(), Multiple(), MULTIPLE_CODE, PRELUDE (+7 more)

### Community 162 - "stories/close_button.rs"
Cohesion: 0.22
Nodes (8): Dismiss(), DISMISS_CODE, PROPS, render(), SIZE_OPTS, VARIANT_OPTS, Variants(), VARIANTS_CODE

### Community 163 - "stories/coachmark.rs"
Cohesion: 0.15
Nodes (14): Bubble(), BUBBLE_CODE, BUBBLE_PROPS, CoachmarkDemo(), FIXED, IN_CONTEXT_CODE, InContext(), LAST_STEP_CODE (+6 more)

### Community 164 - "use_theme"
Cohesion: 0.11
Nodes (16): Carousel(), CarouselSlide(), Dropzone(), FileMetadata, use_theme(), VirtualList(), copy_text(), CurrentThemeTile() (+8 more)

### Community 165 - "stories/color_picker.rs"
Cohesion: 0.14
Nodes (15): Field(), FIELD_CODE, FORMAT_OPTS, IN_CONTEXT_CODE, InContext(), INPUT_PROPS, ONE_HEX_CODE, OneHex() (+7 more)

### Community 166 - "stories/command_palette.rs"
Cohesion: 0.19
Nodes (11): ACTION_PROPS, IN_CONTEXT_CODE, InContext(), MULTI_PICK_CODE, MultiPick(), PALETTE_PROPS, PaletteDemo(), PRELUDE (+3 more)

### Community 167 - "stories/document_card.rs"
Cohesion: 0.17
Nodes (12): DocumentDemo(), IN_CONTEXT_CODE, InContext(), PRELUDE, Preview(), PREVIEW_CODE, PROPS, render() (+4 more)

### Community 168 - "stories/drawer.rs"
Cohesion: 0.16
Nodes (13): BODY_CODE, DrawerDemo(), Filters(), FILTERS_CODE, IN_CONTEXT_CODE, InContext(), NAV_LEFT_CODE, NavLeft() (+5 more)

### Community 169 - "stories/dropdown.rs"
Cohesion: 0.14
Nodes (15): DIVIDER_PROPS, DROPDOWN_PROPS, DropdownDemo(), IN_CONTEXT_CODE, InContext(), ITEM_PROPS, ITEMS_CODE, PLACEMENT_OPTS (+7 more)

### Community 170 - "stories/dropzone.rs"
Cohesion: 0.21
Nodes (9): FILE_PROPS, IN_CONTEXT_CODE, InContext(), PROPS, render(), States(), STATES_CODE, Uploads() (+1 more)

### Community 171 - "src/coachmark.rs"
Cohesion: 0.29
Nodes (7): Coachmark(), CoachmarkPosition, Bottom, Left, Right, Top, TeachingBubble()

### Community 172 - "SyntaxTokenKind"
Cohesion: 0.20
Nodes (9): SyntaxTokenKind, Comment, Function, Keyword, Number, Operator, Plain, String (+1 more)

### Community 173 - "stories/grid.rs"
Cohesion: 0.12
Nodes (18): ALIGN_OPTS, CHILDREN, COL_PROPS, GRID_PROPS, GUTTER_OPTS, IN_CONTEXT_CODE, InContext(), JUSTIFY_OPTS (+10 more)

### Community 174 - "stories/group.rs"
Cohesion: 0.13
Nodes (17): ALIGN_OPTS, Between(), BETWEEN_CODE, CHILDREN, GAP_OPTS, GAPS, Grow(), GROW_CODE (+9 more)

### Community 175 - "stories/highlight.rs"
Cohesion: 0.16
Nodes (12): CASE_CODE, CaseSensitivity(), COLOR_OPTS, COLORS, COMMANDS, IN_CONTEXT_CODE, InContext(), PROPS (+4 more)

### Community 176 - "ChoiceOption"
Cohesion: 0.13
Nodes (17): COLOR_OPTS, SIZE_OPTS, HIGHLIGHT_OPTS, COLOR_OPTS, Colors(), COLORS_CODE, IN_CONTEXT_CODE, InContext() (+9 more)

### Community 177 - "Lit Glass: the Shadow Design language"
Cohesion: 0.40
Nodes (5): 1. Principles, 2. Depth ladder, 4. Glow budget, 5. Motion, Lit Glass: the Shadow Design language

### Community 178 - "Token reference"
Cohesion: 0.17
Nodes (12): Accent (theme-computed from one hex), Breakpoints (SCSS only, because media queries can't read variables), Charts, Code (component-requested), Motion, Radius (scale class set by `ThemeProvider`), Semantic (fixed, never follow the accent), Shadows & glow (+4 more)

### Community 179 - "inputs.rs"
Cohesion: 0.15
Nodes (14): Icons(), ICONS_CODE, IN_CONTEXT_CODE, InContext(), InputDemo(), LIVE_VALIDATION_CODE, LiveValidation(), PRELUDE (+6 more)

### Community 180 - "stories/keytip.rs"
Cohesion: 0.24
Nodes (7): CHILDREN_CODE, Disabled(), DISABLED_CODE, PROPS, render(), Toolbar(), TOOLBAR_CODE

### Community 182 - "TooltipPosition"
Cohesion: 0.24
Nodes (7): Tooltip(), TooltipPosition, Bottom, Left, Right, Top, POSITIONS

### Community 183 - "lists.rs"
Cohesion: 0.13
Nodes (16): Horizontal(), HORIZONTAL_CODE, IN_CONTEXT_CODE, InContext(), ITEM_ICONS_CODE, ITEM_PROPS, ItemIcons(), ITEMS_CODE (+8 more)

### Community 185 - "notifications.rs"
Cohesion: 0.12
Nodes (16): CHILDREN, DURATION_OPTS, DURATIONS, HANDLE_PROPS, IN_CONTEXT_CODE, InContext(), NotifyDemo(), PRELUDE (+8 more)

### Community 186 - "prelude"
Cohesion: 0.07
Nodes (9): Center(), Chunk, Highlight(), split_highlight(), Keytip(), MarqueeRect, MarqueeSelection(), Navbar() (+1 more)

### Community 187 - "Component checklist"
Cohesion: 0.33
Nodes (5): Before writing, Component checklist, Rust (`crate/src/<feature>.rs` or `<feature>/mod.rs`), SCSS (`crate/scss/components/_<feature>.scss`), Verify

### Community 188 - "ADR-0001: Record decisions in ADRs and bind agents to them"
Cohesion: 0.40
Nodes (4): ADR-0001: Record decisions in ADRs and bind agents to them, Consequences, Context, Decision

### Community 189 - "ADR-0004: Component styling contract"
Cohesion: 0.40
Nodes (4): ADR-0004: Component styling contract, Consequences, Context, Decision

### Community 190 - "7. Regole di Sviluppo"
Cohesion: 0.40
Nodes (5): 7. Regole di Sviluppo, Aggiungere o modificare un componente, Concorrenza (`clippy.toml`), Dioxus 0.7, Lint

### Community 191 - "stories/loading_overlay.rs"
Cohesion: 0.21
Nodes (9): CHILDREN, IN_CONTEXT_CODE, InContext(), Minimal(), MINIMAL_CODE, PROPS, Refresh(), REFRESH_CODE (+1 more)

### Community 192 - "stories/marquee_selection.rs"
Cohesion: 0.15
Nodes (12): FILES, GRID_CODE, IN_CONTEXT_CODE, InContext(), MarqueeDemo(), NODES, PRELUDE, PROPS (+4 more)

### Community 193 - "src/code_highlight.rs"
Cohesion: 0.28
Nodes (6): CodeBlock(), CodeHighlight(), SyntaxToken, tokenize_line(), CopyButton(), test_code_highlight_tokenizer()

### Community 194 - "stories/media_card.rs"
Cohesion: 0.16
Nodes (13): ACTION_AREA_CODE, ActionArea(), AREA_PROPS, IN_CONTEXT_CODE, InContext(), MEDIA_PROPS, MediaDemo(), PRELUDE (+5 more)

### Community 195 - "ADR-0003: Runtime CSS tokens are the only theming mechanism"
Cohesion: 0.50
Nodes (4): ADR-0003: Runtime CSS tokens are the only theming mechanism, Consequences, Context, Decision

### Community 196 - "menubar.rs"
Cohesion: 0.15
Nodes (14): BAR_PROPS, Checkable(), CHECKABLE_CODE, CHILDREN, COMMAND_BAR_CODE, CommandBarDemo(), IN_CONTEXT_CODE, InContext() (+6 more)

### Community 199 - "modals.rs"
Cohesion: 0.17
Nodes (12): Blocking(), BLOCKING_CODE, BODY_CODE, IN_CONTEXT_CODE, InContext(), ModalDemo(), NO_TITLE_CODE, NoTitle() (+4 more)

### Community 200 - "stories/navbar.rs"
Cohesion: 0.19
Nodes (10): BRAND_CODE, CHILDREN, IN_CONTEXT_CODE, InContext(), Minimal(), MINIMAL_CODE, PROPS, render() (+2 more)

### Community 201 - "stories/navigation_menu.rs"
Cohesion: 0.18
Nodes (10): IN_CONTEXT_CODE, InContext(), ITEM_PROPS, LINK_PROPS, MEGA_MENU_CODE, MegaMenu(), MENU_PROPS, PREVIEW_PROPS (+2 more)

### Community 202 - "stories/number_input.rs"
Cohesion: 0.16
Nodes (13): Affixes(), AFFIXES_CODE, IN_CONTEXT_CODE, InContext(), NumberDemo(), PRECISION_OPTS, PRECISIONS, PRELUDE (+5 more)

### Community 203 - "stories/overflow_set.rs"
Cohesion: 0.19
Nodes (11): ACTION_PROPS, Disabled(), DISABLED_CODE, Extra(), EXTRA_CODE, IN_CONTEXT_CODE, InContext(), PRELUDE (+3 more)

### Community 204 - "pagination.rs"
Cohesion: 0.17
Nodes (12): Edges(), EDGES_CODE, IN_CONTEXT_CODE, InContext(), PaginationDemo(), PRELUDE, PROPS, render() (+4 more)

### Community 205 - "stories/pin_input.rs"
Cohesion: 0.14
Nodes (15): Complete(), COMPLETE_CODE, IN_CONTEXT_CODE, InContext(), LENGTH_OPTS, LENGTHS, Modes(), MODES_CODE (+7 more)

### Community 207 - "OpsDashboard"
Cohesion: 0.33
Nodes (7): columns(), Metric(), NodeRow, NODES, OpsDashboard(), status_badge(), traffic()

### Community 208 - "reactive_hooks.rs"
Cohesion: 0.14
Nodes (12): CLICK_OUTSIDE_CODE, CLIPBOARD_CODE, CLIPBOARD_PROPS, DEBOUNCE_CODE, HOOK_PROPS, HOTKEY_CODE, IDLE_CODE, INTERSECTION_CODE (+4 more)

### Community 209 - "stories/resizable.rs"
Cohesion: 0.16
Nodes (12): CHILDREN, DIRECTION_OPTS, GROUP_PROPS, HANDLE_PROPS, IN_CONTEXT_CODE, InContext(), PANEL_PROPS, render() (+4 more)

### Community 210 - "GridAlign"
Cohesion: 0.25
Nodes (7): GridAlign, Baseline, Center, End, Start, Stretch, ALIGNS

### Community 211 - "ListSize"
Cohesion: 0.25
Nodes (7): ListSize, Lg, Md, Sm, Xl, Xs, SIZES

### Community 212 - "segmented_controls.rs"
Cohesion: 0.17
Nodes (12): IN_CONTEXT_CODE, InContext(), OPTION_PROPS, PRELUDE, PROPS, render(), SegmentedDemo(), SIZE_OPTS (+4 more)

### Community 213 - "stories/slider.rs"
Cohesion: 0.16
Nodes (13): IN_CONTEXT_CODE, InContext(), PRELUDE, Range(), RANGE_CODE, RANGE_PROPS, render(), SLIDER_PROPS (+5 more)

### Community 214 - "spoiler.rs"
Cohesion: 0.21
Nodes (9): CHILDREN, Controlled(), CONTROLLED_CODE, IN_CONTEXT_CODE, InContext(), PROPS, render(), Short() (+1 more)

### Community 215 - "ListSpacing"
Cohesion: 0.25
Nodes (7): ListSpacing, Lg, Md, Sm, Xl, Xs, SPACINGS

### Community 216 - "stories/switch.rs"
Cohesion: 0.18
Nodes (12): IN_CONTEXT_CODE, InContext(), Maintenance(), MAINTENANCE_CODE, Preferences(), PREFERENCES_CODE, PRELUDE, PROPS (+4 more)

### Community 217 - "tables.rs"
Cohesion: 0.14
Nodes (17): COLUMN_PROPS, columns(), DATA_PROPS, Empty(), EMPTY_CODE, IN_CONTEXT_CODE, InContext(), Peer (+9 more)

### Community 218 - "stories/tabs.rs"
Cohesion: 0.13
Nodes (16): BADGE_OPTS, BADGES, CHILDREN, IN_CONTEXT_CODE, InContext(), KEEP_MOUNTED_CODE, KeepMounted(), LIST_PROPS (+8 more)

### Community 219 - "stories/textarea.rs"
Cohesion: 0.17
Nodes (13): AUTO_PROPS, AUTO_RESIZE_CODE, AutoResize(), Count(), COUNT_CODE, IN_CONTEXT_CODE, InContext(), PRELUDE (+5 more)

### Community 220 - "stories/tooltip.rs"
Cohesion: 0.17
Nodes (12): CHILDREN_CODE, Controlled(), CONTROLLED_CODE, Directions(), DIRECTIONS_CODE, IN_CONTEXT_CODE, InContext(), POSITION_OPTS (+4 more)

### Community 221 - "GridJustify"
Cohesion: 0.29
Nodes (7): GridJustify, Center, End, SpaceAround, SpaceBetween, Start, JUSTIFIES

### Community 222 - "trees.rs"
Cohesion: 0.17
Nodes (12): IN_CONTEXT_CODE, InContext(), NODE_INFO, NODE_PROPS, Plain(), PLAIN_CODE, PRELUDE, Project() (+4 more)

### Community 223 - "stories/virtual_list.rs"
Cohesion: 0.13
Nodes (16): Compact(), COMPACT_CODE, COUNT_OPTS, COUNTS, IN_CONTEXT_CODE, InContext(), level_badge(), LOG (+8 more)

### Community 225 - "hooks/mod.rs"
Cohesion: 0.38
Nodes (5): use_debounced_signal(), use_throttle(), use_local_storage(), Debounce(), LocalStorage()

### Community 226 - "sidebar.rs"
Cohesion: 0.53
Nodes (4): SidebarLink(), SidebarNav(), ThemeCard(), preset_name()

### Community 228 - "CalendarViewMode"
Cohesion: 0.50
Nodes (4): CalendarViewMode, Day, Month, Year

## Knowledge Gaps
- **411 isolated node(s):** `shadow-design`, `Sm`, `Md`, `Lg`, `Default` (+406 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1639 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `IconKind` connect `IconKind` to `Route`, `AvatarSize`, `notification.rs`, `CommandAction`, `src/rich_text.rs`, `schema.rs`, `complex_tooling_tests.rs`, `SvgIcon`, `ToolbarAction`, `src/overflow_set.rs`, `BadgeVariant`, `use_notification`, `ControlSize`, `attachment_cards.rs`, `Date`, `step.rs`, `ComponentSpacing`, `stories/carousel.rs`, `avatar.rs`, `stories/affix.rs`, `use_theme`, `src/select.rs`, `AlertVariant`, `breadcrumbs/mod.rs`, `src/chip.rs`, `src/coachmark.rs`, `CloseButton`, `src/radio.rs`, `stories/chat.rs`, `NavigationLinkItem`, `stories/speed_dial.rs`, `TreeNode`, `stories/checkbox.rs`, `src/context_menu.rs`, `src/number_input.rs`, `use_theme`, `stories/transfer_list.rs`, `PersonaProps`, `src/rating.rs`, `sidebar.rs`, `src/alert_dialog.rs`, `registry.rs`, `src/loading_overlay.rs`, `src/tabs.rs`, `Signal`, `ButtonSize`, `spoiler/mod.rs`, `components/layout.rs`?**
  _High betweenness centrality (0.051) - this node is a cross-community bridge._
- **What connects `shadow-design`, `Sm`, `Md` to the rest of the system?**
  _411 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Route` be split into smaller, more focused modules?**
  _Cohesion score 0.14666666666666667 - nodes in this community are weakly interconnected._
- **Why does `$schema` connect `$schema` to `src/rich_text.rs`, `stepper.rs`, `avatars.rs`, `ComponentSpacing`, `timeline.rs`, `stories/affix.rs`, `PopoverPosition`, `CardShadow`, `stories/chat.rs`, `UseForm<T>`, `stories/progress.rs`, `stories/code_highlight.rs`, `stories/hover_card.rs`, `stories/speed_dial.rs`, `stories/context_menu.rs`, `scroll_area.rs`, `bottom_navigation.rs`, `stories/checkbox.rs`, `stories/radio.rs`, `stories/fieldset.rs`, `skeletons.rs`, `stories/transfer_list.rs`, `dividers.rs`, `stories/select.rs`, `stories/rating.rs`, `breadcrumbs.rs`, `stories/date_picker.rs`, `stories/stack.rs`, `alerts.rs`, `badges.rs`, `buttons.rs`, `stories/facepile.rs`, `stories/activity_feed.rs`, `nav_link.rs`, `stories/copy_button.rs`, `accordion.rs`, `use_notification`, `animations.rs`, `app_shell.rs`, `attachment_cards.rs`, `stories/burger.rs`, `stories/carousel.rs`, `PropDoc`, `stories/chip.rs`, `stories/close_button.rs`, `stories/coachmark.rs`, `stories/color_picker.rs`, `stories/command_palette.rs`, `stories/document_card.rs`, `stories/drawer.rs`, `stories/dropdown.rs`, `stories/dropzone.rs`, `stories/grid.rs`, `stories/group.rs`, `stories/highlight.rs`, `ChoiceOption`, `inputs.rs`, `stories/keytip.rs`, `lists.rs`, `notifications.rs`, `stories/loading_overlay.rs`, `stories/marquee_selection.rs`, `stories/media_card.rs`, `menubar.rs`, `modals.rs`, `stories/navbar.rs`, `stories/navigation_menu.rs`, `stories/number_input.rs`, `stories/overflow_set.rs`, `pagination.rs`, `stories/pin_input.rs`, `reactive_hooks.rs`, `stories/resizable.rs`, `segmented_controls.rs`, `stories/slider.rs`, `spoiler.rs`, `stories/switch.rs`, `tables.rs`, `stories/tabs.rs`, `stories/textarea.rs`, `stories/tooltip.rs`, `trees.rs`, `stories/virtual_list.rs`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Should `lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.028169014084507043 - nodes in this community are weakly interconnected._
- **Why does `Signal` connect `Signal` to `IconKind`, `SegmentedControl`, `notification.rs`, `CommandAction`, `ColorFormat`, `app/layout.rs`, `src/rich_text.rs`, `table.rs`, `SvgIcon`, `menubar/mod.rs`, `Element`, `app_shell/mod.rs`, `Switch`, `src/overflow_set.rs`, `accordion/mod.rs`, `BadgeVariant`, `ControlSize`, `src/slider.rs`, `Date`, `Theme`, `PopoverPosition`, `use_theme`, `src/select.rs`, `breadcrumbs/mod.rs`, `src/chip.rs`, `super`, `src/radio.rs`, `UseForm<T>`, `debounce.rs`, `area.rs`, `src/pin_input.rs`, `stories/hover_card.rs`, `stories/speed_dial.rs`, `TreeNode`, `src/drawer.rs`, `modal.rs`, `Burger`, `stories/checkbox.rs`, `src/context_menu.rs`, `Locale`, `src/number_input.rs`, `stories/transfer_list.rs`, `theme_menu.rs`, `components/controls.rs`, `src/rating.rs`, `hooks/mod.rs`, `src/textarea.rs`, `src/alert_dialog.rs`, `src/tabs.rs`, `reactive_hooks_tests.rs`, `spoiler/mod.rs`, `components/layout.rs`?**
  _High betweenness centrality (0.031) - this node is a cross-community bridge._
- **Should `IconKind` be split into smaller, more focused modules?**
  _Cohesion score 0.034482758620689655 - nodes in this community are weakly interconnected._