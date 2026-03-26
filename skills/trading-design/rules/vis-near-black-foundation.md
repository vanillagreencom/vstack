---
title: Dark-Primary Foundation
impact: CRITICAL
impactDescription: Panels and chrome compete with data for visual attention
tags: background, surface, darkness, canvas, theming
---

## Dark-Primary Foundation

**Impact: CRITICAL (panels and chrome compete with data for visual attention)**

The default canvas is darkness. Backgrounds barely above pure black. Data is the brightest thing on screen. This is the primary theme — not the only theme. The architecture must support established community palettes, but dark is the design baseline and the default experience.

### Why Near-Black

- **Data prominence** — on a dark canvas, even low-opacity text is visible. The data layer (text, numbers, icons) naturally becomes the most visually prominent layer without any effort.
- **Directional color purity** — green and red read most clearly against near-black. On lighter backgrounds, they compete with the background's own luminance.
- **Eye fatigue** — traders stare at screens for 12+ hours. Dark backgrounds with light text produce less eye strain than the reverse, especially in dim trading floors and home offices.
- **Elevation through brightness** — on a near-black canvas, each small brightness increment feels like a physical layer. You get depth from tiny luminance differences that would be invisible on a lighter background.

### Surface Layering

The foundation supports a multi-level elevation system where each layer is a subtle brightness increase:

| Level | Role | Approach |
|-------|------|----------|
| Deepest | App background, the "floor" | Nearest to pure black |
| Panel level | Content panels, primary containers | Barely visible step up |
| Elevated | Cards, menus, dropdowns, popovers | Another subtle step |
| Interactive | Hover states, focused elements | Slightly brighter still |
| Active | Selected items, pressed states | Brightest background level |

The brightness increments between levels should be small enough that adjacent levels don't create harsh contrast, but large enough to be distinguishable. A good test: can you tell which layer is "above" which when they sit side by side? If not, increase the step. If the difference is obvious from across the room, decrease it.

### What "Near-Black" Means

Not pure black (#000000) — pure black causes halation on OLED screens and looks harsh on LCDs. The base should be barely above black, with just enough brightness to feel like a surface rather than a void. Think 3-6% brightness, not 0% and not 10%.

### Borders in a Dark System

In a near-black system, borders are critical for structure but must remain subordinate to content. Use the neutral color at very low opacity (8-15%) rather than a distinct border color. Borders should be felt more than seen — they define panel boundaries without drawing attention to themselves.

### Theming Beyond Dark

Dark is the primary theme and design baseline, but the system must support alternative themes through the semantic token architecture. Principles:

- **Use established community palettes only** — Tokyo Night, Catppuccin, Dracula, Nord, Solarized, Rosé Pine, Gruvbox, One Dark, and similar battle-tested palettes. Never invent a color scheme — use what the community has already validated across thousands of applications.
- **Light mode is a supported variant** — some traders prefer light backgrounds, especially in bright environments. The elevation and hierarchy principles apply in reverse: data is darker than the background, surfaces descend in brightness.
- **All principles still apply** — regardless of palette, the rules hold: two directional hues, opacity-based variation, elevation through surface levels, density-first. A Tokyo Night theme still uses exactly two chromatic hues for direction, still uses opacity for depth, still earns every pixel.
- **Dark is the design target** — design and test in dark first. Other themes are adaptations mapped through the token system, not parallel designs. This keeps the design team focused and ensures the primary experience is the strongest.
- **Theme switching must be seamless** — changing theme changes only the token values, never the layout, density, or information architecture. If a theme change breaks a panel, the panel has hardcoded values.
