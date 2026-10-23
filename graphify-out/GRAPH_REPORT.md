# Graph Report - shadow-design  (2026-10-23)

## Corpus Check
- 252 files · ~167,959 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 106 file(s) not represented in the graph (top: .scss 94, (none) 7, .toml 2)

## Summary
- 2198 nodes · 3888 edges · 190 communities (97 shown, 93 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 77 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `fc2a2cde`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- src/main.rs
- IconKind
- avatar.rs
- notification.rs
- ButtonVariant
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
- prelude
- src/color_picker.rs
- src/overflow_set.rs
- accordion/mod.rs
- BadgeVariant
- FeatureBanner
- ShowcaseTabs
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
- use_theme
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
- PrimaryColor
- super
- ComponentOverride
- CloseButton
- Signal
- DeliveryStatus
- UseForm<T>
- NavigationLinkItem
- ProgressVariant
- PrimaryColor
- hooks/mod.rs
- area.rs
- src/pin_input.rs
- get_series_color
- ThemeBuilder
- MarqueeSelection
- bottom_navigation/mod.rs
- HoverCardPosition
- SegmentedControl
- src/speed_dial.rs
- pie.rs
- TreeNode
- util.rs
- src/drawer.rs
- modal.rs
- ScrollType
- src/loading_overlay.rs
- AffixPosition
- src/media_card.rs
- Burger
- CheckboxSize
- src/context_menu.rs
- use_locale
- src/number_input.rs
- RadiusScale
- check_classes.py
- 📦 Libreria UI (`crate/`)
- BarChart
- GlowMode
- heatmap.rs
- FieldsetVariant
- CalendarViewMode
- SkeletonShape
- ElevationStyle
- src/transfer_list.rs
- layout.rs
- alerts.rs
- Shadow Design (`shadow-design`)
- ColorFormat
- Orientation
- src/rating.rs
- avatars.rs
- Signal
- buttons.rs
- src/tabs.rs
- RatioPreset
- RadarChart
- ContactCard
- src/highlight.rs
- pages/checkbox.rs
- src/center.rs
- UseWindowScroll
- Switch
- Shadow Design — Showcase App
- tables.rs
- pages/coachmark.rs
- AlignItems
- accordion.rs
- pages/activity_feed.rs
- pages/affix.rs
- pages/alert_dialog.rs
- app_shell.rs
- pages/aspect_ratio.rs
- attachment_cards.rs
- pages/command_palette.rs
- breadcrumbs.rs
- pages/burger.rs
- pages/carousel.rs
- pages/center.rs
- charts.rs
- pages/chat.rs
- pages/code_highlight.rs
- pages/color_picker.rs
- pages/context_menu.rs
- pages/fieldset.rs
- pages/date_picker.rs
- dividers.rs
- pages/document_card.rs
- pages/drawer.rs
- pages/dropdown.rs
- pages/pin_input.rs
- skeletons.rs
- pages/slider.rs
- pages/group.rs
- pages/highlight.rs
- pages/hover_card.rs
- pages/indicator.rs
- inputs.rs
- pages/keytip.rs
- lists.rs
- pages/marquee_selection.rs
- pages/masonry.rs
- pages/media_card.rs
- modals.rs
- pages/stack.rs
- pages/navbar.rs
- pages/navigation_menu.rs
- pages/number_input.rs
- timelines.rs
- pagination.rs
- src/textarea.rs
- pages/progress.rs
- CommandAction
- pages/rating.rs
- pages/resizable.rs
- pages/rich_text.rs
- scroll_area.rs
- segmented_controls.rs
- pages/speed_dial.rs
- spoiler.rs
- src/coachmark.rs
- pages/switch.rs
- pages/tabs.rs
- pages/textarea.rs
- pages/tooltip.rs
- nav_link/mod.rs
- JustifyContent
- pages/typography.rs
- AttachmentCard
- STYLES
- GEMINI.md
- shadow-design
- renovate.json
- BreadcrumbItemProps
- App
- AlertDialog
- TextInput

## God Nodes (most connected - your core abstractions)
1. `IconKind` - 134 edges
2. `Route` - 94 edges
3. `PropRow` - 92 edges
4. `ShowcaseTabs()` - 91 edges
5. `FeatureBanner()` - 89 edges
6. `Signal` - 77 edges
7. `Icon()` - 49 edges
8. `Theme` - 48 edges
9. `SvgIcon` - 32 edges
10. `use_theme()` - 27 edges

## Surprising Connections (you probably didn't know these)
- `ReactiveHooksPage()` --calls--> `use_hotkey()`  [INFERRED]
  showcase/src/pages/reactive_hooks.rs → crate/src/hooks/hotkey.rs
- `ReactiveHooksPage()` --calls--> `use_window_scroll()`  [INFERRED]
  showcase/src/pages/reactive_hooks.rs → crate/src/hooks/scroll.rs
- `FeatureBanner()` --calls--> `use_theme()`  [INFERRED]
  showcase/src/components/banner.rs → crate/src/theme.rs
- `Layout()` --calls--> `use_theme()`  [INFERRED]
  showcase/src/components/layout.rs → crate/src/theme.rs
- `SidebarLink()` --calls--> `use_theme()`  [INFERRED]
  showcase/src/components/layout.rs → crate/src/theme.rs

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

## Communities (190 total, 93 thin omitted)

### Community 0 - "src/main.rs"
Cohesion: 0.06
Nodes (90): Route, AccordionPage, ActivityFeedPage, AffixPage, AlertDialogPage, AlertsPage, AnimationsPage, AppShellPage (+82 more)

### Community 2 - "IconKind"
Cohesion: 0.04
Nodes (55): IconKind, AlertCircle, AlertTriangle, .ALL, Bluetooth, Box, Calendar, Check (+47 more)

### Community 3 - "avatar.rs"
Cohesion: 0.06
Nodes (20): ActivityComment, ActivityEvent, ActivityFeed(), ActivityItem(), Avatar(), AvatarGroup(), AvatarSize, Lg (+12 more)

### Community 4 - "notification.rs"
Cohesion: 0.07
Nodes (22): NotificationContainer(), NotificationHandle, NotificationItem, NotificationProvider(), NotificationToast(), NotificationVariant, Danger, Info (+14 more)

### Community 5 - "ButtonVariant"
Cohesion: 0.09
Nodes (16): Button(), ButtonSize, Lg, Md, Sm, Xl, Xs, ButtonVariant (+8 more)

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
Nodes (17): DataTable(), DataTableColumn, DataTableColumn<T>, SortDirection, Ascending, Descending, None, Table() (+9 more)

### Community 11 - "complex_tooling_tests.rs"
Cohesion: 0.06
Nodes (15): CodeBlock(), CodeHighlight(), SyntaxToken, SyntaxTokenKind, Comment, Function, Keyword, Number (+7 more)

### Community 12 - "GridSpacing"
Cohesion: 0.08
Nodes (22): Grid(), GridAlign, Baseline, Center, End, Start, Stretch, GridCol() (+14 more)

### Community 13 - "SvgIcon"
Cohesion: 0.28
Nodes (5): RenderSvgIcon(), SvgIcon, Custom, Kind, SegmentOption

### Community 14 - "menubar/mod.rs"
Cohesion: 0.14
Nodes (16): MenubarItem(), MenubarItemProps, MenubarContent(), MenubarContentProps, MenubarMenu(), MenubarMenuProps, MenubarTrigger(), MenubarTriggerProps (+8 more)

### Community 15 - "stepper/mod.rs"
Cohesion: 0.12
Nodes (16): StepperCompleted(), StepperCompletedProps, Stepper(), StepperCtx, StepperOrientation, .ALL, Horizontal, Vertical (+8 more)

### Community 16 - "app_shell/mod.rs"
Cohesion: 0.14
Nodes (13): AppShellAside(), AppShellAsideProps, AppShellFooter(), AppShellFooterProps, AppShellHeader(), AppShellHeaderProps, AppShellMain(), AppShellMainProps (+5 more)

### Community 17 - "prelude"
Cohesion: 0.09
Nodes (7): Keytip(), Navbar(), copy_text(), BadgesPage(), MenubarPage(), PopoverPage(), RadioPage()

### Community 18 - "src/color_picker.rs"
Cohesion: 0.18
Nodes (22): ColorPicker(), ColorPickerProps, default_color_swatches(), format_color(), hsl_to_rgba(), Hsla, hsv_to_rgb(), parse_color() (+14 more)

### Community 19 - "src/overflow_set.rs"
Cohesion: 0.11
Nodes (12): Dropdown(), DropdownCtx, DropdownDivider(), DropdownItem(), DropdownPlacement, BottomLeft, BottomRight, TopLeft (+4 more)

### Community 20 - "accordion/mod.rs"
Cohesion: 0.11
Nodes (13): AccordionControl(), AccordionItem(), Accordion(), AccordionContext, AccordionItemContext, AccordionVariant, Contained, Default (+5 more)

### Community 21 - "BadgeVariant"
Cohesion: 0.19
Nodes (8): Badge(), BadgeVariant, Danger, Default, Outline, Primary, Success, Warning

### Community 22 - "FeatureBanner"
Cohesion: 0.08
Nodes (8): FeatureBanner(), BottomNavigationPage(), CardsPage(), ChipPage(), CloseButtonPage(), FacepilePage(), TransferListPage(), TreesPage()

### Community 23 - "ShowcaseTabs"
Cohesion: 0.10
Nodes (7): ShowcaseTabs(), AnimationsPage(), GridPage(), LoadingOverlayPage(), NavLinkPage(), OverflowSetPage(), VirtualListPage()

### Community 24 - "ControlSize"
Cohesion: 0.19
Nodes (11): calculate_pagination_range(), PaginationControl(), PaginationControlProps, PaginationItem, Ellipsis, Page, test_pagination_range_at_start(), test_pagination_range_small_total() (+3 more)

### Community 25 - "src/slider.rs"
Cohesion: 0.14
Nodes (12): calc_percentage(), RangeSlider(), Slider(), SliderMark, snap_to_step(), test_slider_marks(), Tooltip(), TooltipPosition (+4 more)

### Community 26 - "Date"
Cohesion: 0.24
Nodes (4): Calendar(), Date, DatePicker(), DateRangePicker()

### Community 27 - "IndicatorColor"
Cohesion: 0.11
Nodes (16): Indicator(), IndicatorColor, Amber, Cyan, Emerald, Rose, Violet, IndicatorPosition (+8 more)

### Community 28 - "ComponentSpacing"
Cohesion: 0.13
Nodes (11): ComponentSpacing, Lg, Md, None, Sm, Xl, Xs, ImageList() (+3 more)

### Community 30 - "timeline.rs"
Cohesion: 0.14
Nodes (15): Timeline(), TimelineAlign, Left, Right, TimelineBulletState, Active, Completed, Default (+7 more)

### Community 31 - "HeadingOrder"
Cohesion: 0.13
Nodes (17): Code(), Heading(), HeadingOrder, H1, H2, H3, H4, H5 (+9 more)

### Community 32 - "AGENTS.md — Shadow Design Workspace Guide"
Cohesion: 0.10
Nodes (18): 1. Panoramica del Progetto, 2. Struttura del Workspace Cargo, 3. Crate Libreria (`crate` / `shadow-design`), 4. Crate Showcase (`showcase`), 5. Regole e Convenzioni per lo Sviluppo, 6. Comandi Utili per il Workflow, AGENTS.md — Shadow Design Workspace Guide, Architettura 100% SCSS & Supporto Nativo Dioxus `asset!()` nella Crate (+10 more)

### Community 33 - "primitives.rs"
Cohesion: 0.17
Nodes (16): CurveType, Linear, Monotone, Step, DEFAULT_CHART_COLORS, generate_stacked_area_band_path(), generate_svg_area_path(), generate_svg_path() (+8 more)

### Community 34 - "use_theme"
Cohesion: 0.33
Nodes (6): ResizableDirection, Horizontal, Vertical, ResizableHandle(), ResizablePanel(), ResizablePanelGroup()

### Community 35 - "PopoverPosition"
Cohesion: 0.13
Nodes (17): Popover(), PopoverContext, PopoverDropdown(), PopoverPosition, Bottom, BottomEnd, BottomStart, Left (+9 more)

### Community 36 - "PropRow"
Cohesion: 0.12
Nodes (7): PropRow, PropsTable(), CopyButtonPage(), DropzonePage(), UserRegistrationModel, SelectPage(), StepperPage()

### Community 37 - "src/select.rs"
Cohesion: 0.24
Nodes (10): Combobox(), display_label(), filter_items(), MultiSelect(), Select(), SelectItem, SelectOption(), test_display_label() (+2 more)

### Community 38 - "Changelog — Shadow Design"
Cohesion: 0.11
Nodes (18): [0.1.0] — 2026-10-09, [0.2.0] — 2026-10-11, [0.3.0] — 2026-10-14, [0.4.0] — 2026-10-20, 📦 Architettura del Crate & Metadati, Changelog — Shadow Design, 🐛 Correzioni & Miglioramenti, 🐛 Correzioni & Ottimizzazioni (+10 more)

### Community 39 - "AlertVariant"
Cohesion: 0.18
Nodes (6): Alert(), AlertVariant, Danger, Info, Success, Warning

### Community 40 - "breadcrumbs/mod.rs"
Cohesion: 0.23
Nodes (8): BreadcrumbData, Breadcrumbs(), BreadcrumbsProps, calculate_collapse_bounds(), test_breadcrumb_data_builder(), test_collapse_bounds_custom_overrides(), test_collapse_bounds_not_exceeding_max(), test_collapse_bounds_with_max_3()

### Community 41 - "CardShadow"
Cohesion: 0.12
Nodes (12): Card(), CardShadow, Lg, Md, None, Sm, Xl, CardVariant (+4 more)

### Community 42 - "ChartSeries"
Cohesion: 0.14
Nodes (6): AreaChart(), LineChart(), ChartDataPoint, ChartSeries, compute_min_max(), test_point_builder()

### Community 43 - "src/chip.rs"
Cohesion: 0.15
Nodes (14): ControlSize, Lg, Md, Sm, Chip(), ChipGroup(), ChipGroupContext, ChipGroupProps (+6 more)

### Community 44 - "Changelog — Shadow Design"
Cohesion: 0.11
Nodes (17): [0.1.0] — 2026-10-09, [0.2.0] — 2026-10-11, [0.3.0] — 2026-10-14, [0.4.0] — 2026-10-20, 📦 Architettura del Crate & Metadati, Changelog — Shadow Design, 🐛 Correzioni & Miglioramenti, 🐛 Correzioni & Ottimizzazioni (+9 more)

### Community 45 - "PrimaryColor"
Cohesion: 0.16
Nodes (7): PrimaryColor, Crimson, Cyan, Emerald, Obsidian, Violet, ThemeProvider()

### Community 46 - "super"
Cohesion: 0.33
Nodes (3): AriaHeading(), NEXT_ID, use_body_scroll_lock()

### Community 48 - "CloseButton"
Cohesion: 0.15
Nodes (9): CloseButton(), CloseButtonSize, Lg, Md, Sm, CloseButtonVariant, Filled, Subtle (+1 more)

### Community 49 - "Signal"
Cohesion: 0.22
Nodes (7): Radio(), RadioCard(), RadioContext, RadioGroup(), RadioOrientation, Horizontal, Vertical

### Community 50 - "DeliveryStatus"
Cohesion: 0.16
Nodes (9): BubbleDirection, Incoming, Outgoing, DeliveryStatus, Delivered, Failed, Sending, Sent (+1 more)

### Community 51 - "UseForm<T>"
Cohesion: 0.13
Nodes (7): FormOptions, use_form(), UseForm, UseForm<T>, test_form_validation_and_errors(), TestFormModel, FormValidationPage()

### Community 52 - "NavigationLinkItem"
Cohesion: 0.21
Nodes (5): NavigationLinkItem, NavigationMenu(), NavigationMenuItem, NavigationMenuSection, NavigationPreviewCard

### Community 53 - "ProgressVariant"
Cohesion: 0.16
Nodes (10): ProgressBar(), ProgressSize, Lg, Md, Sm, ProgressVariant, Danger, Primary (+2 more)

### Community 54 - "PrimaryColor"
Cohesion: 0.19
Nodes (4): ColorTokens, sanitize_css_prefix(), test_cartoon_apply_primary_color_keeps_components(), test_primary_color_custom_with_prefix_sanitized()

### Community 55 - "hooks/mod.rs"
Cohesion: 0.09
Nodes (12): use_click_outside(), use_clipboard(), UseClipboard, use_debounced_signal(), use_throttle(), use_idle(), use_intersection_observer(), use_local_storage() (+4 more)

### Community 56 - "area.rs"
Cohesion: 0.33
Nodes (4): ChartContainer(), ChartGrid(), ChartLegend(), ChartTooltip()

### Community 57 - "src/pin_input.rs"
Cohesion: 0.14
Nodes (9): use_unique_id(), ColorInput(), PinInput(), PinInputProps, PinInputType, Alphanumeric, Numeric, sanitize_pin_value() (+1 more)

### Community 58 - "get_series_color"
Cohesion: 0.20
Nodes (7): get_series_color(), BubbleChart(), ScaleType, Linear, Log, ScatterChart(), ScatterPoint

### Community 59 - "ThemeBuilder"
Cohesion: 0.18
Nodes (5): Custom, test_builder_preserves_overrides_when_changing_color(), test_primary_color_custom_classes(), ThemeBuilder, is_valid_hex_color()

### Community 61 - "bottom_navigation/mod.rs"
Cohesion: 0.14
Nodes (9): BottomNavItem(), BottomNavItemProps, BottomNavigation(), BottomNavigationProps, BottomNavPosition, Absolute, .ALL, Fixed (+1 more)

### Community 62 - "HoverCardPosition"
Cohesion: 0.22
Nodes (9): HoverCard(), HoverCardContext, HoverCardDropdown(), HoverCardPosition, Bottom, Left, Right, Top (+1 more)

### Community 63 - "SegmentedControl"
Cohesion: 0.29
Nodes (5): SegmentedControl(), SegmentedControlSize, Lg, Md, Sm

### Community 64 - "src/speed_dial.rs"
Cohesion: 0.22
Nodes (9): Fab(), SpeedDial(), SpeedDialAction(), SpeedDialContext, SpeedDialDirection, Down, Left, Right (+1 more)

### Community 65 - "pie.rs"
Cohesion: 0.23
Nodes (5): GaugeChart(), RadialBar(), DonutChart(), PieChart(), PieSegment

### Community 66 - "TreeNode"
Cohesion: 0.21
Nodes (3): TreeNode, TreeNodeRow(), TreeView()

### Community 68 - "src/drawer.rs"
Cohesion: 0.24
Nodes (8): Drawer(), DrawerPosition, Left, Right, DrawerSize, Lg, Md, Sm

### Community 69 - "modal.rs"
Cohesion: 0.22
Nodes (7): Modal(), ModalSize, Lg, Md, Sm, Xl, Xxl

### Community 70 - "ScrollType"
Cohesion: 0.23
Nodes (7): ScrollArea(), ScrollAreaProps, ScrollType, Always, Auto, Hover, Never

### Community 73 - "AffixPosition"
Cohesion: 0.29
Nodes (7): Affix(), AffixPosition, BottomLeft, BottomRight, TopLeft, TopRight, BackToTop()

### Community 74 - "src/media_card.rs"
Cohesion: 0.27
Nodes (3): AspectRatio(), CardActionArea(), MediaCard()

### Community 75 - "Burger"
Cohesion: 0.24
Nodes (5): Burger(), BurgerSize, Lg, Md, Sm

### Community 76 - "CheckboxSize"
Cohesion: 0.24
Nodes (3): Checkbox(), CheckboxSize, CheckboxSizeExt

### Community 77 - "src/context_menu.rs"
Cohesion: 0.33
Nodes (7): ContextMenu(), ContextMenuCheckboxItem(), ContextMenuContent(), ContextMenuContext, ContextMenuItem(), ContextMenuSeparator(), ContextMenuTrigger()

### Community 78 - "use_locale"
Cohesion: 0.29
Nodes (3): Locale, LocaleProvider(), use_locale()

### Community 79 - "src/number_input.rs"
Cohesion: 0.31
Nodes (6): format_and_clamp_number(), format_number_string(), NumberInput(), test_format_and_clamp_over_max(), test_format_and_clamp_under_min(), test_format_and_clamp_within_bounds()

### Community 80 - "RadiusScale"
Cohesion: 0.22
Nodes (4): RadiusScale, Medium, Sharp, Soft

### Community 81 - "check_classes.py"
Cohesion: 0.38
Nodes (4): compile_scss(), extract_selectors(), main(), scan_source_classes()

### Community 82 - "📦 Libreria UI (`crate/`)"
Cohesion: 0.20
Nodes (9): Avviare l'Espositore, 🖼️ Espositore (`showcase/`), Feature disponibili, Includere la libreria nei tuoi siti web, 🎨 Integrazione Asset SCSS (`STYLES`), 📦 Libreria UI (`crate/`), Shadow Design, 🎭 Sistema Temi Mantine-Style & ThemeBuilder (+1 more)

### Community 83 - "BarChart"
Cohesion: 0.22
Nodes (8): BarChart(), BarChartType, Default, Percent, Stacked, ChartOrientation, Horizontal, Vertical

### Community 84 - "GlowMode"
Cohesion: 0.28
Nodes (6): GlowMode, Expressive, Off, Subtle, GlowTokens, test_glow_modes_different()

### Community 85 - "heatmap.rs"
Cohesion: 0.36
Nodes (3): CalendarHeatmap(), Heatmap(), HeatmapCell

### Community 86 - "FieldsetVariant"
Cohesion: 0.32
Nodes (5): Fieldset(), FieldsetVariant, Default, Filled, Ungrouped

### Community 87 - "CalendarViewMode"
Cohesion: 0.50
Nodes (4): CalendarViewMode, Day, Month, Year

### Community 88 - "SkeletonShape"
Cohesion: 0.32
Nodes (5): Skeleton(), SkeletonShape, Circle, Rectangle, Text

### Community 89 - "ElevationStyle"
Cohesion: 0.29
Nodes (5): ElevationStyle, Flat, HardBlock, NeonGlow, SoftElevated

### Community 91 - "layout.rs"
Cohesion: 0.10
Nodes (17): use_hotkey(), VERSION, CodeBlock, ColorSwatchButton(), Layout(), SidebarLink(), SidebarView(), all_cargo_features() (+9 more)

### Community 93 - "Shadow Design (`shadow-design`)"
Cohesion: 0.29
Nodes (6): 💻 Esempio d'Uso, 🧩 Feature Flags Disponibili, 📦 Installazione in Cargo.toml, 🎨 Integrazione Diretta Asset SCSS in Dioxus 0.7, Shadow Design (`shadow-design`), 🎨 Sistema del Tema Mantine (`theme`)

### Community 94 - "ColorFormat"
Cohesion: 0.33
Nodes (5): ColorFormat, Hex, Hsl, Rgb, ColorInputProps

### Community 95 - "Orientation"
Cohesion: 0.38
Nodes (4): Divider(), Orientation, Horizontal, Vertical

### Community 99 - "Signal"
Cohesion: 0.14
Nodes (8): Carousel(), CarouselSlide(), Dropzone(), FileMetadata, Signal, Spoiler(), use_theme(), VirtualList()

### Community 101 - "src/tabs.rs"
Cohesion: 0.43
Nodes (5): Tab(), TabList(), TabListCtx, TabPanel(), Tabs()

### Community 105 - "RatioPreset"
Cohesion: 0.33
Nodes (5): RatioPreset, Cinematic21_9, Square1_1, Standard4_3, Widescreen16_9

### Community 108 - "src/highlight.rs"
Cohesion: 0.60
Nodes (3): Chunk, Highlight(), split_highlight()

### Community 113 - "UseWindowScroll"
Cohesion: 0.47
Nodes (3): use_window_scroll(), UseWindowScroll, test_window_scroll_initial_value()

### Community 115 - "Shadow Design — Showcase App"
Cohesion: 0.50
Nodes (3): 🎨 Architettura Stili, Shadow Design — Showcase App, 🚀 Sviluppo Locale (Integrazione Diretta Dioxus 0.7)

### Community 118 - "AlignItems"
Cohesion: 0.17
Nodes (7): AlignItems, Baseline, Center, End, Start, Stretch, Stack()

### Community 172 - "src/coachmark.rs"
Cohesion: 0.28
Nodes (7): Coachmark(), CoachmarkPosition, Bottom, Left, Right, Top, TeachingBubble()

### Community 177 - "nav_link/mod.rs"
Cohesion: 0.31
Nodes (4): SidebarMenu(), SidebarMenuProps, NavLink(), NavLinkProps

### Community 178 - "JustifyContent"
Cohesion: 0.25
Nodes (7): JustifyContent, Center, End, SpaceAround, SpaceBetween, SpaceEvenly, Start

## Knowledge Gaps
- **392 isolated node(s):** `shadow-design`, `Sm`, `Md`, `Lg`, `Default` (+387 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 997 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **93 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `IconKind` connect `IconKind` to `avatar.rs`, `notification.rs`, `ButtonVariant`, `src/rich_text.rs`, `complex_tooling_tests.rs`, `SvgIcon`, `menubar/mod.rs`, `stepper/mod.rs`, `src/overflow_set.rs`, `accordion/mod.rs`, `ControlSize`, `Date`, `ComponentSpacing`, `use_theme`, `CommandAction`, `src/select.rs`, `AlertVariant`, `breadcrumbs/mod.rs`, `src/chip.rs`, `CloseButton`, `nav_link/mod.rs`, `DeliveryStatus`, `Signal`, `NavigationLinkItem`, `AlertDialog`, `src/speed_dial.rs`, `TreeNode`, `src/loading_overlay.rs`, `AffixPosition`, `CheckboxSize`, `src/context_menu.rs`, `src/number_input.rs`, `src/transfer_list.rs`, `layout.rs`, `src/rating.rs`, `Signal`, `src/tabs.rs`?**
  _High betweenness centrality (0.079) - this node is a cross-community bridge._
- **What connects `shadow-design`, `Sm`, `Md` to the rest of the system?**
  _392 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `src/main.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06287985039738195 - nodes in this community are weakly interconnected._
- **Why does `Signal` connect `Signal` to `IconKind`, `notification.rs`, `src/rich_text.rs`, `table.rs`, `menubar/mod.rs`, `app_shell/mod.rs`, `src/color_picker.rs`, `src/overflow_set.rs`, `accordion/mod.rs`, `ControlSize`, `src/slider.rs`, `Date`, `src/textarea.rs`, `PopoverPosition`, `CommandAction`, `src/select.rs`, `breadcrumbs/mod.rs`, `src/chip.rs`, `PrimaryColor`, `super`, `nav_link/mod.rs`, `Signal`, `UseForm<T>`, `hooks/mod.rs`, `area.rs`, `src/pin_input.rs`, `AlertDialog`, `TextInput`, `HoverCardPosition`, `SegmentedControl`, `src/speed_dial.rs`, `TreeNode`, `src/drawer.rs`, `modal.rs`, `Burger`, `CheckboxSize`, `src/context_menu.rs`, `use_locale`, `src/number_input.rs`, `src/transfer_list.rs`, `layout.rs`, `ColorFormat`, `src/rating.rs`, `src/tabs.rs`, `UseWindowScroll`, `Switch`?**
  _High betweenness centrality (0.060) - this node is a cross-community bridge._
- **Should `lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.03333333333333333 - nodes in this community are weakly interconnected._
- **Why does `Theme` connect `Theme` to `lib.rs`, `Signal`, `notification.rs`, `ButtonVariant`, `CardShadow`, `PrimaryColor`, `ComponentOverride`, `RadiusScale`, `GlowMode`, `BadgeVariant`, `PrimaryColor`, `ElevationStyle`, `ThemeBuilder`?**
  _High betweenness centrality (0.027) - this node is a cross-community bridge._
- **Should `IconKind` be split into smaller, more focused modules?**
  _Cohesion score 0.03571428571428571 - nodes in this community are weakly interconnected._