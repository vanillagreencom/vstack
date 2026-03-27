# Professional Trading UI Design

**Version 2.0.0**
vanillagreen

> **Note:**
> This document is mainly for agents and LLMs to follow when implementing,
> reviewing, or refactoring trading UI components. Humans may also
> find it useful, but guidance here is optimized for automation and
> consistency by AI-assisted workflows. Stack-agnostic — principles apply
> regardless of framework, language, or rendering engine. Does not define
> specific tokens, colors, or pixel values; those belong in your design system.

---

## Abstract

Stack-agnostic design principles for professional trading applications. Monospace-forward typography (mono for headings/labels/buttons/data, sans for prose only), zero-radius sharp corners on all elements, left-aligned grid layouts with 1px hairline separators, density-first thinking, two-hue directional color, opacity-driven depth. Covers design philosophy, visual language (color theory, opacity, elevation), typography and density, modular panel architecture, data display conventions, interaction design, component philosophy, and accessibility. Prioritized from critical (philosophy, visual language, typography) through high (layout, data, interaction) to medium (components, accessibility).

> **Important:** This compiled document may lag behind the individual rule files in `rules/`. When in doubt, the individual rule files are the source of truth.

---

## Table of Contents

1. [Design Philosophy](#1-design-philosophy) — **CRITICAL**
   - 1.1 [Design Identity and Anti-Patterns](#11-design-identity-and-anti-patterns)
   - 1.2 [Density First](#12-density-first)
   - 1.3 [Signal Through Noise](#13-signal-through-noise)
2. [Visual Language](#2-visual-language) — **CRITICAL**
   - 2.1 [Two-Hue Directional Color](#21-two-hue-directional-color)
   - 2.2 [Opacity as the Primary Visual Variable](#22-opacity-as-the-primary-visual-variable)
   - 2.3 [Dark-Primary Foundation](#23-dark-primary-foundation)
   - 2.4 [Surface Elevation Model](#24-surface-elevation-model)
3. [Typography & Density](#3-typography--density) — **CRITICAL**
   - 3.1 [Dual-Font System](#31-dual-font-system)
   - 3.2 [Data Alignment and Size Hierarchy](#32-data-alignment-and-size-hierarchy)
   - 3.3 [Semantic Token Architecture](#33-semantic-token-architecture)
4. [Layout & Panels](#4-layout--panels) — **HIGH**
   - 4.1 [Panel Architecture and Docking](#41-panel-architecture-and-docking)
   - 4.2 [Required Panel States](#42-required-panel-states)
   - 4.3 [Responsive Collapse Strategy](#43-responsive-collapse-strategy)
5. [Data Display](#5-data-display) — **HIGH**
   - 5.1 [Trading Data Display Conventions](#51-trading-data-display-conventions)
6. [Interaction Design](#6-interaction-design) — **HIGH**
   - 6.1 [Keyboard-First Interaction](#61-keyboard-first-interaction)
   - 6.2 [Error Prevention and Confirmation](#62-error-prevention-and-confirmation)
7. [Component Philosophy](#7-component-philosophy) — **MEDIUM**
   - 7.1 [Component Design Approach](#71-component-design-approach)
8. [Accessibility](#8-accessibility) — **MEDIUM**
   - 8.1 [Accessibility and Cross-Platform](#81-accessibility-and-cross-platform)

---

## 1. Design Philosophy

**Impact: CRITICAL**

The foundational "why" behind professional trading UI — density-first thinking, signal-through-noise principles, and the identity that separates professional tools from retail platforms.

### 1.1 Design Identity and Anti-Patterns

**Impact: CRITICAL (without a clear identity anchor, UI drifts toward generic dashboards or retail trading aesthetics)**

#### The Target Aesthetic

The intersection of three qualities:

- **Sierra Chart / Bloomberg Terminal density** — every pixel carries data, multi-panel layouts, no wasted space, professional traders can monitor dozens of data points simultaneously
- **Vercel / Linear dark refinement** — near-black canvas, restrained palette, typographic precision, considered spacing even at small scales
- **ShadCN component clarity** — composable, consistent components with clear visual hierarchy, but compressed to trading density

This is software for professionals who stare at it 12+ hours a day. It must be dense without being cluttered, dark without being lifeless, and information-rich without being noisy.

The default theme is dark. But the system must support user-customizable themes built on established community palettes — not invented color schemes. Traders personalize their workspace; the design system enables this without compromising density or directional clarity.

#### Reference Platforms (Study These)

| Platform | What to Learn |
|----------|--------------|
| Sierra Chart | Extreme data density, configurability, tiling efficiency |
| Bloomberg Terminal | Information architecture at scale, keyboard-driven workflows, functional color use |
| Trading Technologies (TT) | Order management UX, ladder precision, professional interaction patterns |
| CQG | Clean professional layout, efficient use of screen real estate |
| Vercel Dashboard | Dark aesthetic, typographic hierarchy, restrained color |
| Linear | Dark, dense, keyboard-first app that feels fast and focused |
| ShadCN/ui | Component composition model, consistent design tokens, accessible defaults |

#### Anti-Patterns (Avoid These)

| Anti-Pattern | Why It Fails |
|-------------|-------------|
| **Robinhood aesthetic** | Hides complexity behind whitespace, gamifies trading, wastes screen density |
| **TradingView chrome** | Good charting but too much social/community chrome dilutes focus; panels compete for attention |
| **Crypto exchange neon** | Multiple bright hues, glowing borders, visual noise everywhere — directional color becomes meaningless |
| **Generic dashboard look** | Cards with rounded corners, large padding, gradient backgrounds — wastes 40%+ of screen on decoration |
| **Electron bloat feel** | Sluggish rendering, visible repaints, input lag — professional tools must feel instant |

#### The Density Test

If you can remove a pixel of padding, a border, a shadow, or a color and the interface remains clear — remove it. If adding information to a panel requires scrolling rather than fitting in view — the panel is not dense enough. Professionals configure their workspace once and expect to see everything simultaneously.

### 1.2 Density First

**Impact: CRITICAL (wasted screen real estate in a data-intensive application)**

Density is the primary design constraint. Trading demands maximum data per pixel. Every element must earn its screen space.

#### Principles

- **Default compact, scale up only for readability** — start at the smallest comfortable size for each element. If a label, gap, or padding can be smaller without harming legibility, make it smaller.
- **Pixel accountability** — decorative elements (gradients, large shadows, rounded corners, excessive padding) are costs. They must justify themselves against the data they displace.
- **Simultaneous visibility** — traders configure their workspace to see everything at once. A panel that requires scrolling to show its core content has failed the density test.
- **Compact does not mean cramped** — density requires deliberate spacing at small scales. A 4px base unit with consistent multiples creates rhythm even at tight spacing. The goal is scannable density, not a wall of text.

#### Density Benchmarks

| Element | Target | Rationale |
|---------|--------|-----------|
| Table row height | 20-28px | Enough for single-line data with small padding |
| Panel padding | 4-8px | Minimal chrome between content and border |
| Inter-element gap | 2-4px | Tight but distinguishable |
| Font size (data) | 11-13px | Readable monospace at typical viewing distance |
| Font size (labels) | 10-12px | Secondary to data, clearly subordinate |
| Icon size | 12-16px | Inline with text, not dominant |

These are guidelines, not absolutes. The right density depends on the data and the trader's viewing distance, but they establish the baseline expectation: this is not a consumer app.

#### Information Hierarchy Through Density

Not all data is equal. Use density itself as a hierarchy tool:

- **Primary data** (price, P&L) — slightly larger, full opacity, prominent position
- **Secondary data** (labels, quantities, timestamps) — standard density, reduced opacity
- **Tertiary data** (metadata, IDs) — smallest size, lowest opacity, available but not competing

The hierarchy should be readable from arm's length: the most important numbers jump out even when everything is dense.

### 1.3 Signal Through Noise

**Impact: CRITICAL (traders miss critical information when decorative elements compete with data)**

In a professional trading interface the signal is price action, position state, and order status. Everything else is noise. The entire visual system exists to maximize the signal-to-noise ratio.

#### The Noise Budget

Every visual element draws from a finite attention budget. Borders, shadows, background variations, animations, icons, and color all cost attention. Spend this budget exclusively on information-carrying elements.

**High-value attention spend:**
- Directional color on P&L and price changes (instant understanding)
- Stale data indicators (prevents acting on old information)
- Error and disconnect states (prevents catastrophic trades)
- Direction icons alongside color (redundant encoding = faster processing)

**Low-value attention spend (minimize or eliminate):**
- Decorative borders between panels (a 1px line at low opacity is enough)
- Box shadows for "depth" (near-black elevation handles this)
- Animated transitions between states (instant state changes are faster to process)
- Color-coded categories that aren't directional (adds hues without adding meaning)

#### Earned Pixels

Every pixel of color, every point of padding, every border must answer: "What information does this convey?" If the answer is "it looks nice" or "it's conventional" — remove it. Professional traders don't want nice-looking software. They want software that makes them faster.

#### State Communication

The interface must immediately communicate these states without the user searching:

| State | Must Be Obvious Because |
|-------|------------------------|
| Connected / disconnected | Trading on stale data causes losses |
| Position direction and P&L | Core awareness at all times |
| Order status | Knowing if orders are working, filled, or rejected |
| Data freshness | Stale prices look identical to live prices without explicit indication |
| Error conditions | Hidden errors lead to missed trades or worse |

#### Animation Philosophy

Animation in trading UI is almost always noise. Use it only when it communicates information that would otherwise be missed:

- **Acceptable:** Brief flash on a price update to indicate the tick direction. Skeleton shimmer to indicate loading (vs. empty).
- **Not acceptable:** Panel slide-in transitions, fade-in effects for data, animated charts of non-real-time data, loading spinners when a skeleton would work.

Transitions should be instant or near-instant (under 100ms). If a user can perceive the animation as an animation rather than a state change, it's too slow.

---

## 2. Visual Language

**Impact: CRITICAL**

Color theory, opacity system, near-black foundation, and surface elevation — the visual principles that create hierarchy without introducing noise.

### 2.1 Two-Hue Directional Color

**Impact: CRITICAL (visual noise and ambiguous directional cues when multiple hues compete)**

The base palette uses exactly two chromatic hues: one for positive direction (buy/bid/profit/long) and one for negative direction (sell/ask/loss/short). Conventionally green and red, but the principle is the constraint — not the specific hues.

All other visual variation comes from a single neutral (typically white) at graduated opacities. No blue, orange, yellow, purple, or other chromatic colors in the base palette.

#### Why Two Hues

Trading interfaces have one overriding semantic axis: **direction**. Up/down, buy/sell, profit/loss, bid/ask. Two hues map perfectly to this axis. Every additional hue dilutes the directional signal:

- **Three hues** — the third hue has no clear directional meaning; users must learn an arbitrary mapping
- **Four+ hues** — the interface becomes a dashboard, not a trading tool; color loses its instant-read quality
- **One hue** — direction becomes invisible; the most important information channel is eliminated

Two hues mean a trader can glance at any part of the screen and instantly know direction from color alone (reinforced by icons/text per accessibility requirements).

#### Neutral Variation

Everything that isn't directional uses the neutral at different opacities:

| Opacity Level | Role |
|--------------|------|
| 100% | Primary text, most important non-directional data |
| 70-80% | Secondary text, labels, headers |
| 40-50% | Tertiary text, timestamps, metadata |
| 20-30% | Disabled text, subtle indicators |
| 8-15% | Borders, dividers, row hover tints |
| 3-6% | Subtle background differentiation |

This creates a rich visual hierarchy from a single neutral — no need for gray, silver, slate, or other named neutrals. One color, many opacities.

#### When You Need a Third Color

Occasionally a non-directional semantic is unavoidable (warnings, informational states, brand accent). Handle this by:

1. **Prefer neutral treatment first** — can you communicate this with opacity, icons, or position instead of a new hue?
2. **If a hue is truly needed** — use it extremely sparingly, at low saturation, and never in contexts where it could be confused with directional color
3. **Never more than one additional hue** — the accent/warning budget is one hue total, not one per semantic

### 2.2 Opacity as the Primary Visual Variable

**Impact: CRITICAL (inconsistent visual hierarchy from ad-hoc color choices)**

Opacity is the primary tool for creating visual hierarchy, state differentiation, and depth. Not new colors. Not brightness adjustments. Alpha.

#### One Hue, Many Roles

A single color at different opacities can serve every visual role:

| Role | Opacity Approach |
|------|-----------------|
| **Background tinting** | Directional hue at 5-10% for row backgrounds |
| **Borders** | Neutral at 8-15% |
| **Hover states** | Current color + 5-10% neutral overlay |
| **Active/selected** | Current color + 10-15% neutral overlay |
| **Disabled elements** | Reduce opacity to 30-40% |
| **Text hierarchy** | Same neutral at 100%, 70%, 45%, 25% |
| **Directional backgrounds** | Positive/negative hue at 8% for position rows |

#### Why Not New Colors

Every new color in the palette is a decision the viewer must decode. "What does this shade of blue mean? Is this gray different from that gray?" Opacity variants of existing colors carry their meaning forward — a faded green is still recognizably "positive direction." A new blue is semantically blank.

#### Opacity for Directional Color

Both directional hues (positive and negative) should have graduated opacity variants for different visual roles:

| Variant | Opacity | Use |
|---------|---------|-----|
| Full | 100% | Text, icons — primary directional signal |
| Medium | 60-70% | Secondary directional elements |
| Subtle | 30-40% | Directional borders, outlines |
| Tint | 8-12% | Row/cell background tinting |
| Ghost | 3-5% | Hover backgrounds on directional elements |

This gives each directional hue five usable states without introducing any new colors.

#### Consistency Rule

If you find yourself reaching for a new color value — stop. Can this be achieved with an opacity variant of an existing color? If yes, use opacity. If genuinely no, escalate the decision — a new color in the palette is an architectural change, not a styling choice.

### 2.3 Dark-Primary Foundation

**Impact: CRITICAL (panels and chrome compete with data for visual attention)**

The default canvas is darkness. Backgrounds barely above pure black. Data is the brightest thing on screen. This is the primary theme — not the only theme. The architecture must support established community palettes, but dark is the design baseline and the default experience.

#### Why Near-Black

- **Data prominence** — on a dark canvas, even low-opacity text is visible. The data layer (text, numbers, icons) naturally becomes the most visually prominent layer without any effort.
- **Directional color purity** — green and red read most clearly against near-black. On lighter backgrounds, they compete with the background's own luminance.
- **Eye fatigue** — traders stare at screens for 12+ hours. Dark backgrounds with light text produce less eye strain than the reverse, especially in dim trading floors and home offices.
- **Elevation through brightness** — on a near-black canvas, each small brightness increment feels like a physical layer. You get depth from tiny luminance differences that would be invisible on a lighter background.

#### Surface Layering

The foundation supports a multi-level elevation system where each layer is a subtle brightness increase:

| Level | Role | Approach |
|-------|------|----------|
| Deepest | App background, the "floor" | Nearest to pure black |
| Panel level | Content panels, primary containers | Barely visible step up |
| Elevated | Cards, menus, dropdowns, popovers | Another subtle step |
| Interactive | Hover states, focused elements | Slightly brighter still |
| Active | Selected items, pressed states | Brightest background level |

The brightness increments between levels should be small enough that adjacent levels don't create harsh contrast, but large enough to be distinguishable. A good test: can you tell which layer is "above" which when they sit side by side? If not, increase the step. If the difference is obvious from across the room, decrease it.

#### What "Near-Black" Means

Not pure black (#000000) — pure black causes halation on OLED screens and looks harsh on LCDs. The base should be barely above black, with just enough brightness to feel like a surface rather than a void. Think 3-6% brightness, not 0% and not 10%.

#### Borders in a Dark System

In a near-black system, borders are critical for structure but must remain subordinate to content. Use the neutral color at very low opacity (8-15%) rather than a distinct border color. Borders should be felt more than seen — they define panel boundaries without drawing attention to themselves.

#### Theming Beyond Dark

Dark is the primary theme and design baseline, but the system must support alternative themes through the semantic token architecture. Principles:

- **Use established community palettes only** — Tokyo Night, Catppuccin, Dracula, Nord, Solarized, Rosé Pine, Gruvbox, One Dark, and similar battle-tested palettes. Never invent a color scheme — use what the community has already validated across thousands of applications.
- **Light mode is a supported variant** — some traders prefer light backgrounds, especially in bright environments. The elevation and hierarchy principles apply in reverse: data is darker than the background, surfaces descend in brightness.
- **All principles still apply** — regardless of palette, the rules hold: two directional hues, opacity-based variation, elevation through surface levels, density-first. A Tokyo Night theme still uses exactly two chromatic hues for direction, still uses opacity for depth, still earns every pixel.
- **Dark is the design target** — design and test in dark first. Other themes are adaptations mapped through the token system, not parallel designs. This keeps the design team focused and ensures the primary experience is the strongest.
- **Theme switching must be seamless** — changing theme changes only the token values, never the layout, density, or information architecture. If a theme change breaks a panel, the panel has hardcoded values.

### 2.4 Surface Elevation Model

**Impact: CRITICAL (flat UI with no visual hierarchy between layers)**

Depth in a professional trading UI comes from a structured elevation system — not shadows, not gradients, not distinct background colors. Each elevation level is a controlled brightness increment from the near-black foundation.

#### The Elevation Ladder

Define 5 elevation levels. Each level has one job:

| Level | Name | Purpose |
|-------|------|---------|
| 0 | Base | App background — the deepest layer, visible between panels |
| 1 | Panel | Primary content containers — where data lives |
| 2 | Raised | Cards, menus, dropdowns, popovers — elements above panels |
| 3 | Hover | Interactive feedback — hover state on any surface |
| 4 | Active | Selected/pressed state — highest emphasis background |

#### Design Principles

- **No custom backgrounds** — every background color in the application should map to one of these five levels. If you're defining a background that doesn't fit, the elevation system needs extension, not circumvention.
- **No shadows** — in a near-black system, drop shadows are invisible or require unrealistic spread/opacity. Elevation through brightness is cleaner and more performant.
- **No gradients on surfaces** — gradients introduce visual noise. Flat surfaces at consistent elevation levels are easier to scan.
- **Borders do the rest** — where brightness difference alone isn't enough to separate adjacent same-level elements, use a low-opacity neutral border (8-15%). This is cheaper than adding elevation levels.

#### Elevation in Context

| UI Element | Elevation Level |
|-----------|----------------|
| App background (gaps between panels) | 0 (Base) |
| Panel content area | 1 (Panel) |
| Header bar, status bar | 1 (Panel) |
| Dropdown menus, context menus | 2 (Raised) |
| Tooltips, popovers | 2 (Raised) |
| Dialog/modal backdrop | Overlay (semi-transparent black over everything) |
| Dialog/modal content | 2 (Raised) |
| Row on hover | 3 (Hover) |
| Selected row, active tab | 4 (Active) |
| Pressed button | 4 (Active) |

#### Implementation Note

Your design system should define these as semantic surface tokens, not raw color values. Components reference "surface-panel" or "surface-hover," not a specific brightness value. This allows the entire elevation system to be tuned in one place and supports theme adaptation (dark, light, or community palettes like Catppuccin or Tokyo Night) without touching component code.

The elevation ladder is theme-independent. In dark themes, elevation increases brightness. In light themes, elevation may decrease brightness or add subtle shadows. The semantic names and the 5-level structure remain constant — only the values change per theme.

---

## 3. Typography & Density

**Impact: CRITICAL**

Dual-font system, data alignment, size hierarchy, and semantic token architecture — the typographic and structural patterns that enable extreme data density.

### 3.1 Dual-Font System

**Impact: CRITICAL (misaligned numeric columns and inconsistent data presentation)**

Two font categories. No exceptions. No third font.

#### The Two Categories

| Category | Font Type | Used For |
|----------|-----------|----------|
| **Structural / Identity** | Monospace | Headings, section labels, navigation, button text, status indicators, form labels, panel titles, badges, all numeric data (prices, quantities, P&L, timestamps, order IDs, percentages) |
| **Prose / Description** | Sans-serif | Body paragraphs, descriptive copy, tooltips, help text, long-form explanations — anything that reads as a sentence |

#### Why This Matters

Monospace fonts guarantee that:
- **Columns align** — 1,234.56 and 9,876.54 occupy identical widths, enabling instant visual scanning
- **Prices are scannable** — decimal points stack vertically, letting traders compare values by position rather than reading each number
- **Changes are detectable** — when a price updates, only the changed digits shift appearance, not the entire number's width

Sans-serif fonts guarantee that:
- **Labels are compact** — proportional width means shorter strings for the same text, saving horizontal space
- **Prose stays subordinate** — the visual difference between mono headings and sans body creates automatic hierarchy without needing size jumps

#### Critical: Tabular/Lining Figures

Not all monospace fonts are equal for trading. The monospace font must support:

- **Tabular figures** — all digits the same width (not proportional oldstyle)
- **Lining figures** — digits sit on the baseline and reach cap height (not descending oldstyle figures)
- **Clear zero distinction** — 0 must be visually distinct from O at 11px

Fonts known to work well for dense numeric data: JetBrains Mono, IBM Plex Mono, Iosevka, Berkeley Mono, Cascadia Code. Test at your target size (11-13px) before committing.

#### The Line: Where Sans Meets Mono

Some elements exist at the boundary. The rule: **if it contains a number the user needs to compare or scan, it's monospace.** Examples:

| Element | Font | Reasoning |
|---------|------|-----------|
| "Positions" panel title | Sans | UI label, no numeric content |
| "Orders (3)" badge count | The "3" in mono, rest in sans — or all mono if simpler | Numeric content users scan |
| Order quantity "100" | Mono | Numeric, users compare quantities |
| "Cancel" button | Sans | Text action, no numeric content |
| "Buy 5 ES @ 4,512.25" button | Mono | Contains numbers users must verify before clicking |
| Timestamp "14:32:05" | Mono | Numeric sequence users scan |

### 3.2 Data Alignment and Size Hierarchy

**Impact: CRITICAL (traders cannot scan columns quickly when numbers don't align)**

#### Decimal Alignment

In any column of numeric data, the decimal points must align vertically. This is the single most important typographic rule for trading data. It enables:

- **Instant magnitude comparison** — "is this price bigger or smaller?" answered by vertical position of digits, not by reading
- **Change detection** — when scanning a column, misaligned decimals force re-reading; aligned decimals let the eye flow

Implementation approaches (choose based on your stack):
- Right-align numeric columns with consistent decimal places
- Use tabular figures in a monospace font (handles most cases automatically)
- Pad with non-breaking spaces if your rendering engine doesn't support tabular alignment natively
- For mixed-precision instruments, align on the decimal and let trailing digits extend

#### Size Hierarchy for Data

Use font size itself (not just opacity) to establish what matters most. In a dense trading panel:

| Level | Relative Size | Use |
|-------|--------------|-----|
| **Primary** | Base + 1-2px | Current price, total P&L, key metric the panel exists to show |
| **Standard** | Base (11-13px) | Most data: quantities, individual prices, order details |
| **Secondary** | Base - 1px | Labels, column headers, timestamps |
| **Tertiary** | Base - 2px | Metadata, IDs, supplementary info |

The difference between levels should be small (1-2px) because the base is already small. Large size jumps waste density. Subtle size differences combined with opacity hierarchy create a legible information stack without wasting vertical space.

#### Column Layout Principles

- **Right-align all numeric data** — this is how traders expect to see numbers. Left-aligned numbers in a column are always wrong.
- **Left-align text data** — symbols, names, labels.
- **Fixed column widths** — columns should not resize when data changes. A price going from 99.50 to 100.50 should not cause adjacent columns to shift. Design for the maximum expected width.
- **Header alignment matches data** — if the column data is right-aligned, the header is right-aligned.

### 3.3 Semantic Token Architecture

**Impact: CRITICAL (hardcoded values bypass theming, break consistency, and cause visual drift)**

No component should contain a raw color value, pixel measurement, or font specification. All visual properties reference semantic tokens defined in a central design system. This is a structural requirement, not a stylistic preference.

#### Token Categories

Your design system must define tokens in these categories:

| Category | Examples | Why Tokens |
|----------|----------|-----------|
| **Directional colors** | positive, negative (bid/ask, buy/sell, profit/loss) | Directional meaning must be consistent everywhere |
| **Surface levels** | surface-base, surface-panel, surface-raised, surface-hover, surface-active | Elevation system is architectural |
| **Text hierarchy** | text-primary, text-secondary, text-tertiary, text-disabled | Information hierarchy must be consistent |
| **Borders** | border-default, border-subtle | Panel structure consistency |
| **Spacing** | space-xs, space-sm, space-md, space-lg, space-xl | Layout rhythm |
| **Typography** | font-ui, font-data, size-primary, size-standard, size-secondary | Font system enforcement |

#### Why This Matters for Trading UI

Trading applications have unusually strict consistency requirements:

1. **Directional color must be identical everywhere** — if "green" means "profit" in the P&L panel but a slightly different green means "buy" in order entry, the user's subconscious pattern-matching breaks
2. **Density requires precision** — at 4px spacing units, a hardcoded "5px" gap is visibly wrong. Tokens enforce the grid.
3. **Multi-panel layouts amplify inconsistency** — traders see 6-12 panels simultaneously. A color mismatch between panels is immediately obvious.
4. **Theming is not optional** — different markets use different directional conventions (red/green in US, green/red in some Asian markets). Traders expect to customize their workspace with established palettes (Tokyo Night, Catppuccin, Dracula, Nord, etc.). Tokens make all of this a configuration change, not a code change.

#### Naming Convention

Name tokens by their **semantic role**, not their visual appearance:

| Wrong | Right | Why |
|-------|-------|-----|
| `green-500` | `color-positive` | What if positive is red in another market? |
| `dark-bg` | `surface-base` | "Dark" describes appearance, not role |
| `small-text` | `text-secondary` | "Small" is relative and not semantic |
| `gray-border` | `border-default` | Gray is a color, not a meaning |

#### Enforcement

In code review, any hardcoded color value, pixel measurement, or font name in component code is a defect — not a style issue. Treat it with the same severity as a logic bug. The only place raw values should exist is in the design system's token definition file.

---

## 4. Layout & Panels

**Impact: HIGH**

Panel architecture, docking/tiling, priority-based space allocation, required states, and responsive collapse strategies.

### 4.1 Panel Architecture and Docking

**Impact: HIGH (bad layouts waste screen density or hide critical information)**

Professional trading interfaces are modular panel systems, not page layouts. Every piece of functionality lives in a discrete, dockable panel that can be arranged, resized, and collapsed by the trader.

#### Core Layout Principles

- **Tiling, not floating** — panels tile to fill available space with no gaps. Floating windows waste the background underneath. The layout should behave like a tiling window manager: every pixel is owned by a panel.
- **Priority-based space allocation** — the chart (or primary data visualization) gets remaining space after all other panels claim their minimums. This ensures the highest-value content always has the largest area.
- **Defined minimums** — every panel has a minimum useful size. Below that size, the panel collapses rather than rendering unusably. Minimums are part of the panel's specification, not an afterthought.
- **User-controlled layout** — traders arrange their workspace once and expect it to persist. Layout state (which panels are visible, their positions, sizes) is saved and restored reliably.

#### Panel Priority Ordering

Panels have a collapse priority. When viewport space shrinks, the lowest-priority panel collapses first:

| Priority | Panels | Collapse Behavior |
|----------|--------|-------------------|
| Never collapse | Chart, order entry | These are the reason the application exists |
| Last to collapse | Positions, active orders | Critical active-state awareness |
| Early collapse | Watchlist, account info, alerts | Important but not moment-to-moment critical |
| First to collapse | Settings, logs, analytics | Reference panels the trader checks periodically |

When a panel collapses, it should show a compact indicator with key counts (e.g., "Orders (3)", "Positions (2)") so the trader knows there's active data even when the panel isn't visible.

#### Shell Structure

The application shell consists of:

1. **Header bar** (fixed) — symbol, account selector, connection status, global controls
2. **Content area** (flexible) — the dockable panel grid, resizable and rearrangeable
3. **Status bar** (fixed) — system status, connection latency, clock, global state indicators

The header and status bar are thin (24-32px each) and fixed. They never compete with the content area for space. The content area handles all the docking, splitting, and resizing.

#### Panel Composition

Each panel follows a consistent internal structure:

1. **Panel header** (thin, 20-28px) — panel title, key action buttons, collapse/close controls
2. **Panel content** — the panel's primary function
3. **Panel footer** (optional, only if needed) — summary data, status

Keep panel chrome (header + footer) as thin as possible. The content area should dominate. A panel that's 50% chrome and 50% content has failed the density test.

### 4.2 Required Panel States

**Impact: HIGH (blank panels, missing error feedback, or invisible data staleness)**

Every panel must implement all five states. A panel is not complete until each state renders correctly and communicates the right information.

#### The Five States

| State | Visual Pattern | Purpose |
|-------|---------------|---------|
| **Loading (known layout)** | Skeleton shimmer matching expected content shape | User sees that data is coming and knows roughly what to expect |
| **Loading (unknown)** | Centered spinner with context text ("Connecting to feed...") | User knows something is happening, even if layout can't be predicted |
| **Empty** | Centered icon + helpful text ("No positions. Place an order to get started.") | User knows the panel is working but has no data to show — and knows how to change that |
| **Error** | Inline banner with actionable message ("Feed disconnected. Reconnecting in 5s..." or "Failed to load orders. [Retry]") | User knows what went wrong and what they can do about it |
| **Disconnected** | Last data shown but grayed/dimmed with stale warning and timestamp. Order entry and modifications disabled. | User sees the last known state but is clearly warned it may be outdated. Prevents trading on stale data. |

#### Why Every State Matters

| Missing State | Consequence |
|--------------|-------------|
| No loading state | Panel is blank; user doesn't know if it's broken or loading |
| No empty state | Panel is blank; user doesn't know if it's loading or has nothing to show |
| No error state | Panel is blank or shows stale data; user doesn't know something is wrong |
| No disconnected state | User sees stale prices at full opacity and may trade on outdated data |

In trading, each of these consequences can lead to financial loss. Blank panels with no explanation are never acceptable.

#### Disconnected State: Special Attention

The disconnected state deserves particular focus because it's the most dangerous for the trader:

- All data remains visible but at **reduced opacity** — clearly dimmed compared to live data
- A **stale data warning** is visible without scrolling — timestamp of last update, reconnection status
- **Order entry is disabled** — the most important safety measure. Cannot submit orders on stale data.
- **Order modification/cancellation remains enabled** — cancelling existing orders on stale data is safer than leaving them active
- The transition from live to disconnected should be **instant and obvious** — no subtle fade, no delay

#### Transition Between States

State transitions should be immediate. No fade animations between states. When data arrives, the loading state is instantly replaced by data. When connection drops, the display immediately enters the disconnected state. Speed of state communication is a safety feature.

### 4.3 Responsive Collapse Strategy

**Impact: HIGH (panels overlap or become unusable at constrained viewport sizes)**

Trading applications typically run on large monitors (often multiple), but the layout must degrade gracefully when viewport space is constrained.

#### Collapse Sequence

When the viewport shrinks below the combined minimum sizes of all visible panels:

1. **Collapse lowest-priority panels first** — following the panel priority ordering, collapse the least critical panels into compact indicators
2. **Stack remaining panels vertically** — when horizontal space can no longer accommodate side-by-side panels, stack them
3. **Switch to tabbed view** — at the smallest viable size, show one panel at a time with tab navigation between them

#### Rules

- **Chart and order entry never collapse** — regardless of viewport size, these panels are always visible. They are the core function.
- **Breakpoints are defined in the design system** — not hardcoded in components. A single source of truth for layout breakpoints ensures consistent behavior.
- **Collapsed panels show data counts** — "Orders (3)" or "Positions (2 active)" so the trader maintains awareness even when the panel is collapsed.
- **Transitions are instant** — no slide animations when panels collapse or expand. The layout change should be immediate.
- **User can override** — if a trader explicitly forces a panel to stay visible, respect that even if it means other panels collapse earlier.

#### Minimum Panel Sizes

Define minimum sizes as part of each panel's specification:

| Panel Type | Minimum Width | Minimum Height | Rationale |
|-----------|--------------|----------------|-----------|
| Chart | 400px | 300px | Usable price action visibility |
| Order entry | 250px | 200px | All fields visible without scrolling |
| Positions | 300px | 100px | At least one row plus header visible |
| Order book | 200px | 200px | Enough depth levels to be useful |
| Watchlist | 200px | 100px | At least 3-4 rows visible |

These are reference values. Actual minimums depend on your specific panel content, but the principle is: define them explicitly, don't let panels render at sizes where they're unusable.

---

## 5. Data Display

**Impact: HIGH**

Conventions for rendering prices, positions, orders, P&L, alerts, and stale data.

### 5.1 Trading Data Display Conventions

**Impact: HIGH (misaligned prices, ambiguous direction, or missing context misleads traders)**

Every data type in a trading application has specific display conventions. These are not suggestions — they are the industry-standard patterns professional traders expect.

#### Price Display

- **Always monospace** — prices are the most frequently scanned numbers in the interface
- **Direction indicator: icon + color** — triangle up/down (or arrow) plus directional semantic color. Never color alone.
- **Show both absolute and percentage change** — "$4,512.50 +12.25 (+0.27%)" gives both magnitude and relative context
- **Current price is primary, change is secondary** — use size and opacity hierarchy to subordinate the change to the current price
- **Decimal alignment in columns** — all prices in a column must align on the decimal point
- **Consistent decimal places per instrument** — ES futures always show 2 decimals, BTC might show 1. Don't truncate or pad inconsistently.

#### Position Display

- **Direction badge** — a compact indicator showing "Long" or "Short" in the appropriate directional color
- **Quantity in monospace** — right-aligned, tabular figures
- **Entry price in secondary text** — visible but subordinate to current P&L
- **P&L with directional color + icon** — the most important number in the position row. Show both unrealized and realized when applicable.
- **Row tint** — the entire position row gets a subtle directional background tint (5-10% opacity of the position's directional color). This makes scanning a list of mixed long/short positions instant.

#### Order Display

- **Side indicator** — clearly shows Buy or Sell with appropriate directional color
- **All prices and quantities in monospace** — right-aligned, decimal-aligned
- **Status coloring** — pending (neutral/warning), filled (positive flash then neutral), rejected (negative), cancelled (dimmed)
- **Cancel action always visible** — on active (working) orders, the cancel button is always visible without hovering or expanding. Traders need to cancel orders instantly.
- **Time priority visible** — show order time in secondary text. Traders need to know how long an order has been working.

#### P&L Display

- **Directional color + icon** — profit in positive color with up indicator, loss in negative color with down indicator
- **Monospace, right-aligned** — like all numeric data
- **Show currency symbol or unit** — "$+1,234.56" not just "+1,234.56"
- **Realized vs unrealized distinction** — when showing both, clearly label which is which. Don't rely on position alone.
- **Daily/total toggle** — traders often need to switch between session P&L and total P&L

#### Alerts and Notifications

| Severity | Behavior | Dismissal | Position |
|----------|----------|-----------|----------|
| **Info/fills** | Transient, appears at screen edge | Auto-dismiss (3-5s) | Toast position, doesn't interrupt workflow |
| **Warning** | Visible but non-blocking | Timed (10s) or manual | Toast position, slightly more prominent |
| **Error** | Prominent, demands attention | Manual dismiss required | Inline in affected panel or prominent toast |
| **Persistent** | Inline with affected content | Until condition resolves | Inline banner in the relevant panel |

Errors must never auto-dismiss. A trader must acknowledge an error. Fills and info can auto-dismiss because they are confirmations, not problems.

#### General Data Rules

- **Stale data** — gray out with "Last update: HH:MM:SS" timestamp. Never show stale data at full opacity.
- **Empty columns** — show a dash "—" not blank space. Blank cells are ambiguous (loading? empty? error?).
- **Loading** — skeleton shimmer for known layouts, spinner for unknown. Never show a blank panel while loading.
- **Watchlist rows** — compact: symbol (left-aligned), last price (right-aligned), change indicator (right-aligned). Row interactions: click (select), double-click (open/action), right-click (context menu), drag (reorder).
- **Confirmation dialogs for high-stakes actions** — required for orders above a configurable size threshold. Show full details: side, quantity, symbol, price, estimated cost. Primary action button uses directional color. Cancel is always available and clearly labeled.

---

## 6. Interaction Design

**Impact: HIGH**

Keyboard-first workflows, error prevention, confirmation flows, focus management.

### 6.1 Keyboard-First Interaction

**Impact: HIGH (mouse-dependent workflows slow traders down in time-critical moments)**

Professional traders minimize mouse usage. Every critical action should be reachable by keyboard. The mouse is a fallback, not the primary input method.

#### Principles

- **Every action has a shortcut** — order placement, cancellation, panel navigation, symbol switching. If a trader does it frequently, it must have a keyboard shortcut.
- **Shortcuts are discoverable** — tooltips on all buttons include the keyboard shortcut. A help overlay (accessible via `?` or similar) shows all shortcuts.
- **Focus is always visible** — a visible focus ring on every interactive element, every time. If focus is invisible, keyboard users are lost. The focus ring should be high-contrast against the dark background.
- **Focus order is logical** — tab order follows visual layout, not DOM order or widget tree order. In a trading context: order entry fields tab in the sequence a trader fills them (symbol → side → quantity → price → submit).
- **Escape always cancels** — in any modal, dialog, dropdown, or input state, Escape returns to the previous state. This is universal in professional tools.

#### Trading-Specific Keyboard Patterns

| Action | Expected Pattern |
|--------|-----------------|
| Place order | Shortcut submits the current order entry form |
| Cancel all orders | Panic shortcut with confirmation (one extra keystroke, not a dialog) |
| Cancel last order | Single shortcut, no confirmation for speed |
| Flatten position | Shortcut to close all positions in current symbol |
| Switch symbol | Type-ahead search that activates from any context |
| Navigate panels | Directional shortcuts to move focus between panels |
| Cycle panel tabs | Tab-like shortcuts within a panel group |

#### Focus Management

- **Focus trap in modals** — when a confirmation dialog is open, focus cycles within the dialog. Tab doesn't escape to background panels.
- **Return focus on close** — when a dropdown or dialog closes, focus returns to the element that triggered it.
- **Panel focus** — panels should have a "focused" state that is visually distinct (subtle border highlight or header emphasis). The currently focused panel receives keyboard shortcuts.

#### Tooltips

Every icon-only button must have a tooltip. The tooltip must include:
1. **Action name** — what this button does
2. **Keyboard shortcut** — how to trigger this without the mouse

Tooltips appear after a brief hover delay (300-500ms) to avoid visual noise during fast mouse movement. They disappear immediately on mouse leave.

### 6.2 Error Prevention and Confirmation

**Impact: HIGH (accidental orders or unintended trades cause financial loss)**

Trading UI handles real money. Error prevention is not a nice-to-have — it's a core design requirement. The interface should make it difficult to do the wrong thing accidentally and easy to recover when mistakes happen.

#### Confirmation Flow Requirements

High-stakes actions require confirmation. At minimum:

| Action | Confirmation Required | Details in Confirmation |
|--------|----------------------|------------------------|
| Order placement (above threshold) | Yes | Side, quantity, symbol, price, order type, estimated cost |
| Position close/flatten | Yes | Symbol, current P&L, quantity being closed |
| Cancel all orders | Yes | Count of orders being cancelled, symbols affected |
| Modify working order | Context-dependent | Original vs new values highlighted |

#### Confirmation Dialog Design

- **Show full details** — the trader must see exactly what will happen. "Are you sure?" with no context is useless.
- **Primary action button matches direction** — a buy confirmation's primary button uses the positive directional color. A sell confirmation uses the negative directional color. This provides one more visual check.
- **Cancel is always available** — prominent, keyboard-accessible, and never hidden
- **No nested confirmations** — one confirmation per action. "Are you really sure?" after "Are you sure?" is hostile UX
- **Configurable thresholds** — what constitutes a "large" order varies by trader and instrument. The size threshold that triggers confirmation should be configurable.

#### Prevention Over Confirmation

Better than confirming a mistake is preventing it:

- **Quantity validation** — reject obviously wrong quantities (10x the typical size, negative numbers, zero)
- **Price validation** — warn when limit price is far from market (potential fat-finger)
- **Symbol verification** — highlight when the order symbol doesn't match the currently viewed chart
- **Side verification** — visually emphasize buy vs sell throughout the order entry process so the trader always knows which direction they're trading

#### Recovery

When mistakes happen:
- **Cancel is always one action away** — on every working order row, cancel is visible and clickable without expanding or hovering
- **Undo where possible** — if an order hasn't been sent to the exchange yet, allow undo
- **Clear error messages** — when an order is rejected, show why in plain language with the rejected order details

---

## 7. Component Philosophy

**Impact: MEDIUM**

When to compose vs custom-render, ShadCN-density adaptation, trading widget patterns.

### 7.1 Component Design Approach

**Impact: MEDIUM (inconsistent components across panels, or over-engineering where simplicity suffices)**

Trading UI components fall into two categories. Choose the right approach for each.

#### Two Approaches

**Composed components** — assembled from existing primitives (text, row, column, button, input). Use for most trading widgets.

- PriceDisplay, PositionBadge, PnlDisplay, AlertBanner, NumericStepper, SymbolSearch, StatusIndicator
- Faster to build, automatically inherit framework accessibility and interaction patterns
- Easier to maintain consistency because they use the same primitives as everything else

**Custom-rendered components** — drawn directly via canvas, WebGL, GPU primitives, or equivalent. Use only for performance-critical visualization.

- Charts (candlestick, depth, time & sales)
- DOM/order book with high-frequency updates
- Heatmaps, volume profiles
- Any component that needs to render 1000+ data points at 60fps

The threshold is simple: if a composed component can maintain 60fps with your data volume, use composition. If it can't, drop to custom rendering for that specific component.

#### Component Density: The ShadCN Model, Compressed

ShadCN/ui demonstrates excellent component design principles: consistent tokens, composable primitives, clear visual hierarchy. For trading, apply the same principles but at higher density:

| ShadCN Pattern | Trading Adaptation |
|---------------|-------------------|
| Generous padding (px-4 py-2) | Minimal padding (px-2 py-1 or less) |
| Comfortable line-height | Tight line-height (1.2-1.3) |
| Standard 14-16px body text | 11-13px data text |
| Card-based layouts with gaps | Edge-to-edge panels with minimal gaps |
| Rounded corners (radius-md) | Zero radius. Sharp corners on everything — cards, buttons, inputs, modals, badges, dropdowns. No exceptions. |
| Prominent hover states | Subtle hover (opacity shift, not color change) |

The principle is the same (consistent, composable, token-driven) but the spatial budget is dramatically smaller.

#### Trading Widget Patterns

Standard widgets every trading interface needs. These are patterns, not implementations — build them in whatever stack you're using:

| Widget | Purpose | Key Design Requirements |
|--------|---------|------------------------|
| **PriceDisplay** | Current price + change | Monospace, directional color + icon, absolute and percentage change |
| **PositionBadge** | Compact position indicator | Direction (long/short), quantity, P&L — all in one dense row |
| **PnlDisplay** | P&L with breakdown | Monospace, directional color + icon, realized/unrealized distinction |
| **NumericStepper** | Precise numeric input | Step size tied to instrument tick size, keyboard + scroll support, min/max bounds |
| **SymbolSearch** | Instrument lookup | Type-ahead, fuzzy match, recent history, compact results list |
| **StatusIndicator** | Connection/system health | Color-coded dot + label, semantic color from tokens |
| **AlertBanner** | Dismissible notification | Severity levels, inline or toast, manual or auto-dismiss per severity |
| **OrderTicket** | Order entry form | Side toggle, quantity stepper, price input, order type selector, submit with directional color |

#### Component Checklist

Before any component is considered complete:

- [ ] All visual values from semantic tokens (no hardcoded colors, sizes, fonts)
- [ ] Numeric data in monospace with tabular figures
- [ ] Directional data has both color and icon/text indicator
- [ ] All five panel states handled (loading, empty, error, disconnected, data)
- [ ] Keyboard accessible with visible focus indicator
- [ ] Tooltips on all icon-only interactive elements
- [ ] Tested at target density (not just "looks good in isolation")

---

## 8. Accessibility

**Impact: MEDIUM**

Contrast requirements, never-color-alone principle, focus indicators, cross-platform rendering.

### 8.1 Accessibility and Cross-Platform

**Impact: MEDIUM (unusable interface for visually impaired users or on specific platforms)**

#### Contrast Requirements

Dark interfaces must meet contrast ratios carefully — it's easy to fail on a near-black background:

| Element | Minimum Ratio | Standard |
|---------|--------------|----------|
| Body text (< 18px) | 4.5:1 | WCAG AA |
| Large text (>= 18px) | 3:1 | WCAG AA |
| Interactive element boundaries | 3:1 | WCAG 2.1 |
| Focus indicators | 3:1 | WCAG 2.1 |

Test contrast for every text opacity level in your hierarchy. The tertiary/disabled text levels are the most likely to fail. If your "disabled" text is invisible on near-black, it fails accessibility even though "disabled things are supposed to look faded."

#### Never Color Alone

This is the most important accessibility rule for trading UI. Directional color (positive/negative) must always be reinforced with a second indicator:

| Color Indicator | Required Reinforcement |
|----------------|----------------------|
| Green price change | Up arrow/triangle icon |
| Red P&L | Down arrow/triangle icon |
| Buy/sell button colors | "Buy"/"Sell" text label |
| Position direction color | "Long"/"Short" text badge |
| Status indicator color | Status text label |

~8% of men have color vision deficiency. A trading interface that relies on color alone for direction makes one in twelve male traders unable to distinguish buy from sell. This is not an edge case — it's a structural failure.

#### Focus Indicators

- Every interactive element must have a visible focus indicator
- The focus ring must be visible against both the dark background and any surface level
- Use a high-contrast color for focus (the accent/brand color works well here since it's not directional)
- Focus rings should be 2px minimum width
- Never remove focus indicators for aesthetic reasons — if the default focus ring looks wrong, restyle it, don't hide it

#### Cross-Platform Rendering

If your application targets multiple platforms:

- **Design at 1x (96 DPI)** — this is your baseline. All measurements and visual rules apply at 1x.
- **Test at 100%, 125%, 150%, 200% scaling** — the most common DPI settings. Ensure text remains readable, borders remain visible (a 1px border at 150% can become blurry), and spacing scales proportionally.
- **Use vector assets** — raster icons and images will blur at non-integer scale factors. SVG, font icons, or programmatically drawn graphics scale cleanly.
- **Font rendering varies** — the same font at the same size will look different across FreeType (Linux), DirectWrite (Windows), and Core Text (macOS). Test on all target platforms, especially at your small data text sizes (11-13px) where rendering differences are most visible.
- **Custom window chrome** — if using custom title bars, test them on all platforms. Native window management behavior (snap, resize, minimize) must work correctly.
- **Never assume pixel-perfect cross-platform rendering** — design with enough margin that minor rendering differences don't break the layout.
