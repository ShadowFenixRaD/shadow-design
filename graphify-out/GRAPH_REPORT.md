# Graph Report - shadow-design  (2026-10-24)

## Corpus Check
- 375 files · ~199,685 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 107 file(s) not represented in the graph (top: .scss 94, (none) 7, .toml 2)

## Summary
- 2791 nodes · 5039 edges · 225 communities (131 shown, 94 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 99 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `5815872d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Route
- IconKind
- avatar.rs
- notification.rs
- ChoicePicker
- AnimationVariant
- ListMarker
- src/rich_text.rs
- SpacingToken
- table.rs
- complex_tooling_tests.rs
- GridSpacing
- SvgIcon
- menubar/mod.rs
- stepper/mod.rs
- app_shell/mod.rs
- page.rs
- src/color_picker.rs
- src/overflow_set.rs
- accordion/mod.rs
- BadgeVariant
- FeatureBanner
- PropRow
- ControlSize
- src/slider.rs
- ButtonVariant
- IndicatorColor
- ComponentSpacing
- Theme
- timeline.rs
- HeadingOrder
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
- DeliveryStatus
- UseForm<T>
- NavigationLinkItem
- ProgressVariant
- ComponentOverride
- hooks/mod.rs
- area.rs
- src/pin_input.rs
- get_series_color
- ThemeBuilder
- MarqueeSelection
- bottom_navigation/mod.rs
- HoverCardPosition
- JustifyContent
- src/speed_dial.rs
- pie.rs
- TreeNode
- PageHeader
- src/drawer.rs
- modal.rs
- ScrollType
- 4. Snippets
- AffixPosition
- AlignItems
- Burger
- CheckboxSize
- src/context_menu.rs
- Locale
- src/number_input.rs
- RadiusScale
- lint_design.py
- 📦 Libreria UI (`crate/`)
- BarChart
- GlowMode
- heatmap.rs
- FieldsetVariant
- use_theme
- SkeletonShape
- ElevationStyle
- src/transfer_list.rs
- theme_menu.rs
- components/controls.rs
- Shadow Design (`shadow-design`)
- PersonaProps
- Orientation
- src/rating.rs
- src/textarea.rs
- 3. Recipes
- registry.rs
- src/tabs.rs
- stories/select.rs
- RadarChart
- ContactCard
- src/highlight.rs
- Playground
- src/center.rs
- reactive_hooks_tests.rs
- adr/README.md
- Shadow Design — Showcase App
- Signal
- pages/typography.rs
- TrendDirection
- home.rs
- spoiler/mod.rs
- sections.rs
- Showcase revamp: batch migration guide (phase 2)
- pages/stepper.rs
- Knobs
- components/layout.rs
- pages/scroll_area.rs
- buttons.rs
- Category
- nav.rs
- schema.rs
- SegmentedControl
- app/layout.rs
- Kind
- ColorFormat
- RatioPreset
- Emit
- click_outside.rs
- ADR-0005: Three type roles: Kanit, Onest, Source Code Pro
- ADR-0006: The showcase is data-driven documentation built from the library
- 7. Regole di Sviluppo
- Switch
- ADR-0002: One design language: Lit Glass
- pages/resizable.rs
- not_found.rs
- settings_form.rs
- AriaHeading
- use_theme
- Lit Glass: the Shadow Design language
- Token reference
- STYLES
- shadow-design
- $schema
- prelude
- Component checklist
- ADR-0001: Record decisions in ADRs and bind agents to them
- ADR-0004: Component styling contract
- 7. Regole di Sviluppo
- ADR-0003: Runtime CSS tokens are the only theming mechanism
- design-system.md

## God Nodes (most connected - your core abstractions)
1. `IconKind` - 224 edges
2. `Signal` - 92 edges
3. `PropRow` - 87 edges
4. `$schema` - 86 edges
5. `ShowcaseTabs()` - 86 edges
6. `PageHeader()` - 85 edges
7. `FeatureBanner()` - 84 edges
8. `Playground` - 64 edges
9. `Theme` - 50 edges
10. `Icon()` - 49 edges

## Surprising Connections (you probably didn't know these)
- `status_badge()` --references--> `BadgeVariant`  [EXTRACTED]
  showcase/src/scenes/ops_dashboard.rs → crate/src/badge.rs
- `VARIANTS` --references--> `ButtonVariant`  [EXTRACTED]
  showcase/src/stories/buttons.rs → crate/src/button.rs
- `ReactiveHooksPage()` --calls--> `use_hotkey()`  [INFERRED]
  showcase/legacy/pages/reactive_hooks.rs → crate/src/hooks/hotkey.rs
- `Layout()` --calls--> `use_hotkey()`  [INFERRED]
  showcase/src/app/layout.rs → crate/src/hooks/hotkey.rs
- `Home()` --calls--> `use_media_query()`  [INFERRED]
  showcase/src/pages/home.rs → crate/src/hooks/media_query.rs

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

## Communities (225 total, 94 thin omitted)

### Community 0 - "Route"
Cohesion: 0.16
Nodes (16): App(), FAVICON, HEADER_SVG, Route, Home, IconsPage, NotFound, StoryPage (+8 more)

### Community 2 - "IconKind"
Cohesion: 0.03
Nodes (56): AccordionControl(), IconKind, AlertCircle, AlertTriangle, .ALL, Bluetooth, Box, Calendar (+48 more)

### Community 3 - "avatar.rs"
Cohesion: 0.09
Nodes (14): Avatar(), AvatarGroup(), AvatarSize, Lg, Md, Sm, Xl, Xs (+6 more)

### Community 4 - "notification.rs"
Cohesion: 0.06
Nodes (27): NotificationContainer(), NotificationHandle, NotificationItem, NotificationProvider(), NotificationToast(), NotificationVariant, Danger, Info (+19 more)

### Community 5 - "ChoicePicker"
Cohesion: 0.04
Nodes (23): ChoicePicker(), ControlRow(), ToggleControl(), AccordionPage(), BadgesPage(), BottomNavigationPage(), BreadcrumbsPage(), CardsPage() (+15 more)

### Community 6 - "AnimationVariant"
Cohesion: 0.05
Nodes (37): Animations, .DRAWER_LEFT, .DRAWER_RIGHT, .DROPDOWN_ENTER, .FADE_IN, .FADE_OUT, .INDETERMINATE, .MODAL_ENTER (+29 more)

### Community 7 - "ListMarker"
Cohesion: 0.07
Nodes (32): List(), ListContext, ListItem(), ListMarker, Circle, Custom, Decimal, Disc (+24 more)

### Community 8 - "src/rich_text.rs"
Cohesion: 0.08
Nodes (29): format_inline(), html_escape(), html_to_markdown(), markdown_to_html(), RichTextCommand, Blockquote, Bold, BulletList (+21 more)

### Community 9 - "SpacingToken"
Cohesion: 0.25
Nodes (7): SpacingToken, Lg, Md, None, Sm, Xl, Xs

### Community 10 - "table.rs"
Cohesion: 0.09
Nodes (19): DataTable(), DataTableColumn, DataTableColumn<T>, SortDirection, Ascending, Descending, None, Table() (+11 more)

### Community 11 - "complex_tooling_tests.rs"
Cohesion: 0.05
Nodes (18): CodeBlock(), CodeHighlight(), SyntaxToken, SyntaxTokenKind, Comment, Function, Keyword, Number (+10 more)

### Community 12 - "GridSpacing"
Cohesion: 0.08
Nodes (22): Grid(), GridAlign, Baseline, Center, End, Start, Stretch, GridCol() (+14 more)

### Community 13 - "SvgIcon"
Cohesion: 0.10
Nodes (10): AttachmentCard(), BottomNavItem(), BottomNavItemProps, render_icon_paths(), RenderSvgIcon(), SvgIcon, Custom, Kind (+2 more)

### Community 14 - "menubar/mod.rs"
Cohesion: 0.14
Nodes (16): MenubarItem(), MenubarItemProps, MenubarContent(), MenubarContentProps, MenubarMenu(), MenubarMenuProps, MenubarTrigger(), MenubarTriggerProps (+8 more)

### Community 15 - "stepper/mod.rs"
Cohesion: 0.12
Nodes (16): StepperCompleted(), StepperCompletedProps, Stepper(), StepperCtx, StepperOrientation, .ALL, Horizontal, Vertical (+8 more)

### Community 16 - "app_shell/mod.rs"
Cohesion: 0.14
Nodes (13): AppShellAside(), AppShellAsideProps, AppShellFooter(), AppShellFooterProps, AppShellHeader(), AppShellHeaderProps, AppShellMain(), AppShellMainProps (+5 more)

### Community 17 - "page.rs"
Cohesion: 0.24
Nodes (7): IconsPage(), USAGE_CODE, LivePlayground(), StoryHead(), StoryPage(), StoryView(), SectionHead()

### Community 18 - "src/color_picker.rs"
Cohesion: 0.18
Nodes (21): ColorPicker(), default_color_swatches(), format_color(), hsl_to_rgba(), Hsla, hsv_to_rgb(), parse_color(), parse_hex() (+13 more)

### Community 19 - "src/overflow_set.rs"
Cohesion: 0.11
Nodes (11): Dropdown(), DropdownDivider(), DropdownItem(), DropdownPlacement, BottomLeft, BottomRight, TopLeft, TopRight (+3 more)

### Community 20 - "accordion/mod.rs"
Cohesion: 0.12
Nodes (12): AccordionItem(), Accordion(), AccordionContext, AccordionItemContext, AccordionVariant, Contained, Default, Separated (+4 more)

### Community 21 - "BadgeVariant"
Cohesion: 0.12
Nodes (12): Badge(), BadgeVariant, Danger, Default, Outline, Primary, Success, Warning (+4 more)

### Community 22 - "FeatureBanner"
Cohesion: 0.05
Nodes (22): FeatureBanner(), ActivityFeedPage(), CoachmarkPage(), CommandPalettePage(), ContextMenuPage(), CopyButtonPage(), DrawerPage(), DropdownPage() (+14 more)

### Community 23 - "PropRow"
Cohesion: 0.05
Nodes (17): PropRow, PropsTable(), ShowcaseTabs(), AlertsPage(), AnimationsPage(), AppShellPage(), AspectRatioPage(), CarouselPage() (+9 more)

### Community 24 - "ControlSize"
Cohesion: 0.19
Nodes (11): calculate_pagination_range(), PaginationControl(), PaginationControlProps, PaginationItem, Ellipsis, Page, test_pagination_range_at_start(), test_pagination_range_small_total() (+3 more)

### Community 25 - "src/slider.rs"
Cohesion: 0.14
Nodes (12): calc_percentage(), RangeSlider(), Slider(), SliderMark, snap_to_step(), test_slider_marks(), Tooltip(), TooltipPosition (+4 more)

### Community 26 - "ButtonVariant"
Cohesion: 0.05
Nodes (33): AlertDialog(), ConfirmDialog(), Button(), ButtonSize, Lg, Md, Sm, Xl (+25 more)

### Community 27 - "IndicatorColor"
Cohesion: 0.11
Nodes (16): Indicator(), IndicatorColor, Amber, Cyan, Emerald, Rose, Violet, IndicatorPosition (+8 more)

### Community 28 - "ComponentSpacing"
Cohesion: 0.11
Nodes (11): ComponentSpacing, Lg, Md, None, Sm, Xl, Xs, ImageList() (+3 more)

### Community 29 - "Theme"
Cohesion: 0.12
Nodes (3): test_css_variables_share_one_source(), test_presets_emit_identical_classes(), Theme

### Community 30 - "timeline.rs"
Cohesion: 0.14
Nodes (15): Timeline(), TimelineAlign, Left, Right, TimelineBulletState, Active, Completed, Default (+7 more)

### Community 31 - "HeadingOrder"
Cohesion: 0.13
Nodes (17): Code(), Heading(), HeadingOrder, H1, H2, H3, H4, H5 (+9 more)

### Community 32 - "AGENTS.md — Shadow Design Workspace Guide"
Cohesion: 0.12
Nodes (16): 0. Navigazione del Codice con graphify, 1. Panoramica del Progetto, 2. Struttura del Workspace Cargo, 3. Feature Flags, 4. Sistema di Stile (100% SCSS), 5. Theme Engine (`crate/src/theme.rs`, ADR-0003), 6. Showcase (`showcase/`), 8. Comandi (+8 more)

### Community 33 - "primitives.rs"
Cohesion: 0.24
Nodes (12): CurveType, Linear, Monotone, Step, DEFAULT_CHART_COLORS, generate_stacked_area_band_path(), generate_svg_area_path(), generate_svg_path() (+4 more)

### Community 34 - "theme.rs"
Cohesion: 0.12
Nodes (14): AccentTokens, DEFAULT_ACCENT, DEFAULT_SURFACES, mix(), mix_channel(), Custom, relative_luminance(), safe_hex() (+6 more)

### Community 35 - "PopoverPosition"
Cohesion: 0.13
Nodes (17): Popover(), PopoverContext, PopoverDropdown(), PopoverPosition, Bottom, BottomEnd, BottomStart, Left (+9 more)

### Community 37 - "src/select.rs"
Cohesion: 0.24
Nodes (10): Combobox(), display_label(), filter_items(), MultiSelect(), Select(), SelectItem, SelectOption(), test_display_label() (+2 more)

### Community 38 - "Changelog — Shadow Design"
Cohesion: 0.06
Nodes (34): [0.1.0] — 2026-10-09, [0.2.0] — 2026-10-11, [0.3.0] — 2026-10-14, [0.4.0] — 2026-10-20, [0.5.0] — 2026-10-24, 📦 Architettura del Crate & Metadati, ⚠️ Breaking changes, ⚠️ Breaking — fondamenta Lit Glass (ADR-0002, ADR-0003, ADR-0004) (+26 more)

### Community 39 - "AlertVariant"
Cohesion: 0.19
Nodes (6): Alert(), AlertVariant, Danger, Info, Success, Warning

### Community 40 - "breadcrumbs/mod.rs"
Cohesion: 0.16
Nodes (10): BreadcrumbItem(), BreadcrumbItemProps, BreadcrumbData, Breadcrumbs(), BreadcrumbsProps, calculate_collapse_bounds(), test_breadcrumb_data_builder(), test_collapse_bounds_custom_overrides() (+2 more)

### Community 41 - "CardShadow"
Cohesion: 0.08
Nodes (15): AspectRatio(), Card(), CardShadow, Lg, Md, None, Sm, Xl (+7 more)

### Community 42 - "ChartSeries"
Cohesion: 0.14
Nodes (6): AreaChart(), LineChart(), ChartDataPoint, ChartSeries, compute_min_max(), test_point_builder()

### Community 43 - "src/chip.rs"
Cohesion: 0.15
Nodes (14): ControlSize, Lg, Md, Sm, Chip(), ChipGroup(), ChipGroupContext, ChipGroupProps (+6 more)

### Community 44 - "Changelog — Shadow Design"
Cohesion: 0.07
Nodes (29): [0.1.0] — 2026-10-09, [0.2.0] — 2026-10-11, [0.3.0] — 2026-10-14, [0.4.0] — 2026-10-20, [0.5.0] — 2026-10-24, 📦 Architettura del Crate & Metadati, Changelog — Shadow Design, 🐛 Correzioni & Miglioramenti (+21 more)

### Community 45 - "AGENTS.md — Shadow Design Workspace Guide"
Cohesion: 0.12
Nodes (16): 0. Navigazione del Codice con graphify, 1. Panoramica del Progetto, 2. Struttura del Workspace Cargo, 3. Feature Flags, 4. Sistema di Stile (100% SCSS), 5. Theme Engine (`crate/src/theme.rs`, ADR-0003), 6. Showcase (`showcase/`), 8. Comandi (+8 more)

### Community 47 - "PrimaryColor"
Cohesion: 0.16
Nodes (7): PrimaryColor, Crimson, Cyan, Emerald, Obsidian, Violet, sanitize_css_prefix()

### Community 48 - "CloseButton"
Cohesion: 0.15
Nodes (9): CloseButton(), CloseButtonSize, Lg, Md, Sm, CloseButtonVariant, Filled, Subtle (+1 more)

### Community 49 - "src/radio.rs"
Cohesion: 0.22
Nodes (7): Radio(), RadioCard(), RadioContext, RadioGroup(), RadioOrientation, Horizontal, Vertical

### Community 50 - "DeliveryStatus"
Cohesion: 0.15
Nodes (9): BubbleDirection, Incoming, Outgoing, DeliveryStatus, Delivered, Failed, Sending, Sent (+1 more)

### Community 51 - "UseForm<T>"
Cohesion: 0.15
Nodes (6): FormOptions, FormValidator, use_form(), UseForm, UseForm<T>, FormValidationPage()

### Community 52 - "NavigationLinkItem"
Cohesion: 0.21
Nodes (5): NavigationLinkItem, NavigationMenu(), NavigationMenuItem, NavigationMenuSection, NavigationPreviewCard

### Community 53 - "ProgressVariant"
Cohesion: 0.16
Nodes (10): ProgressBar(), ProgressSize, Lg, Md, Sm, ProgressVariant, Danger, Primary (+2 more)

### Community 54 - "ComponentOverride"
Cohesion: 0.17
Nodes (5): ComponentOverride, test_builder_preserves_overrides_when_changing_color(), test_custom_color_any_prefix_uses_generic_classes(), test_explicit_glow_always_yields_class(), test_radius_scale_is_provider_class_only()

### Community 55 - "hooks/mod.rs"
Cohesion: 0.11
Nodes (10): use_clipboard(), UseClipboard, use_debounced_signal(), use_throttle(), use_idle(), use_intersection_observer(), use_local_storage(), use_media_query() (+2 more)

### Community 56 - "area.rs"
Cohesion: 0.31
Nodes (4): ChartContainer(), ChartGrid(), ChartLegend(), ChartTooltip()

### Community 57 - "src/pin_input.rs"
Cohesion: 0.13
Nodes (10): use_unique_id(), ColorInput(), PinInput(), PinInputProps, PinInputSize, PinInputType, Alphanumeric, Numeric (+2 more)

### Community 58 - "get_series_color"
Cohesion: 0.20
Nodes (7): get_series_color(), BubbleChart(), ScaleType, Linear, Log, ScatterChart(), ScatterPoint

### Community 61 - "bottom_navigation/mod.rs"
Cohesion: 0.21
Nodes (7): BottomNavigation(), BottomNavigationProps, BottomNavPosition, Absolute, .ALL, Fixed, Relative

### Community 62 - "HoverCardPosition"
Cohesion: 0.22
Nodes (9): HoverCard(), HoverCardContext, HoverCardDropdown(), HoverCardPosition, Bottom, Left, Right, Top (+1 more)

### Community 63 - "JustifyContent"
Cohesion: 0.15
Nodes (8): Group(), JustifyContent, Center, End, SpaceAround, SpaceBetween, SpaceEvenly, Start

### Community 64 - "src/speed_dial.rs"
Cohesion: 0.22
Nodes (8): Fab(), SpeedDial(), SpeedDialAction(), SpeedDialDirection, Down, Left, Right, Up

### Community 65 - "pie.rs"
Cohesion: 0.23
Nodes (5): GaugeChart(), RadialBar(), DonutChart(), PieChart(), PieSegment

### Community 66 - "TreeNode"
Cohesion: 0.23
Nodes (3): TreeNode, TreeNodeRow(), TreeView()

### Community 67 - "PageHeader"
Cohesion: 0.06
Nodes (16): Counter(), PageHeader(), AlertDialogPage(), BurgerPage(), CheckboxPage(), ChipPage(), CloseButtonPage(), DatePickerPage() (+8 more)

### Community 68 - "src/drawer.rs"
Cohesion: 0.23
Nodes (9): Drawer(), DrawerPosition, Left, Right, DrawerSize, Lg, Md, Sm (+1 more)

### Community 69 - "modal.rs"
Cohesion: 0.22
Nodes (7): Modal(), ModalSize, Lg, Md, Sm, Xl, Xxl

### Community 70 - "ScrollType"
Cohesion: 0.23
Nodes (7): ScrollArea(), ScrollAreaProps, ScrollType, Always, Auto, Hover, Never

### Community 71 - "4. Snippets"
Cohesion: 0.06
Nodes (34): 1.1 Hero concept: "Un hex, tutto il tema", 1.2 Presets, radius and glow demonstrated live, 1.3 Composed scenes versus isolated widgets, 1.4 Density, rhythm, type, 1.5 Motion moments, 1.6 Mobile (390), 1. Visual direction, 2.1 Navigation (+26 more)

### Community 73 - "AffixPosition"
Cohesion: 0.29
Nodes (7): Affix(), AffixPosition, BottomLeft, BottomRight, TopLeft, TopRight, BackToTop()

### Community 74 - "AlignItems"
Cohesion: 0.17
Nodes (7): AlignItems, Baseline, Center, End, Start, Stretch, Stack()

### Community 75 - "Burger"
Cohesion: 0.24
Nodes (5): Burger(), BurgerSize, Lg, Md, Sm

### Community 76 - "CheckboxSize"
Cohesion: 0.25
Nodes (3): Checkbox(), CheckboxSize, CheckboxSizeExt

### Community 77 - "src/context_menu.rs"
Cohesion: 0.33
Nodes (7): ContextMenu(), ContextMenuCheckboxItem(), ContextMenuContent(), ContextMenuContext, ContextMenuItem(), ContextMenuSeparator(), ContextMenuTrigger()

### Community 79 - "src/number_input.rs"
Cohesion: 0.31
Nodes (6): format_and_clamp_number(), format_number_string(), NumberInput(), test_format_and_clamp_over_max(), test_format_and_clamp_under_min(), test_format_and_clamp_within_bounds()

### Community 80 - "RadiusScale"
Cohesion: 0.11
Nodes (12): RadiusScale, Medium, Sharp, Soft, GlowTile(), RadiusTile(), ScopedTheme(), SurfaceBox() (+4 more)

### Community 81 - "lint_design.py"
Cohesion: 0.17
Nodes (15): compile_scss(), extract_selectors(), main(), sass_command(), scan_source_classes(), compiled_classes(), lint_missing_classes(), lint_rust() (+7 more)

### Community 82 - "📦 Libreria UI (`crate/`)"
Cohesion: 0.20
Nodes (9): Avviare l'Espositore, 🖼️ Espositore (`showcase/`), Feature disponibili, Includere la libreria nei tuoi siti web, 🎨 Integrazione Asset SCSS (`STYLES`), 📦 Libreria UI (`crate/`), Shadow Design, 🎭 Sistema Temi Mantine-Style & ThemeBuilder (+1 more)

### Community 83 - "BarChart"
Cohesion: 0.22
Nodes (8): BarChart(), BarChartType, Default, Percent, Stacked, ChartOrientation, Horizontal, Vertical

### Community 84 - "GlowMode"
Cohesion: 0.25
Nodes (4): GlowMode, Expressive, Off, Subtle

### Community 85 - "heatmap.rs"
Cohesion: 0.36
Nodes (3): CalendarHeatmap(), Heatmap(), HeatmapCell

### Community 86 - "FieldsetVariant"
Cohesion: 0.32
Nodes (5): Fieldset(), FieldsetVariant, Default, Filled, Ungrouped

### Community 87 - "use_theme"
Cohesion: 0.33
Nodes (6): ResizableDirection, Horizontal, Vertical, ResizableHandle(), ResizablePanel(), ResizablePanelGroup()

### Community 88 - "SkeletonShape"
Cohesion: 0.32
Nodes (5): Skeleton(), SkeletonShape, Circle, Rectangle, Text

### Community 89 - "ElevationStyle"
Cohesion: 0.29
Nodes (4): ElevationStyle, Flat, NeonGlow, test_glow_strength_and_flat()

### Community 90 - "src/transfer_list.rs"
Cohesion: 0.25
Nodes (3): TransferColumn(), TransferItem, TransferList()

### Community 91 - "theme_menu.rs"
Cohesion: 0.15
Nodes (12): VERSION, SearchPalette(), THEME_CATEGORY, glow_from_key(), glow_key(), PRESETS, radius_from_key(), radius_key() (+4 more)

### Community 92 - "components/controls.rs"
Cohesion: 0.09
Nodes (14): CheckControl(), Controls(), DemoCell(), OptionPicker(), Playground(), SEGMENT_MAX, Stage(), AttachmentCardsPage() (+6 more)

### Community 93 - "Shadow Design (`shadow-design`)"
Cohesion: 0.29
Nodes (6): 💻 Esempio d'Uso, 🧩 Feature Flags Disponibili, 📦 Installazione in Cargo.toml, 🎨 Integrazione Diretta Asset SCSS in Dioxus 0.7, Shadow Design (`shadow-design`), 🎨 Sistema del Tema Mantine (`theme`)

### Community 94 - "PersonaProps"
Cohesion: 0.13
Nodes (6): ActivityComment, ActivityEvent, ActivityFeed(), ActivityItem(), PersonaProps, DocumentCard()

### Community 95 - "Orientation"
Cohesion: 0.38
Nodes (4): Divider(), Orientation, Horizontal, Vertical

### Community 99 - "3. Recipes"
Cohesion: 0.14
Nodes (14): 3.10 Semantic surface (alert, toast, badge, status pill, callout): `@include sd-semantic(<sem>)`, 3.11 Value fill (progress, slider range, switch on, gauge, rating), 3.12 Text and labels, 3.13 Data visualisation, 3.1 Container: `@include sd-surface(1)`, 3.2 Floating panel: `@include sd-surface(2)` or `sd-surface(3)`, 3.3 Backdrop, 3.4 Item (menu item, list option, tab, segmented item, pagination page, tree node, nav link): `@include sd-item` (+6 more)

### Community 100 - "registry.rs"
Cohesion: 0.13
Nodes (9): ALL, all(), by_category(), CRATE_MANIFEST, find(), matches(), neighbours(), STORIES (+1 more)

### Community 101 - "src/tabs.rs"
Cohesion: 0.43
Nodes (5): Tab(), TabList(), TabListCtx, TabPanel(), Tabs()

### Community 105 - "stories/select.rs"
Cohesion: 0.13
Nodes (19): PROPS, Combo(), COMBO_CODE, COMBO_PROPS, frameworks(), Grouped(), GROUPED_CODE, IN_CONTEXT_CODE (+11 more)

### Community 108 - "src/highlight.rs"
Cohesion: 0.60
Nodes (3): Chunk, Highlight(), split_highlight()

### Community 109 - "Playground"
Cohesion: 0.19
Nodes (14): AffixPage(), CenterPage(), bool_template_on_by_default_is_printed(), changed_knob_is_printed_and_highlighted(), children_line_is_highlighted_when_its_knob_changes(), defaults_print_only_children(), generate(), Generated (+6 more)

### Community 113 - "reactive_hooks_tests.rs"
Cohesion: 0.24
Nodes (6): use_window_scroll(), UseWindowScroll, test_debounced_signal_initial_value(), test_form_validation_and_errors(), test_window_scroll_initial_value(), TestFormModel

### Community 114 - "adr/README.md"
Cohesion: 0.17
Nodes (6): Architecture Decision Records, Rules, Template, Shadow Design: design documentation, Note specifiche per Showcase, Showcase — Istruzioni Agenti

### Community 115 - "Shadow Design — Showcase App"
Cohesion: 0.50
Nodes (3): 🎨 Architettura Stili, Shadow Design — Showcase App, 🚀 Sviluppo Locale (Integrazione Diretta Dioxus 0.7)

### Community 116 - "Signal"
Cohesion: 0.24
Nodes (12): DropdownCtx, Signal, SpeedDialContext, BoolKnob(), ChoiceKnob(), ColorKnob(), Knob(), KnobPanel() (+4 more)

### Community 117 - "pages/typography.rs"
Cohesion: 0.29
Nodes (6): HEADINGS_CODE, INLINE_CODE, Role, ROLES, TEXT_CODE, TypographyPage()

### Community 118 - "TrendDirection"
Cohesion: 0.40
Nodes (5): TrendDirection, Auto, Negative, Neutral, Positive

### Community 119 - "home.rs"
Cohesion: 0.26
Nodes (8): SidebarLink(), SidebarNav(), ThemeCard(), preset_name(), Home(), PRESETS, PresetTile(), Stat()

### Community 121 - "sections.rs"
Cohesion: 0.20
Nodes (14): Example, PropGroup, Recipe, Span, Full, Half, DemoCell(), ExampleCard() (+6 more)

### Community 122 - "Showcase revamp: batch migration guide (phase 2)"
Cohesion: 0.12
Nodes (15): 1. Where things are, 2. The stub contract, 3. Steps for one story, 4.1 Controls and code generation, 4.2 Gotchas the compiler or the tests will catch, 4.3 Shared vocabulary you may use, 4.4 Trivial components, 4. Engine cheat sheet (+7 more)

### Community 123 - "pages/stepper.rs"
Cohesion: 0.12
Nodes (6): ControlGroup(), AvatarsPage(), ChatPage(), DividersPage(), HighlightPage(), StepperPage()

### Community 124 - "Knobs"
Cohesion: 0.18
Nodes (7): Knobs, Value, Bool, Choice, Color, Number, Text

### Community 125 - "components/layout.rs"
Cohesion: 0.22
Nodes (9): use_hotkey(), ColorSwatchButton(), glow_key(), Layout(), radius_key(), SIDEBAR_SCROLL_JS, SidebarLink(), SidebarNav() (+1 more)

### Community 126 - "pages/scroll_area.rs"
Cohesion: 0.15
Nodes (4): GridPage(), GroupPage(), ScrollAreaPage(), StackPage()

### Community 127 - "buttons.rs"
Cohesion: 0.19
Nodes (10): IN_CONTEXT_CODE, render(), Sizes(), SIZES_CODE, States(), STATES_CODE, VARIANT_OPTS, Variants() (+2 more)

### Community 128 - "Category"
Cohesion: 0.15
Nodes (9): Category, .ALL, Contenuti, Dati, Fondamenta, Form, Navigazione, Overlay (+1 more)

### Community 129 - "nav.rs"
Cohesion: 0.28
Nodes (6): CATEGORIES, ComponentCategory, ComponentItem, ComponentNavEntry, COMPONENTS, SearchPalette()

### Community 130 - "schema.rs"
Cohesion: 0.25
Nodes (8): SIZE_OPTS, VARIANT_OPTS, ChoiceOption, opt(), Status, Beta, Stable, Stub

### Community 131 - "SegmentedControl"
Cohesion: 0.29
Nodes (5): SegmentedControl(), SegmentedControlSize, Lg, Md, Sm

### Community 132 - "app/layout.rs"
Cohesion: 0.25
Nodes (3): Header(), Layout(), SCROLL_ACTIVE_JS

### Community 133 - "Kind"
Cohesion: 0.25
Nodes (6): Kind, Bool, Choice, Color, Number, Text

### Community 134 - "ColorFormat"
Cohesion: 0.33
Nodes (6): ColorFormat, Hex, Hsl, Rgb, ColorInputProps, ColorPickerProps

### Community 135 - "RatioPreset"
Cohesion: 0.33
Nodes (5): RatioPreset, Cinematic21_9, Square1_1, Standard4_3, Widescreen16_9

### Community 136 - "Emit"
Cohesion: 0.33
Nodes (5): Emit, Always, IfChanged, Skip, Template

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

### Community 164 - "use_theme"
Cohesion: 0.14
Nodes (14): Carousel(), CarouselSlide(), use_theme(), VirtualList(), copy_text(), CurrentThemeTile(), elevation_key(), OverrideField() (+6 more)

### Community 177 - "Lit Glass: the Shadow Design language"
Cohesion: 0.40
Nodes (5): 1. Principles, 2. Depth ladder, 4. Glow budget, 5. Motion, Lit Glass: the Shadow Design language

### Community 178 - "Token reference"
Cohesion: 0.17
Nodes (12): Accent (theme-computed from one hex), Breakpoints (SCSS only, because media queries can't read variables), Charts, Code (component-requested), Motion, Radius (scale class set by `ThemeProvider`), Semantic (fixed, never follow the accent), Shadows & glow (+4 more)

### Community 186 - "prelude"
Cohesion: 0.09
Nodes (6): Dropzone(), FileMetadata, Icon(), Keytip(), LoadingOverlay(), Navbar()

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

### Community 195 - "ADR-0003: Runtime CSS tokens are the only theming mechanism"
Cohesion: 0.50
Nodes (4): ADR-0003: Runtime CSS tokens are the only theming mechanism, Consequences, Context, Decision

## Knowledge Gaps
- **415 isolated node(s):** `shadow-design`, `Sm`, `Md`, `Lg`, `Default` (+410 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1324 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **94 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `IconKind` connect `IconKind` to `Route`, `avatar.rs`, `notification.rs`, `src/rich_text.rs`, `complex_tooling_tests.rs`, `SvgIcon`, `stepper/mod.rs`, `src/overflow_set.rs`, `BadgeVariant`, `ControlSize`, `ButtonVariant`, `ComponentSpacing`, `src/select.rs`, `AlertVariant`, `breadcrumbs/mod.rs`, `src/chip.rs`, `src/radio.rs`, `NavigationLinkItem`, `src/speed_dial.rs`, `TreeNode`, `AffixPosition`, `src/context_menu.rs`, `src/number_input.rs`, `use_theme`, `PersonaProps`, `src/rating.rs`, `registry.rs`, `src/tabs.rs`, `Signal`, `home.rs`, `components/layout.rs`, `Category`, `nav.rs`, `schema.rs`, `stories/accordion.rs`, `stories/affix.rs`, `stories/alert_dialog.rs`, `stories/animations.rs`, `stories/app_shell.rs`, `stories/aspect_ratio.rs`, `stories/attachment_cards.rs`, `stories/burger.rs`, `stories/cards.rs`, `stories/carousel.rs`, `stories/center.rs`, `stories/charts.rs`, `stories/chat.rs`, `stories/checkbox.rs`, `stories/chip.rs`, `stories/close_button.rs`, `stories/coachmark.rs`, `use_theme`, `stories/color_picker.rs`, `stories/command_palette.rs`, `stories/document_card.rs`, `stories/drawer.rs`, `stories/dropdown.rs`, `stories/dropzone.rs`, `stories/fieldset.rs`, `stories/form.rs`, `stories/grid.rs`, `stories/group.rs`, `stories/highlight.rs`, `stories/indicator.rs`, `stories/inputs.rs`, `stories/keytip.rs`, `stories/lists.rs`, `$schema`, `prelude`, `stories/loading_overlay.rs`, `stories/marquee_selection.rs`, `stories/masonry.rs`, `stories/media_card.rs`, `stories/menubar.rs`, `stories/modals.rs`, `stories/navbar.rs`, `stories/navigation_menu.rs`, `stories/number_input.rs`, `stories/overflow_set.rs`, `stories/pagination.rs`, `stories/pin_input.rs`, `stories/popover.rs`, `stories/progress.rs`, `stories/reactive_hooks.rs`, `stories/resizable.rs`, `stories/rich_text.rs`, `stories/scroll_area.rs`, `stories/segmented_controls.rs`, `stories/slider.rs`, `stories/spoiler.rs`, `stories/stepper.rs`, `stories/switch.rs`, `stories/tables.rs`, `stories/tabs.rs`, `stories/textarea.rs`, `stories/tooltip.rs`, `stories/transfer_list.rs`, `stories/trees.rs`, `stories/virtual_list.rs`?**
  _High betweenness centrality (0.160) - this node is a cross-community bridge._
- **What connects `shadow-design`, `Sm`, `Md` to the rest of the system?**
  _415 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.02857142857142857 - nodes in this community are weakly interconnected._
- **Why does `Signal` connect `Signal` to `nav.rs`, `IconKind`, `SegmentedControl`, `notification.rs`, `ChoicePicker`, `ColorFormat`, `app/layout.rs`, `src/rich_text.rs`, `table.rs`, `complex_tooling_tests.rs`, `SvgIcon`, `menubar/mod.rs`, `Switch`, `app_shell/mod.rs`, `src/overflow_set.rs`, `accordion/mod.rs`, `BadgeVariant`, `ControlSize`, `src/slider.rs`, `ButtonVariant`, `theme.rs`, `PopoverPosition`, `use_theme`, `src/select.rs`, `breadcrumbs/mod.rs`, `src/chip.rs`, `super`, `src/radio.rs`, `UseForm<T>`, `hooks/mod.rs`, `area.rs`, `src/pin_input.rs`, `HoverCardPosition`, `src/speed_dial.rs`, `TreeNode`, `src/drawer.rs`, `modal.rs`, `Burger`, `CheckboxSize`, `src/context_menu.rs`, `Locale`, `src/number_input.rs`, `src/transfer_list.rs`, `theme_menu.rs`, `components/controls.rs`, `src/rating.rs`, `src/textarea.rs`, `src/tabs.rs`, `reactive_hooks_tests.rs`, `spoiler/mod.rs`?**
  _High betweenness centrality (0.065) - this node is a cross-community bridge._
- **Should `IconKind` be split into smaller, more focused modules?**
  _Cohesion score 0.034482758620689655 - nodes in this community are weakly interconnected._
- **Should `avatar.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08923076923076922 - nodes in this community are weakly interconnected._
- **Should `notification.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06184012066365008 - nodes in this community are weakly interconnected._