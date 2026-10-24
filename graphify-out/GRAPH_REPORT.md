# Graph Report - shadow-design  (2026-10-24)

## Corpus Check
- 267 files · ~167,158 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 107 file(s) not represented in the graph (top: .scss 94, (none) 7, .toml 2)

## Summary
- 2378 nodes · 4469 edges · 147 communities (116 shown, 31 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 74 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `653dfd07`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- src/main.rs
- IconKind
- avatar.rs
- notification.rs
- Button
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
- Playground
- src/color_picker.rs
- src/overflow_set.rs
- accordion/mod.rs
- BadgeVariant
- prelude
- PageHeader
- ControlSize
- src/slider.rs
- Date
- IndicatorColor
- ComponentSpacing
- Theme
- timeline.rs
- HeadingOrder
- AGENTS.md — Shadow Design Workspace Guide
- primitives.rs
- theme.rs
- PopoverPosition
- PropRow
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
- Signal
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
- util.rs
- src/drawer.rs
- modal.rs
- ScrollType
- ButtonVariant
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
- use_theme
- ChoicePicker
- Shadow Design (`shadow-design`)
- nav.rs
- Orientation
- Icon
- src/textarea.rs
- 3. Recipes
- ControlRow
- src/tabs.rs
- RatioPreset
- RadarChart
- ContactCard
- src/highlight.rs
- src/alert_dialog.rs
- src/center.rs
- reactive_hooks_tests.rs
- adr/README.md
- Shadow Design — Showcase App
- nav_link.rs
- ShowcaseTabs
- TrendDirection
- CodeBlock
- spoiler/mod.rs
- VirtualList
- src/keytip.rs
- src/navbar.rs
- FeatureBanner
- controls.rs
- CommandAction
- theme_builder.rs
- src/coachmark.rs
- Lit Glass: the Shadow Design language
- Token reference
- StepState
- STYLES
- shadow-design
- renovate.json
- src/dropzone.rs
- Component checklist
- ADR-0001: Record decisions in ADRs and bind agents to them
- ADR-0004: Component styling contract
- 7. Regole di Sviluppo
- src/carousel.rs
- CalendarViewMode
- ADR-0003: Runtime CSS tokens are the only theming mechanism
- 4. Sistema di Stile (100% SCSS)
- design-system.md

## God Nodes (most connected - your core abstractions)
1. `IconKind` - 134 edges
2. `Route` - 94 edges
3. `PropRow` - 92 edges
4. `PageHeader()` - 91 edges
5. `ShowcaseTabs()` - 91 edges
6. `FeatureBanner()` - 88 edges
7. `Signal` - 83 edges
8. `Playground()` - 58 edges
9. `ChoicePicker()` - 51 edges
10. `Icon()` - 49 edges

## Surprising Connections (you probably didn't know these)
- `Layout()` --calls--> `use_hotkey()`  [INFERRED]
  showcase/src/components/layout.rs → crate/src/hooks/hotkey.rs
- `ReactiveHooksPage()` --calls--> `use_window_scroll()`  [INFERRED]
  showcase/src/pages/reactive_hooks.rs → crate/src/hooks/scroll.rs
- `ThemeBuilderPage()` --calls--> `use_notification()`  [INFERRED]
  showcase/src/pages/theme_builder.rs → crate/src/notification.rs
- `OverrideField()` --calls--> `use_theme()`  [INFERRED]
  showcase/src/pages/theme_builder.rs → crate/src/theme.rs
- `SurfaceToken()` --calls--> `use_theme()`  [INFERRED]
  showcase/src/pages/theme_builder.rs → crate/src/theme.rs

## Import Cycles
- 1-file cycle: `crate/src/lib.rs -> crate/src/lib.rs`
- 1-file cycle: `showcase/src/main.rs -> showcase/src/main.rs`
- 2-file cycle: `showcase/src/components/layout.rs -> showcase/src/main.rs -> showcase/src/components/layout.rs`
- 2-file cycle: `crate/src/stepper/mod.rs -> crate/src/stepper/step.rs -> crate/src/stepper/mod.rs`
- 2-file cycle: `crate/src/stepper/completed.rs -> crate/src/stepper/mod.rs -> crate/src/stepper/completed.rs`
- 2-file cycle: `crate/src/menubar/item.rs -> crate/src/menubar/mod.rs -> crate/src/menubar/item.rs`
- 2-file cycle: `crate/src/menubar/menu.rs -> crate/src/menubar/mod.rs -> crate/src/menubar/menu.rs`
- 2-file cycle: `crate/src/app_shell/mod.rs -> crate/src/app_shell/navbar.rs -> crate/src/app_shell/mod.rs`
- 2-file cycle: `crate/src/app_shell/aside.rs -> crate/src/app_shell/mod.rs -> crate/src/app_shell/aside.rs`
- 2-file cycle: `crate/src/accordion/mod.rs -> crate/src/accordion/panel.rs -> crate/src/accordion/mod.rs`
- 3-file cycle: `showcase/src/components/layout.rs -> showcase/src/components/nav.rs -> showcase/src/main.rs -> showcase/src/components/layout.rs`

## Communities (147 total, 31 thin omitted)

### Community 0 - "src/main.rs"
Cohesion: 0.06
Nodes (95): App(), FAVICON, HEADER_SVG, Route, AccordionPage, ActivityFeedPage, AffixPage, AlertDialogPage (+87 more)

### Community 2 - "IconKind"
Cohesion: 0.03
Nodes (56): AccordionControl(), IconKind, AlertCircle, AlertTriangle, .ALL, Bluetooth, Box, Calendar (+48 more)

### Community 3 - "avatar.rs"
Cohesion: 0.06
Nodes (20): ActivityComment, ActivityEvent, ActivityFeed(), ActivityItem(), Avatar(), AvatarGroup(), AvatarSize, Lg (+12 more)

### Community 4 - "notification.rs"
Cohesion: 0.10
Nodes (14): NotificationContainer(), NotificationHandle, NotificationItem, NotificationProvider(), NotificationToast(), NotificationVariant, Danger, Info (+6 more)

### Community 5 - "Button"
Cohesion: 0.19
Nodes (7): Button(), ButtonSize, Lg, Md, Sm, Xl, Xs

### Community 6 - "AnimationVariant"
Cohesion: 0.05
Nodes (37): Animations, .DRAWER_LEFT, .DRAWER_RIGHT, .DROPDOWN_ENTER, .FADE_IN, .FADE_OUT, .INDETERMINATE, .MODAL_ENTER (+29 more)

### Community 7 - "ListMarker"
Cohesion: 0.07
Nodes (32): List(), ListContext, ListItem(), ListMarker, Circle, Custom, Decimal, Disc (+24 more)

### Community 8 - "src/rich_text.rs"
Cohesion: 0.08
Nodes (28): format_inline(), html_escape(), html_to_markdown(), markdown_to_html(), RichTextCommand, Blockquote, Bold, BulletList (+20 more)

### Community 9 - "SpacingToken"
Cohesion: 0.15
Nodes (8): Group(), SpacingToken, Lg, Md, None, Sm, Xl, Xs

### Community 10 - "table.rs"
Cohesion: 0.09
Nodes (19): DataTable(), DataTableColumn, DataTableColumn<T>, SortDirection, Ascending, Descending, None, Table() (+11 more)

### Community 11 - "complex_tooling_tests.rs"
Cohesion: 0.06
Nodes (15): CodeBlock(), CodeHighlight(), SyntaxToken, SyntaxTokenKind, Comment, Function, Keyword, Number (+7 more)

### Community 12 - "GridSpacing"
Cohesion: 0.08
Nodes (22): Grid(), GridAlign, Baseline, Center, End, Start, Stretch, GridCol() (+14 more)

### Community 13 - "SvgIcon"
Cohesion: 0.10
Nodes (12): AttachmentCard(), RenderSvgIcon(), SvgIcon, Custom, Kind, TextInput(), SegmentedControl(), SegmentedControlSize (+4 more)

### Community 14 - "menubar/mod.rs"
Cohesion: 0.14
Nodes (16): MenubarItem(), MenubarItemProps, MenubarContent(), MenubarContentProps, MenubarMenu(), MenubarMenuProps, MenubarTrigger(), MenubarTriggerProps (+8 more)

### Community 15 - "stepper/mod.rs"
Cohesion: 0.18
Nodes (9): StepperCompleted(), StepperCompletedProps, Stepper(), StepperCtx, StepperOrientation, .ALL, Horizontal, Vertical (+1 more)

### Community 16 - "app_shell/mod.rs"
Cohesion: 0.14
Nodes (13): AppShellAside(), AppShellAsideProps, AppShellFooter(), AppShellFooterProps, AppShellHeader(), AppShellHeaderProps, AppShellMain(), AppShellMainProps (+5 more)

### Community 17 - "Playground"
Cohesion: 0.05
Nodes (21): ControlGroup(), Playground(), ToggleControl(), AccordionPage(), AnimationsPage(), AttachmentCardsPage(), AvatarsPage(), BadgesPage() (+13 more)

### Community 18 - "src/color_picker.rs"
Cohesion: 0.16
Nodes (23): use_unique_id(), ColorInput(), ColorPicker(), default_color_swatches(), format_color(), hsl_to_rgba(), Hsla, hsv_to_rgb() (+15 more)

### Community 19 - "src/overflow_set.rs"
Cohesion: 0.11
Nodes (12): Dropdown(), DropdownCtx, DropdownDivider(), DropdownItem(), DropdownPlacement, BottomLeft, BottomRight, TopLeft (+4 more)

### Community 20 - "accordion/mod.rs"
Cohesion: 0.12
Nodes (12): AccordionItem(), Accordion(), AccordionContext, AccordionItemContext, AccordionVariant, Contained, Default, Separated (+4 more)

### Community 21 - "BadgeVariant"
Cohesion: 0.10
Nodes (14): Badge(), BadgeVariant, Danger, Default, Outline, Primary, Success, Warning (+6 more)

### Community 22 - "prelude"
Cohesion: 0.11
Nodes (5): AppShellPage(), BottomNavigationPage(), MasonryPage(), RichTextPage(), TransferListPage()

### Community 23 - "PageHeader"
Cohesion: 0.06
Nodes (15): Counter(), PageHeader(), BurgerPage(), CloseButtonPage(), UserRegistrationModel, InputsPage(), MarqueeSelectionPage(), MenubarPage() (+7 more)

### Community 24 - "ControlSize"
Cohesion: 0.15
Nodes (15): ControlSize, Lg, Md, Sm, calculate_pagination_range(), PaginationControl(), PaginationControlProps, PaginationItem (+7 more)

### Community 25 - "src/slider.rs"
Cohesion: 0.14
Nodes (12): calc_percentage(), RangeSlider(), Slider(), SliderMark, snap_to_step(), test_slider_marks(), Tooltip(), TooltipPosition (+4 more)

### Community 26 - "Date"
Cohesion: 0.26
Nodes (4): Calendar(), Date, DatePicker(), DateRangePicker()

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
Nodes (17): 0. Navigazione del Codice con graphify, 1. Panoramica del Progetto, 2. Struttura del Workspace Cargo, 3. Feature Flags, 5. Theme Engine (`crate/src/theme.rs`, ADR-0003), 6. Showcase (`showcase/`), 7. Regole di Sviluppo, 8. Comandi (+9 more)

### Community 33 - "primitives.rs"
Cohesion: 0.24
Nodes (12): CurveType, Linear, Monotone, Step, DEFAULT_CHART_COLORS, generate_stacked_area_band_path(), generate_svg_area_path(), generate_svg_path() (+4 more)

### Community 34 - "theme.rs"
Cohesion: 0.13
Nodes (12): AccentTokens, DEFAULT_ACCENT, DEFAULT_SURFACES, mix(), mix_channel(), Custom, relative_luminance(), safe_hex() (+4 more)

### Community 35 - "PopoverPosition"
Cohesion: 0.13
Nodes (17): Popover(), PopoverContext, PopoverDropdown(), PopoverPosition, Bottom, BottomEnd, BottomStart, Left (+9 more)

### Community 36 - "PropRow"
Cohesion: 0.23
Nodes (4): PropRow, PropsTable(), MediaCardPage(), SelectPage()

### Community 37 - "src/select.rs"
Cohesion: 0.24
Nodes (10): Combobox(), display_label(), filter_items(), MultiSelect(), Select(), SelectItem, SelectOption(), test_display_label() (+2 more)

### Community 38 - "Changelog — Shadow Design"
Cohesion: 0.06
Nodes (32): [0.1.0] — 2026-10-09, [0.2.0] — 2026-10-11, [0.3.0] — 2026-10-14, [0.4.0] — 2026-10-20, [0.5.0] — 2026-10-24, 📦 Architettura del Crate & Metadati, ⚠️ Breaking changes, ⚠️ Breaking — fondamenta Lit Glass (ADR-0002, ADR-0003, ADR-0004) (+24 more)

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
Cohesion: 0.18
Nodes (11): Chip(), ChipGroup(), ChipGroupContext, ChipGroupProps, ChipProps, ChipVariant, Filled, Light (+3 more)

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
Cohesion: 0.14
Nodes (10): AriaHeading(), CloseButton(), CloseButtonSize, Lg, Md, Sm, CloseButtonVariant, Filled (+2 more)

### Community 49 - "Signal"
Cohesion: 0.10
Nodes (16): ColorFormat, Hex, Hsl, Rgb, ColorInputProps, ColorPickerProps, use_intersection_observer(), Signal (+8 more)

### Community 50 - "DeliveryStatus"
Cohesion: 0.16
Nodes (9): BubbleDirection, Incoming, Outgoing, DeliveryStatus, Delivered, Failed, Sending, Sent (+1 more)

### Community 51 - "UseForm<T>"
Cohesion: 0.16
Nodes (6): FormOptions, FormValidator, use_form(), UseForm, UseForm<T>, FormValidationPage()

### Community 52 - "NavigationLinkItem"
Cohesion: 0.21
Nodes (5): NavigationLinkItem, NavigationMenu(), NavigationMenuItem, NavigationMenuSection, NavigationPreviewCard

### Community 53 - "ProgressVariant"
Cohesion: 0.16
Nodes (10): ProgressBar(), ProgressSize, Lg, Md, Sm, ProgressVariant, Danger, Primary (+2 more)

### Community 54 - "ComponentOverride"
Cohesion: 0.20
Nodes (3): ComponentOverride, test_builder_preserves_overrides_when_changing_color(), test_explicit_glow_always_yields_class()

### Community 55 - "hooks/mod.rs"
Cohesion: 0.11
Nodes (10): use_click_outside(), use_clipboard(), UseClipboard, use_debounced_signal(), use_throttle(), use_hotkey(), use_idle(), use_local_storage() (+2 more)

### Community 56 - "area.rs"
Cohesion: 0.31
Nodes (4): ChartContainer(), ChartGrid(), ChartLegend(), ChartTooltip()

### Community 57 - "src/pin_input.rs"
Cohesion: 0.16
Nodes (7): PinInput(), PinInputProps, PinInputSize, PinInputType, Alphanumeric, Numeric, sanitize_pin_value()

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
Cohesion: 0.17
Nodes (8): JustifyContent, Center, End, SpaceAround, SpaceBetween, SpaceEvenly, Start, Stack()

### Community 64 - "src/speed_dial.rs"
Cohesion: 0.22
Nodes (9): Fab(), SpeedDial(), SpeedDialAction(), SpeedDialContext, SpeedDialDirection, Down, Left, Right (+1 more)

### Community 65 - "pie.rs"
Cohesion: 0.23
Nodes (5): GaugeChart(), RadialBar(), DonutChart(), PieChart(), PieSegment

### Community 66 - "TreeNode"
Cohesion: 0.23
Nodes (3): TreeNode, TreeNodeRow(), TreeView()

### Community 68 - "src/drawer.rs"
Cohesion: 0.23
Nodes (9): Drawer(), DrawerPosition, Left, Right, DrawerSize, Lg, Md, Sm (+1 more)

### Community 69 - "modal.rs"
Cohesion: 0.20
Nodes (7): Modal(), ModalSize, Lg, Md, Sm, Xl, Xxl

### Community 70 - "ScrollType"
Cohesion: 0.23
Nodes (7): ScrollArea(), ScrollAreaProps, ScrollType, Always, Auto, Hover, Never

### Community 71 - "ButtonVariant"
Cohesion: 0.20
Nodes (7): ButtonVariant, Danger, Ghost, Outline, Primary, Secondary, Success

### Community 73 - "AffixPosition"
Cohesion: 0.29
Nodes (7): Affix(), AffixPosition, BottomLeft, BottomRight, TopLeft, TopRight, BackToTop()

### Community 74 - "AlignItems"
Cohesion: 0.25
Nodes (6): AlignItems, Baseline, Center, End, Start, Stretch

### Community 75 - "Burger"
Cohesion: 0.24
Nodes (5): Burger(), BurgerSize, Lg, Md, Sm

### Community 76 - "CheckboxSize"
Cohesion: 0.27
Nodes (3): Checkbox(), CheckboxSize, CheckboxSizeExt

### Community 77 - "src/context_menu.rs"
Cohesion: 0.33
Nodes (7): ContextMenu(), ContextMenuCheckboxItem(), ContextMenuContent(), ContextMenuContext, ContextMenuItem(), ContextMenuSeparator(), ContextMenuTrigger()

### Community 79 - "src/number_input.rs"
Cohesion: 0.31
Nodes (6): format_and_clamp_number(), format_number_string(), NumberInput(), test_format_and_clamp_over_max(), test_format_and_clamp_under_min(), test_format_and_clamp_within_bounds()

### Community 80 - "RadiusScale"
Cohesion: 0.15
Nodes (6): RadiusScale, Medium, Sharp, Soft, test_custom_color_any_prefix_uses_generic_classes(), test_radius_scale_is_provider_class_only()

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
Cohesion: 0.27
Nodes (3): TransferColumn(), TransferItem, TransferList()

### Community 91 - "use_theme"
Cohesion: 0.23
Nodes (10): VERSION, use_theme(), ColorSwatchButton(), glow_key(), Layout(), radius_key(), SIDEBAR_SCROLL_JS, SidebarLink() (+2 more)

### Community 92 - "ChoicePicker"
Cohesion: 0.08
Nodes (9): ChoicePicker(), AffixPage(), AlertsPage(), AspectRatioPage(), BreadcrumbsPage(), ChipPage(), ResizablePage(), SpoilerPage() (+1 more)

### Community 93 - "Shadow Design (`shadow-design`)"
Cohesion: 0.29
Nodes (6): 💻 Esempio d'Uso, 🧩 Feature Flags Disponibili, 📦 Installazione in Cargo.toml, 🎨 Integrazione Diretta Asset SCSS in Dioxus 0.7, Shadow Design (`shadow-design`), 🎨 Sistema del Tema Mantine (`theme`)

### Community 94 - "nav.rs"
Cohesion: 0.32
Nodes (6): CATEGORIES, ComponentCategory, ComponentItem, ComponentNavEntry, COMPONENTS, SearchPalette()

### Community 95 - "Orientation"
Cohesion: 0.38
Nodes (4): Divider(), Orientation, Horizontal, Vertical

### Community 96 - "Icon"
Cohesion: 0.14
Nodes (5): Icon(), render_icon_paths(), LoadingOverlay(), clamp_rating_value(), Rating()

### Community 99 - "3. Recipes"
Cohesion: 0.14
Nodes (14): 3.10 Semantic surface (alert, toast, badge, status pill, callout): `@include sd-semantic(<sem>)`, 3.11 Value fill (progress, slider range, switch on, gauge, rating), 3.12 Text and labels, 3.13 Data visualisation, 3.1 Container: `@include sd-surface(1)`, 3.2 Floating panel: `@include sd-surface(2)` or `sd-surface(3)`, 3.3 Backdrop, 3.4 Item (menu item, list option, tab, segmented item, pagination page, tree node, nav link): `@include sd-item` (+6 more)

### Community 100 - "ControlRow"
Cohesion: 0.08
Nodes (9): ControlRow(), ButtonsPage(), ColorPickerPage(), ListsPage(), NumberInputPage(), PinInputPage(), RadioPage(), RatingPage() (+1 more)

### Community 101 - "src/tabs.rs"
Cohesion: 0.43
Nodes (5): Tab(), TabList(), TabListCtx, TabPanel(), Tabs()

### Community 105 - "RatioPreset"
Cohesion: 0.33
Nodes (5): RatioPreset, Cinematic21_9, Square1_1, Standard4_3, Widescreen16_9

### Community 108 - "src/highlight.rs"
Cohesion: 0.60
Nodes (3): Chunk, Highlight(), split_highlight()

### Community 113 - "reactive_hooks_tests.rs"
Cohesion: 0.20
Nodes (7): use_window_scroll(), UseWindowScroll, test_clipboard_initial_state(), test_debounced_signal_initial_value(), test_form_validation_and_errors(), test_window_scroll_initial_value(), TestFormModel

### Community 114 - "adr/README.md"
Cohesion: 0.19
Nodes (6): Architecture Decision Records, Rules, Template, Shadow Design: design documentation, Note specifiche per Showcase, Showcase — Istruzioni Agenti

### Community 115 - "Shadow Design — Showcase App"
Cohesion: 0.50
Nodes (3): 🎨 Architettura Stili, Shadow Design — Showcase App, 🚀 Sviluppo Locale (Integrazione Diretta Dioxus 0.7)

### Community 116 - "nav_link.rs"
Cohesion: 0.14
Nodes (6): CheckControl(), CardsPage(), NavLinkPage(), PaginationPage(), PeerRecord, TablesPage()

### Community 117 - "ShowcaseTabs"
Cohesion: 0.05
Nodes (17): Stage(), ShowcaseTabs(), ActivityFeedPage(), AlertDialogPage(), CoachmarkPage(), CommandPalettePage(), ContextMenuPage(), CopyButtonPage() (+9 more)

### Community 118 - "TrendDirection"
Cohesion: 0.40
Nodes (5): TrendDirection, Auto, Negative, Neutral, Positive

### Community 119 - "CodeBlock"
Cohesion: 0.50
Nodes (3): CodeBlock, all_cargo_features(), Home()

### Community 129 - "FeatureBanner"
Cohesion: 0.11
Nodes (6): FeatureBanner(), CarouselPage(), CenterPage(), DocumentCardPage(), DropzonePage(), NavbarDemoPage()

### Community 131 - "controls.rs"
Cohesion: 0.09
Nodes (10): Controls(), DemoCell(), OptionPicker(), SEGMENT_MAX, ChartsPage(), CodeHighlightPage(), GridPage(), GroupPage() (+2 more)

### Community 164 - "theme_builder.rs"
Cohesion: 0.20
Nodes (7): copy_text(), OverrideField(), save_theme_to_storage(), STORAGE_KEY, SurfaceToken(), ThemeBuilderPage(), use_theme_field()

### Community 172 - "src/coachmark.rs"
Cohesion: 0.29
Nodes (7): Coachmark(), CoachmarkPosition, Bottom, Left, Right, Top, TeachingBubble()

### Community 177 - "Lit Glass: the Shadow Design language"
Cohesion: 0.18
Nodes (9): ADR-0002: One design language: Lit Glass, Consequences, Context, Decision, 1. Principles, 2. Depth ladder, 4. Glow budget, 5. Motion (+1 more)

### Community 178 - "Token reference"
Cohesion: 0.17
Nodes (12): Accent (theme-computed from one hex), Breakpoints (SCSS only, because media queries can't read variables), Charts, Code (component-requested), Motion, Radius (scale class set by `ThemeProvider`), Semantic (fixed, never follow the accent), Shadows & glow (+4 more)

### Community 180 - "StepState"
Cohesion: 0.29
Nodes (7): StepperStep(), StepperStepProps, StepState, Active, Completed, Error, Upcoming

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

### Community 194 - "CalendarViewMode"
Cohesion: 0.50
Nodes (4): CalendarViewMode, Day, Month, Year

### Community 195 - "ADR-0003: Runtime CSS tokens are the only theming mechanism"
Cohesion: 0.50
Nodes (4): ADR-0003: Runtime CSS tokens are the only theming mechanism, Consequences, Context, Decision

### Community 196 - "4. Sistema di Stile (100% SCSS)"
Cohesion: 0.50
Nodes (4): 4. Sistema di Stile (100% SCSS), File principali, Gerarchia Z-Index (`_zindex.scss`), Regole (dettagli in ADR-0004)

## Knowledge Gaps
- **386 isolated node(s):** `shadow-design`, `Sm`, `Md`, `Lg`, `Default` (+381 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1098 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **31 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `IconKind` connect `IconKind` to `avatar.rs`, `notification.rs`, `Button`, `src/rich_text.rs`, `complex_tooling_tests.rs`, `SvgIcon`, `src/overflow_set.rs`, `BadgeVariant`, `ControlSize`, `Date`, `ComponentSpacing`, `CommandAction`, `src/select.rs`, `AlertVariant`, `breadcrumbs/mod.rs`, `src/chip.rs`, `src/coachmark.rs`, `CloseButton`, `Signal`, `DeliveryStatus`, `NavigationLinkItem`, `StepState`, `src/dropzone.rs`, `src/carousel.rs`, `src/speed_dial.rs`, `TreeNode`, `AffixPosition`, `CheckboxSize`, `src/context_menu.rs`, `src/number_input.rs`, `use_theme`, `src/transfer_list.rs`, `use_theme`, `nav.rs`, `Icon`, `src/tabs.rs`, `src/alert_dialog.rs`, `spoiler/mod.rs`?**
  _High betweenness centrality (0.067) - this node is a cross-community bridge._
- **What connects `shadow-design`, `Sm`, `Md` to the rest of the system?**
  _386 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `src/main.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.056363636363636366 - nodes in this community are weakly interconnected._
- **Why does `Signal` connect `Signal` to `IconKind`, `controls.rs`, `notification.rs`, `src/rich_text.rs`, `table.rs`, `SvgIcon`, `menubar/mod.rs`, `app_shell/mod.rs`, `Playground`, `src/overflow_set.rs`, `accordion/mod.rs`, `BadgeVariant`, `ControlSize`, `src/slider.rs`, `Date`, `CommandAction`, `PopoverPosition`, `theme.rs`, `src/select.rs`, `theme_builder.rs`, `breadcrumbs/mod.rs`, `src/chip.rs`, `super`, `UseForm<T>`, `hooks/mod.rs`, `area.rs`, `src/pin_input.rs`, `HoverCardPosition`, `src/carousel.rs`, `src/speed_dial.rs`, `TreeNode`, `src/drawer.rs`, `modal.rs`, `Burger`, `CheckboxSize`, `src/context_menu.rs`, `Locale`, `src/number_input.rs`, `src/transfer_list.rs`, `use_theme`, `ChoicePicker`, `nav.rs`, `Icon`, `src/textarea.rs`, `src/tabs.rs`, `src/alert_dialog.rs`, `reactive_hooks_tests.rs`, `nav_link.rs`, `spoiler/mod.rs`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Should `lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.03333333333333333 - nodes in this community are weakly interconnected._
- **Why does `Locale` connect `Locale` to `lib.rs`, `DeliveryStatus`, `src/drawer.rs`?**
  _High betweenness centrality (0.030) - this node is a cross-community bridge._
- **Should `IconKind` be split into smaller, more focused modules?**
  _Cohesion score 0.034482758620689655 - nodes in this community are weakly interconnected._