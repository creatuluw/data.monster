# LRS Design System — Complete Node Tree

**Specification:** DS2-GLACIER-7742  
**Version:** 2.0  
**Color Space:** OKLCH  
**Seed:** DS2-GLACIER-7742

---

## Root Tokens (`:root`)

```
:root
├── Color Tokens (OKLCH)
│   ├── Accent
│   │   ├── --color-accent: oklch(0.69 0.16 41)
│   │   ├── --color-accent-light: oklch(0.78 0.09 41)
│   │   ├── --color-accent-dark: oklch(0.44 0.14 41)
│   │   └── --color-accent-muted: oklch(0.93 0.03 41)
│   ├── Surface
│   │   ├── --color-surface: oklch(0.98 0.003 250)
│   │   ├── --color-surface-raised: oklch(0.96 0.005 250)
│   │   └── --color-surface-sunken: oklch(0.935 0.006 250)
│   ├── Text
│   │   ├── --color-text: oklch(0.14 0.01 250)
│   │   ├── --color-text-secondary: oklch(0.4 0.012 250)
│   │   ├── --color-text-tertiary: oklch(0.58 0.01 250)
│   │   └── --color-text-on-accent: oklch(0.97 0.005 250)
│   ├── Border / Grid
│   │   ├── --color-border: oklch(0.86 0.006 250)
│   │   ├── --color-border-strong: oklch(0.72 0.012 250)
│   │   └── --color-grid: oklch(0.9 0.004 250)
│   └── Semantic
│       ├── --color-success: oklch(0.68 0.14 160)
│       ├── --color-warning: oklch(0.76 0.13 80)
│       └── --color-danger: oklch(0.58 0.17 22)
│
├── Typography Tokens
│   ├── Font Families
│   │   ├── --font-display: 'Source Serif 4', serif
│   │   ├── --font-body: 'Manrope', sans-serif
│   │   └── --font-mono: 'Courier New', Courier, monospace
│   ├── Type Scale (1.25× Major Third)
│   │   ├── --text-xs: 0.6875rem
│   │   ├── --text-sm: 0.8125rem
│   │   ├── --text-base: 1rem
│   │   ├── --text-md: 1.25rem
│   │   ├── --text-lg: 1.563rem
│   │   ├── --text-xl: 1.953rem
│   │   ├── --text-2xl: 2.441rem
│   │   ├── --text-3xl: clamp(2.8rem, 5.5vw, 3.815rem)
│   │   └── --text-4xl: clamp(3.2rem, 7vw, 5.06rem)
│   └── Line Heights
│       ├── --leading-tight: 1.08
│       ├── --leading-snug: 1.25
│       ├── --leading-normal: 1.6
│       └── --leading-relaxed: 1.8
│
├── Spacing Tokens (4pt base)
│   ├── --space-1: 0.25rem  (4px)
│   ├── --space-2: 0.5rem   (8px)
│   ├── --space-3: 0.75rem  (12px)
│   ├── --space-4: 1rem     (16px)
│   ├── --space-6: 1.5rem   (24px)
│   ├── --space-8: 2rem     (32px)
│   ├── --space-12: 3rem    (48px)
│   ├── --space-16: 4rem    (64px)
│   └── --space-24: 6rem    (96px)
│
├── Radius Tokens
│   ├── --radius-xs: 2px
│   ├── --radius-sm: 3px
│   ├── --radius-md: 4px
│   ├── --radius-lg: 6px
│   ├── --radius-xl: 10px
│   └── --radius-full: 9999px
│
├── Shadow Tokens
│   ├── --shadow-sm: 0 1px 2px oklch(0.22 0.005 250 / 0.07)
│   ├── --shadow-md: 0 2px 8px oklch(0.22 0.005 250 / 0.09)
│   └── --shadow-lg: 0 6px 24px oklch(0.22 0.005 250 / 0.12)
│
├── Motion Tokens
│   ├── Easing
│   │   ├── --ease-out-expo: cubic-bezier(0.16, 1, 0.3, 1)
│   │   ├── --ease-out-quart: cubic-bezier(0.25, 1, 0.5, 1)
│   │   └── --ease-industrial: cubic-bezier(0.22, 0.61, 0.36, 1)
│   └── Duration
│       ├── --duration-fast: 120ms
│       ├── --duration-base: 250ms
│       ├── --duration-slow: 450ms
│       └── --duration-enter: 600ms
│
├── Layout
│   ├── --max-width: 72rem
│   └── --gutter: clamp(1rem, 3vw, 2rem)
│
└── Grid Overlay
    ├── --grid-color: oklch(0.9 0.004 250 / 0.35)
    └── --grid-spacing: 24px
```

---

## Base Styles

```
*
├── margin: 0
├── padding: 0
└── box-sizing: border-box

html
├── scroll-behavior: smooth
├── -webkit-font-smoothing: antialiased
└── -moz-osx-font-smoothing: grayscale

body
├── font-family: var(--font-body)
├── font-size: var(--text-base)
├── line-height: var(--leading-normal)
├── color: var(--color-text)
└── background: var(--color-surface)

img
├── display: block
└── max-width: 100%
```

---

## Grid Background Effect

```
.grid-bg
└── ::before
    ├── content: ''
    ├── position: absolute / inset: 0
    ├── background-image:
    │   ├── repeating-linear-gradient(90deg, var(--grid-color) 0 1px, transparent 1px var(--grid-spacing))
    │   └── repeating-linear-gradient(0deg, var(--grid-color) 0 1px, transparent 1px var(--grid-spacing))
    ├── background-size: var(--grid-spacing) var(--grid-spacing)
    ├── pointer-events: none
    ├── z-index: 0
    ├── opacity: 0
    └── transition: opacity var(--duration-slow) ease

.grid-bg:hover::before
└── opacity: 1
```

---

## Annotation Utility

```
.annotation::after
├── content: attr(data-measure)
├── position: absolute
├── font-family: var(--font-mono)
├── font-size: 9px
├── line-height: 1
├── color: var(--color-accent-light)
├── letter-spacing: 0.04em
└── white-space: nowrap

.annotation-right::after  → right: 0, top: 50%, translateX(100% + 6px) translateY(-50%)
.annotation-bottom::after → bottom: 0, left: 50%, translateX(-50%) translateY(100% + 4px)
```

---

## Entrance Animations

```
@keyframes slideUp
├── from: opacity 0, translateY(12px)
└── to:   opacity 1, translateY(0)

@keyframes typeIn
├── from: opacity 0, letter-spacing 0.12em
└── to:   opacity 1, letter-spacing inherit

.entrance
└── animation: slideUp var(--duration-enter) var(--ease-out-expo) both

.entrance-d1 → delay: 60ms
.entrance-d2 → delay: 120ms
.entrance-d3 → delay: 180ms
.entrance-d4 → delay: 240ms
.entrance-d5 → delay: 300ms

@media (prefers-reduced-motion: reduce)
└── .entrance* → animation: none, opacity: 1
```

---

## Navigation

```
.nav
├── position: sticky / top: 0
├── z-index: 100
├── background: oklch(0.98 0.003 250 / 0.92)
├── backdrop-filter: blur(10px)
└── border-bottom: 1px solid var(--color-border)

.nav-inner
├── max-width: var(--max-width)
├── margin: 0 auto
├── padding: var(--space-3) var(--gutter)
├── display: flex
├── align-items: center
└── justify-content: space-between

.nav-brand
├── font-family: var(--font-display)
├── font-size: var(--text-md)
├── font-weight: 600
├── letter-spacing: -0.02em
├── color: var(--color-text)
├── text-decoration: none
├── display: flex
├── align-items: center
└── gap: var(--space-2)

.nav-brand .brand-mark
├── width: 20px / height: 20px
├── border: 2px solid var(--color-accent)
├── border-radius: var(--radius-xs)
├── display: inline-flex
├── align-items: center
└── ::after → content: '', width: 6px, height: 6px, background: var(--color-accent), border-radius: 1px

.nav-seed
├── font-family: var(--font-mono)
├── font-size: 9px
├── letter-spacing: 0.08em
├── color: var(--color-text-tertiary)
├── padding: 2px 6px
├── border: 1px solid var(--color-border)
├── border-radius: var(--radius-xs)
└── margin-left: var(--space-3)

.nav-links
├── display: flex
├── gap: var(--space-1)
└── list-style: none

.nav-links a
├── font-family: var(--font-body)
├── font-size: var(--text-sm)
├── font-weight: 500
├── color: var(--color-text-secondary)
├── text-decoration: none
├── padding: var(--space-2) var(--space-3)
├── border-radius: var(--radius-xs)
├── transition: color var(--duration-fast) ease, background var(--duration-fast) ease
├── :hover → color: var(--color-accent), background: var(--color-accent-muted)
└── :focus-visible → outline: 2px solid var(--color-accent), outline-offset: 1px

.nav-links content → [Colors, Type, Spacing, Components, Motion, Principles]
```

---

## Hero Section

```
.hero
├── max-width: var(--max-width)
├── margin: 0 auto
├── padding: var(--space-24) var(--gutter) var(--space-16)
├── class: grid-bg
│
├── .hero-classification
│   ├── content: "Design System Specification — v2.0"
│   ├── display: inline-flex
│   ├── align-items: center
│   ├── gap: var(--space-2)
│   ├── font-family: var(--font-mono)
│   ├── font-size: 10px
│   ├── letter-spacing: 0.12em
│   ├── text-transform: uppercase
│   ├── color: var(--color-text-tertiary)
│   ├── margin-bottom: var(--space-6)
│   └── .class-line → width: 32px, height: 1px, background: var(--color-border-strong)
│
├── .hero-title
│   ├── content: "Clarity by" + br + .accent-word "structure."
│   ├── font-family: var(--font-display)
│   ├── font-size: var(--text-4xl)
│   ├── font-weight: 700
│   ├── line-height: var(--leading-tight)
│   ├── letter-spacing: -0.03em
│   ├── color: var(--color-text)
│   ├── max-width: 16ch
│   └── .accent-word → color: var(--color-accent)
│
├── .hero-desc
│   ├── content: "A design system for teaching, learning, and progress tracking — where clarity reduces cognitive load and every element serves the people registering, reviewing, and reporting student growth. No decoration without purpose. No ambiguity without consequence. Built for the teachers who track progress for every student."
│   ├── margin-top: var(--space-8)
│   ├── font-size: var(--text-md)
│   ├── line-height: var(--leading-relaxed)
│   ├── color: var(--color-text-secondary)
│   └── max-width: 52ch
│
└── .hero-specs
    ├── display: grid
    ├── grid-template-columns: repeat(auto-fit, minmax(140px, 1fr))
    ├── gap: 1px
    ├── background: var(--color-border)
    ├── max-width: 600px
    │
    ├── .hero-spec[1]
    │   ├── .hero-spec-label → "Color Space"
    │   │   └── font: var(--font-mono) 9px / letter-spacing: 0.1em / color: var(--color-text-tertiary)
    │   └── .hero-spec-value → "OKLCH"
    │       └── font: var(--font-display) var(--text-lg) weight 600 / letter-spacing: -0.02em / color: var(--color-text)
    │
    ├── .hero-spec[2]
    │   ├── .hero-spec-label → "Type Scale"
    │   └── .hero-spec-value → "1.25×"
    │
    ├── .hero-spec[3]
    │   ├── .hero-spec-label → "Grid Base"
    │   └── .hero-spec-value → "4 px"
    │
    └── .hero-spec[4]
        ├── .hero-spec-label → "Fonts"
        └── .hero-spec-value → "2"

.hero-spec (base)
├── background: var(--color-surface)
├── padding: var(--space-4) var(--space-6)
├── display: flex
├── flex-direction: column
└── gap: var(--space-1)
```

---

## Section 01 — Color Palette (`#colors`)

```
.section
├── max-width: var(--max-width)
├── margin: 0 auto
├── padding: var(--space-16) var(--gutter)
└── position: relative

.section-header
├── display: flex
├── align-items: baseline
├── gap: var(--space-4)
└── margin-bottom: var(--space-12)

.section-number
├── font: var(--font-mono) var(--text-xs)
├── letter-spacing: 0.1em
├── color: var(--color-accent)
├── padding: var(--space-1) var(--space-2)
├── border: 1px solid var(--color-accent-muted)
├── border-radius: var(--radius-xs)
├── background: var(--color-accent-muted)
└── white-space: nowrap

.section-title
├── font: var(--font-display) var(--text-xl) weight 700
├── letter-spacing: -0.02em
├── line-height: var(--leading-snug)
└── color: var(--color-text)

.section-number: "01"
.section-title: "Color Palette"

.color-grid
├── display: grid
├── grid-template-columns: repeat(auto-fit, minmax(180px, 1fr))
├── gap: 1px
└── background: var(--color-border)

.color-swatch
├── background: var(--color-surface)
├── overflow: hidden
├── cursor: pointer
├── transition: background var(--duration-fast) ease
├── position: relative
├── tabindex: 0
├── role: button
├── :hover → background: var(--color-surface-raised)
├── :focus-visible → outline: 2px solid var(--color-accent), outline-offset: -2px, z-index: 1
│
├── .color-swatch-fill
│   ├── height: 80px
│   └── ::after → content: attr(data-token), position: absolute, bottom: 6px, right: 8px
│              font: var(--font-mono) 8px, letter-spacing: 0.06em
│              color: oklch(1 0 0 / 0.6), mix-blend-mode: difference
│
├── .color-swatch-info
│   ├── padding: var(--space-3) var(--space-4)
│   ├── display: flex / flex-direction: column / gap: 2px
│   ├── .color-swatch-name → font: var(--font-display) var(--text-sm) weight 600, color: var(--color-text)
│   └── .color-swatch-value → font: var(--font-mono) 10px, color: var(--color-text-tertiary), letter-spacing: 0.02em
│
└── .copy-confirm
    ├── position: absolute / inset: 0
    ├── background: oklch(0.69 0.16 41 / 0.9)
    ├── display: flex / align-items: center / justify-content: center
    ├── font: var(--font-body) var(--text-sm) weight 600
    ├── color: var(--color-text-on-accent)
    ├── opacity: 0
    └── transition: opacity var(--duration-fast) ease

Swatches:
├── Accent         → background: var(--color-accent)         → oklch(0.69 0.16 41)
├── Accent Light   → background: var(--color-accent-light)   → oklch(0.78 0.09 41)
├── Accent Dark    → background: var(--color-accent-dark)    → oklch(0.44 0.14 41)
├── Accent Muted   → background: var(--color-accent-muted)   → oklch(0.93 0.03 41)
├── Surface        → background: var(--color-surface)        → oklch(0.98 0.003 250)  + border-bottom: 1px solid var(--color-border)
├── Raised         → background: var(--color-surface-raised) → oklch(0.96 0.005 250)  + border-bottom: 1px solid var(--color-border)
├── Sunken         → background: var(--color-surface-sunken) → oklch(0.935 0.006 250) + border-bottom: 1px solid var(--color-border)
├── Text Primary   → background: var(--color-text)           → oklch(0.14 0.01 250)
├── Text Secondary → background: var(--color-text-secondary) → oklch(0.4 0.012 250)
├── Success        → background: var(--color-success)        → oklch(0.68 0.14 160)
├── Warning        → background: var(--color-warning)        → oklch(0.76 0.13 80)
└── Danger         → background: var(--color-danger)         → oklch(0.58 0.17 22)
```

---

## Section 02 — Typography (`#type`)

```
.section-number: "02"
.section-title: "Typography"

.type-specimen
├── display: flex
├── flex-direction: column
│
└── .type-row (each)
    ├── display: grid
    ├── grid-template-columns: 140px 1fr
    ├── gap: var(--space-6)
    ├── align-items: baseline
    ├── padding: var(--space-6) 0
    ├── border-bottom: 1px dashed var(--color-border)
    │
    ├── .type-meta
    │   ├── display: flex / flex-direction: column / gap: 3px
    │   ├── .type-token-name → font: var(--font-mono) 10px weight 700, letter-spacing: 0.08em, color: var(--color-accent)
    │   └── .type-token-value → font: var(--font-mono) 10px, color: var(--color-text-tertiary), letter-spacing: 0.02em
    │
    └── .type-sample-display / .type-sample-body
        ├── display → font: var(--font-display), line-height: var(--leading-tight), letter-spacing: -0.02em, color: var(--color-text)
        └── body    → font: var(--font-body), line-height: var(--leading-normal), color: var(--color-text), max-width: 62ch

Type Scale Specimens:
├── --text-4xl
│   ├── token-value: "clamp(3.2, 7vw, 5.06)"
│   ├── specimen: display
│   ├── font-size: var(--text-4xl)
│   ├── font-weight: 800
│   ├── letter-spacing: -0.04em
│   └── sample: "Glacier"
│
├── --text-3xl
│   ├── token-value: "clamp(2.8, 5.5vw, 3.815)"
│   ├── specimen: display
│   ├── font-size: var(--text-3xl)
│   ├── font-weight: 700
│   ├── letter-spacing: -0.03em
│   └── sample: "Specification"
│
├── --text-2xl
│   ├── token-value: "2.441rem / 700"
│   ├── specimen: display
│   ├── font-size: var(--text-2xl)
│   ├── font-weight: 700
│   └── sample: "Subsystem Header"
│
├── --text-xl
│   ├── token-value: "1.953rem / 600"
│   ├── specimen: display
│   ├── font-size: var(--text-xl)
│   ├── font-weight: 600
│   └── sample: "Component Title"
│
├── --text-lg
│   ├── token-value: "1.563rem / 600"
│   ├── specimen: display
│   ├── font-size: var(--text-lg)
│   ├── font-weight: 600
│   └── sample: "Section Label"
│
├── --text-md
│   ├── token-value: "1.25rem / 500"
│   ├── specimen: display
│   ├── font-size: var(--text-md)
│   ├── font-weight: 500
│   └── sample: "Subsection"
│
├── --text-base
│   ├── token-value: "1rem / 400"
│   ├── specimen: body
│   ├── font-size: var(--text-base)
│   ├── font-weight: 400
│   └── sample: "Manrope brings geometric clarity with generous spacing and a friendly character. Its clean lines and balanced x-height keep paragraphs comfortable for long sessions, while the slightly informal touch keeps text from feeling clinical or cold."
│
├── --text-sm
│   ├── token-value: "0.8125rem / 400"
│   ├── specimen: body
│   ├── font-size: var(--text-sm)
│   ├── font-weight: 400
│   └── sample: "Small body for metadata, timestamps, and secondary readings. Subordinate to the main flow but never disposable."
│
└── --text-xs
    ├── token-value: "0.6875rem / 600"
    ├── specimen: display
    ├── font-size: var(--text-xs)
    ├── font-weight: 600
    ├── letter-spacing: 0.08em
    └── sample: "Classification / Reference"

Font Pair Grid:
.font-pair-grid
├── display: grid
├── grid-template-columns: 1fr 1fr
├── gap: 1px
├── background: var(--color-border)
├── margin-top: var(--space-12)
│
├── Font Card 1 — Display
│   ├── .font-pair-label: "Display Font"
│   │   └── font: var(--font-mono) 9px, letter-spacing: 0.1em, text-transform: uppercase, color: var(--color-text-tertiary)
│   ├── .font-pair-name: "Source Serif 4"
│   │   └── font: var(--font-display) var(--text-xl) weight 700, letter-spacing: -0.02em, color: var(--color-text)
│   ├── .font-pair-classification: "Variable optical serif / Angle-accent terminals"
│   │   └── font: var(--font-mono) 10px, color: var(--color-text-tertiary)
│   └── .font-pair-sample (display-sample)
│       ├── font-family: var(--font-display), font-weight: 600, letter-spacing: -0.02em
│       ├── font-size: var(--text-md), line-height: var(--leading-relaxed)
│       └── content: full alphabet + digits + "Precision. Warmth. Memory."
│
└── Font Card 2 — Body
    ├── .font-pair-label: "Body Font"
    ├── .font-pair-name: "Manrope"
    │   └── style: font-family: var(--font-body)
    ├── .font-pair-classification: "Variable sans / Geometric / Expanded metrics"
    └── .font-pair-sample (body-sample)
        ├── font-family: var(--font-body), font-weight: 400
        └── content: full alphabet + digits + "Clear. Open. Kind."
```

---

## Section 03 — Spacing Scale (`#spacing`)

```
.section-number: "03"
.section-title: "Spacing Scale"

.spacing-specimen
├── display: flex / flex-direction: column
│
└── .spacing-row (each)
    ├── display: flex / align-items: center / gap: var(--space-4)
    ├── padding: var(--space-2) 0
    ├── border-bottom: 1px dashed var(--color-border)
    │
    ├── .spacing-label
    │   ├── font: var(--font-mono) 10px weight 700
    │   ├── letter-spacing: 0.06em
    │   ├── color: var(--color-text-secondary)
    │   ├── min-width: 72px
    │   └── text-align: right
    │
    ├── .spacing-bar-track
    │   ├── flex: 1
    │   ├── height: 16px
    │   ├── background: var(--color-surface-sunken)
    │   └── .spacing-bar-fill
    │       ├── height: 100%
    │       ├── background: var(--color-accent)
    │       ├── opacity: 0.55
    │       ├── transition: width var(--duration-slow) var(--ease-out-expo)
    │       └── ::after → width: 1px, right: 0, background: var(--color-accent), opacity: 1
    │
    ├── .spacing-px → font: var(--font-mono) 10px, color: var(--color-text-tertiary), min-width: 32px
    └── .spacing-rem → font: var(--font-mono) 10px, color: var(--color-accent-light), min-width: 48px

Scale:
├── --space-1  → width: 4.2%   → "4px"   "0.25r"
├── --space-2  → width: 8.3%   → "8px"   "0.5r"
├── --space-3  → width: 12.5%  → "12px"  "0.75r"
├── --space-4  → width: 16.7%  → "16px"  "1r"
├── --space-6  → width: 25%    → "24px"  "1.5r"
├── --space-8  → width: 33.3%  → "32px"  "2r"
├── --space-12 → width: 50%    → "48px"  "3r"
├── --space-16 → width: 66.7%  → "64px"  "4r"
└── --space-24 → width: 100%   → "96px"  "6r"
```

---

## Section 04 — Components (`#components`)

```
.section-number: "04"
.section-title: "Components"

.component-area
├── display: flex / flex-direction: column / gap: var(--space-16)

.component-group
├── position: relative
├── .component-group-label
│   ├── font: var(--font-mono) 9px
│   ├── letter-spacing: 0.14em
│   ├── text-transform: uppercase
│   ├── color: var(--color-text-tertiary)
│   ├── margin-bottom: var(--space-6)
│   ├── display: flex / align-items: center / gap: var(--space-2)
│   └── ::before → content: '■', color: var(--color-accent), font-size: 7px
│
└── .component-group-title
    ├── font: var(--font-display) var(--text-md) weight 700
    ├── letter-spacing: -0.01em
    ├── color: var(--color-text)
    └── margin-bottom: var(--space-6)
```

### Buttons

```
component-group-label: "Interactive Elements"
component-group-title: "Buttons"

.btn (base)
├── display: inline-flex / align-items: center / gap: var(--space-2)
├── padding: var(--space-2) var(--space-4)
├── font: var(--font-body) var(--text-xs) weight 600
├── line-height: 1.4
├── cursor: pointer
├── border: 1px solid transparent
├── border-radius: var(--radius-xs)
├── transition: transform var(--duration-fast) var(--ease-out-expo),
│              background var(--duration-fast) ease,
│              color var(--duration-fast) ease,
│              border-color var(--duration-fast) ease
├── :focus-visible → outline: 2px solid var(--color-accent), outline-offset: 2px
└── :active → transform: scale(0.97)

Variants:
├── .btn-primary
│   ├── background: var(--color-accent)
│   ├── color: var(--color-text-on-accent)
│   ├── border-color: var(--color-accent)
│   └── :hover → background: var(--color-accent-dark), border-color: var(--color-accent-dark)
│
├── .btn-secondary
│   ├── background: transparent
│   ├── color: var(--color-text)
│   ├── border-color: var(--color-border-strong)
│   └── :hover → border-color: var(--color-accent), color: var(--color-accent)
│
├── .btn-ghost
│   ├── background: transparent
│   ├── color: var(--color-text-secondary)
│   ├── border-color: transparent
│   └── :hover → background: var(--color-surface-sunken), color: var(--color-text)
│
├── .btn-danger
│   ├── background: var(--color-danger)
│   ├── color: oklch(0.97 0.005 22)
│   ├── border-color: var(--color-danger)
│   └── :hover → background: oklch(0.48 0.15 22), border-color: oklch(0.48 0.15 22)
│
├── .btn-sm  → padding: var(--space-1) var(--space-3), font-size: 9px
└── .btn-lg  → padding: var(--space-3) var(--space-8), font-size: var(--text-sm)

Instances:
├── .btn-primary.btn-lg    → content: "Register"
├── .btn-primary           → content: "Confirm"
├── .btn-secondary         → content: "Edit"
├── .btn-ghost             → content: "Cancel"
└── .btn-danger.btn-sm     → content: "Remove"
```

### Tags

```
component-group-label: "Status Indicators"
component-group-title: "Tags"

.tag (base)
├── display: inline-flex / align-items: center / gap: var(--space-1)
├── padding: 2px var(--space-2)
├── border: 1px solid
├── font: var(--font-body) 9px weight 600
├── line-height: 1.5
└── border-radius: var(--radius-xs)

Variants:
├── .tag-default  → bg: var(--color-surface-sunken), color: var(--color-text-secondary), border: var(--color-border)
├── .tag-accent   → bg: var(--color-accent-muted), color: var(--color-accent-dark), border: oklch(0.82 0.03 41)
├── .tag-success  → bg: oklch(0.95 0.03 160), color: oklch(0.4 0.1 160), border: oklch(0.88 0.04 160)
├── .tag-warning  → bg: oklch(0.96 0.03 80), color: oklch(0.42 0.1 80), border: oklch(0.9 0.03 80)
└── .tag-danger   → bg: oklch(0.95 0.03 22), color: oklch(0.38 0.12 22), border: oklch(0.9 0.04 22)

Instances:
├── .tag-default  → "Draft"
├── .tag-accent   → "Active"
├── .tag-success  → "On Track"
├── .tag-warning  → "At Risk"
└── .tag-danger   → "Overdue"
```

### Inputs

```
component-group-label: "Data Entry"
component-group-title: "Inputs"

.input-row
├── display: grid
├── grid-template-columns: repeat(auto-fit, minmax(220px, 1fr))
├── gap: var(--space-6)
└── align-items: flex-start

.field
├── display: flex / flex-direction: column / gap: var(--space-2)
│
├── .field-label
│   ├── font: var(--font-display) var(--text-sm) weight 600
│   └── color: var(--color-text)
│
├── .input
│   ├── padding: var(--space-2) var(--space-3)
│   ├── border: 1px solid var(--color-border-strong)
│   ├── font: var(--font-mono) var(--text-sm)
│   ├── color: var(--color-text)
│   ├── background: var(--color-surface)
│   ├── border-radius: var(--radius-xs)
│   ├── transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease
│   ├── ::placeholder → color: var(--color-text-tertiary), font-family: var(--font-mono)
│   ├── :focus → outline: none, border-color: var(--color-accent), box-shadow: 0 0 0 2px var(--color-accent-muted)
│   └── :invalid:not(:placeholder-shown) → border-color: var(--color-danger), box-shadow: 0 0 0 2px oklch(0.95 0.03 22)
│
└── .field-hint
    ├── font: var(--font-mono) 9px
    ├── color: var(--color-text-tertiary)
    └── letter-spacing: 0.02em

Instances:
├── Field 1
│   ├── .field-label: "Student Name"
│   ├── .input: type="text", placeholder="e.g. Jan de Vries"
│   └── id: "host-input"
│
└── Field 2
    ├── .field-label: "Subject Code"
    ├── .input: type="url", placeholder="e.g. WIS-301", required
    ├── .field-hint: "Alphanumeric. No spaces."
    └── id: "port-input"
```

### Toggles

```
component-group-label: "Binary Controls"
component-group-title: "Toggles"

.toggle (base)
├── position: relative
├── width: 40px / height: 20px
├── appearance: none / -webkit-appearance: none
├── background: var(--color-border-strong)
├── border-radius: var(--radius-xs)
├── border: 1px solid var(--color-border)
├── cursor: pointer
├── transition: background var(--duration-fast) ease, border-color var(--duration-fast) ease
├── ::after
│   ├── content: ''
│   ├── position: absolute, top: 2px, left: 2px
│   ├── width: 14px / height: 14px
│   ├── background: var(--color-surface)
│   ├── border-radius: 1px
│   ├── transition: transform var(--duration-base) var(--ease-out-expo)
│   └── box-shadow: 0 1px 2px oklch(0.22 0.005 250 / 0.2)
├── :checked → background: var(--color-accent), border-color: var(--color-accent)
├── :checked::after → transform: translateX(20px)
└── :focus-visible → outline: 2px solid var(--color-accent), outline-offset: 2px

.toggle-label
├── font: var(--font-body) var(--text-sm)
├── color: var(--color-text-secondary)
└── cursor: pointer

Instances:
├── "Notifications"       → checked
├── "Detailed View"       → unchecked
└── "Auto-save"           → checked
```

### Cards

```
component-group-label: "Information Panels"
component-group-title: "Cards"

.card-grid
├── display: grid
├── grid-template-columns: repeat(auto-fit, minmax(280px, 1fr))
├── gap: 1px
└── background: var(--color-border)

.card
├── background: var(--color-surface)
├── padding: var(--space-8)
├── position: relative
├── transition: background var(--duration-fast) ease
└── :hover → background: var(--color-surface-raised)
│
├── .card-id
│   ├── font: var(--font-mono) 9px, letter-spacing: 0.1em
│   ├── color: var(--color-text-tertiary)
│   ├── margin-bottom: var(--space-4)
│   └── ::before → content: '', width: 4px, height: 4px, background: var(--color-accent)
│
├── .card-title
│   ├── font: var(--font-display) var(--text-lg) weight 700
│   ├── letter-spacing: -0.01em
│   ├── color: var(--color-text)
│   └── margin-bottom: var(--space-3)
│
├── .card-body
│   ├── font-size: var(--text-sm)
│   ├── line-height: var(--leading-relaxed)
│   ├── color: var(--color-text-secondary)
│   └── max-width: 45ch
│
└── .card-footer
    ├── margin-top: var(--space-6)
    ├── display: flex / align-items: center / gap: var(--space-3)
    ├── padding-top: var(--space-4)
    └── border-top: 1px dashed var(--color-border)

Instances:
├── Card GL-001
│   ├── .card-id: "GL-001"
│   ├── .card-title: "Consistent Patterns"
│   ├── .card-body: "Every visual value traces to a named token. Teachers and students see the same buttons, colors, and layouts everywhere. Consistency reduces cognitive load and builds trust."
│   └── .card-footer: .tag-accent "Core" + .btn-ghost.btn-sm "Details"
│
├── Card GL-002
│   ├── .card-id: "GL-002"
│   ├── .card-title: "Measured Motion"
│   ├── .card-body: "Movement communicates state change in the interface. When a registration is saved or a report generated, the transition confirms the action. Nothing bounces. Everything arrives."
│   └── .card-footer: .tag-default "Motion" + .btn-ghost.btn-sm "Details"
│
└── Card GL-003
    ├── .card-id: "GL-003"
    ├── .card-title: "Perceptual Accuracy"
    ├── .card-body: "OKLCH ensures colors look equally distinct to all users. Progress indicators and status tags are visually clear. The cold blue undertone gives the interface a calm, focused character."
    └── .card-footer: .tag-default "Color" + .btn-ghost.btn-sm "Details"
```

### Accordion

```
component-group-label: "Collapsible Content"
component-group-title: "Accordion"

.accordion
├── border: 1px solid var(--color-border)
├── background: var(--color-surface)
└── + .accordion → border-top: none

.accordion-trigger
├── width: 100%
├── display: flex / align-items: center / justify-content: space-between
├── padding: var(--space-4) var(--space-6)
├── background: var(--color-surface)
├── border: none / cursor: pointer
├── font: var(--font-display) var(--text-sm) weight 600
├── color: var(--color-text) / text-align: left
├── transition: background var(--duration-fast) ease, color var(--duration-fast) ease
├── :hover → background: var(--color-surface-raised)
├── :focus-visible → outline: 2px solid var(--color-accent), outline-offset: -2px, z-index: 1
│
├── .accordion-trigger-left → display: flex / align-items: center / gap: var(--space-3)
├── .accordion-ref → font: var(--font-mono) 9px, letter-spacing: 0.08em, color: var(--color-accent), min-width: 48px
└── .accordion-icon
    ├── width: 16px / height: 16px
    ├── display: flex / align-items: center / justify-content: center
    ├── transition: transform var(--duration-base) var(--ease-out-expo)
    ├── color: var(--color-text-tertiary)
    ├── content: "▾"
    └── [aria-expanded=true] → transform: rotate(180deg)

[aria-expanded=true] → background: var(--color-surface-raised), border-bottom: 1px solid var(--color-border)

.accordion-panel
├── overflow: hidden
├── max-height: 0
├── transition: max-height var(--duration-slow) var(--ease-out-expo)
└── [aria-hidden=false] → max-height: 500px

.accordion-content
├── padding: var(--space-6)
├── font-size: var(--text-sm)
├── line-height: var(--leading-relaxed)
├── color: var(--color-text-secondary)
├── max-width: 60ch
├── ul → margin-top: var(--space-3), flex-direction: column, gap: var(--space-2)
└── li → display: flex / align-items: baseline / gap: var(--space-2)
         font: var(--font-mono) var(--text-xs), color: var(--color-text-tertiary)
         ::before → content: '—', color: var(--color-border-strong)

Instances:
├── GL-ACC-01: "Assessment Criteria"
│   ├── content: "Standards for evaluating student progress. All assessments must map to curriculum objectives before registration."
│   └── items: ["Scale: 1–10 per learning objective", "Minimum: 3 assessments per term per subject", "Weighting: configurable per step type", "Rounding: one decimal place"]
│
├── GL-ACC-02: "Registration Workflow"
│   ├── content: "Standard procedure for recording student progress. Start with draft entries, review for accuracy, then finalize."
│   └── items: ["Step 1: Select students and subject", "Step 2: Enter grades and observations", "Step 3: Review for completeness", "Step 4: Submit and lock"]
│
└── GL-ACC-03: "Progress Classification"
    ├── content: "Classification taxonomy for student progress states. Every student maps to exactly one status per step."
    └── items: ["Not Started: No registration yet", "In Progress: Partial assessment recorded", "Completed: Full assessment submitted", "Overdue: Past deadline without submission"]
```

### Modal

```
component-group-label: "Overlay Dialogs"
component-group-title: "Modal"

Description: "A focused surface for confirmations, student editing, and detail views. Traps focus. Returns focus on close."
Trigger buttons: .btn-primary "Open Confirm" / .btn-secondary "Open Edit" / .btn-danger "Open Remove"

.modal-backdrop
├── position: fixed / inset: 0
├── background: oklch(0.14 0.01 250 / 0.6)
├── backdrop-filter: blur(4px)
├── z-index: 1000
├── display: flex / align-items: center / justify-content: center
├── opacity: 0 / visibility: hidden
├── transition: opacity var(--duration-slow) var(--ease-out-expo), visibility var(--duration-slow)
└── .is-open → opacity: 1, visibility: visible

.modal
├── background: var(--color-surface)
├── border: 1px solid var(--color-border)
├── box-shadow: var(--shadow-lg)
├── width: 90%
├── max-width: 520px
├── transform: translateY(12px) scale(0.98)
├── transition: transform var(--duration-slow) var(--ease-out-expo)
└── .is-open .modal → transform: translateY(0) scale(1)

.modal-header
├── display: flex / align-items: center / justify-content: space-between
├── padding: var(--space-6)
├── border-bottom: 1px solid var(--color-border)
│
├── .modal-header-left → display: flex / align-items: center / gap: var(--space-3)
├── .modal-header-ref → font: var(--font-mono) 9px, letter-spacing: 0.1em, color: var(--color-accent)
└── .modal-title → font: var(--font-display) var(--text-md) weight 600, letter-spacing: -0.01em, color: var(--color-text)

.modal-close
├── width: 28px / height: 28px
├── display: flex / align-items: center / justify-content: center
├── background: transparent
├── border: 1px solid var(--color-border)
├── border-radius: var(--radius-xs)
├── cursor: pointer
├── color: var(--color-text-tertiary) / font-size: 14px / line-height: 1
├── transition: color var(--duration-fast) ease, border-color var(--duration-fast) ease, background var(--duration-fast) ease
├── :hover → color: var(--color-text), border-color: var(--color-border-strong), background: var(--color-surface-sunken)
└── :focus-visible → outline: 2px solid var(--color-accent), outline-offset: 1px

.modal-body
├── padding: var(--space-6)
└── p → font-size: var(--text-sm), line-height: var(--leading-relaxed), color: var(--color-text-secondary), margin-bottom: var(--space-4)

.modal-footer
├── display: flex / align-items: center / justify-content: flex-end / gap: var(--space-3)
├── padding: var(--space-4) var(--space-6)
└── border-top: 1px dashed var(--color-border)

Instances:
├── GL-MOD-01: "Confirm Registration" (id: modal-confirm)
│   ├── body: "You are about to submit assessments for <strong>Group 7B</strong> in <strong>Mathematics</strong>. This will register grades for 24 students and update their progress records."
│   │         "Estimated processing: <mono-xs>~2 sec</mono-xs>. Edit window: <mono-xs>24 hours</mono-xs>."
│   └── footer: .btn-ghost "Cancel" / .btn-primary "Submit"
│
├── GL-MOD-02: "Edit Student" (id: modal-config)
│   ├── body fields:
│   │   ├── Field "Student Name" → input type="text" placeholder="Jan de Vries"
│   │   ├── Field "Class" → input type="text" placeholder="7B"
│   │   └── Field "Student Number" → input type="number" placeholder="2024001"
│   │       └── .field-hint: "6-digit ID. Unique per student."
│   └── footer: .btn-ghost "Discard" / .btn-primary "Save"
│
└── GL-MOD-03: "Remove Student" (id: modal-danger)
    ├── header: border-bottom-color: oklch(0.9 0.04 22), ref color: var(--color-danger), title color: var(--color-danger)
    ├── body: "This will remove <strong>Jan de Vries</strong> from <strong>Group 7B</strong>. All in-progress registrations will be saved as draft. Submitted grades will not be affected."
    │         "▸ THIS ACTION CANNOT BE UNDONE" → font: var(--font-mono) var(--text-xs), color: var(--color-danger), letter-spacing: 0.04em
    └── footer: .btn-ghost "Cancel" / .btn-danger "Remove"

Behavior:
├── Focus trap on open (Tab/Shift+Tab cycle)
├── Escape key closes
├── Backdrop click closes
├── Returns focus to trigger on close
└── body overflow: hidden while open
```

### Search-ahead Select

```
component-group-label: "Filtered Selection"
component-group-title: "Search-ahead Select"

Description: "Type to filter. Arrow keys to navigate. Enter to select. Backspace on empty input to remove last chip."

.searchahead
├── position: relative
├── width: 100% / max-width: 420px
│
├── .searchahead-input-wrap
│   ├── position: relative / display: flex / align-items: center
│   ├── ::after → border-bottom connector line, opacity 0→1 when .has-dropdown
│   ├── ::before → content: '⌕', position: absolute, left: var(--space-3), font-size: var(--text-sm), color: var(--color-text-tertiary)
│   │
│   ├── .searchahead-input
│   │   ├── width: 100%
│   │   ├── padding: var(--space-2) var(--space-3) var(--space-2) var(--space-8)
│   │   ├── border: 1px solid var(--color-border-strong)
│   │   ├── font: var(--font-mono) var(--text-sm)
│   │   ├── color: var(--color-text) / background: var(--color-surface)
│   │   ├── border-radius: var(--radius-xs)
│   │   ├── ::placeholder → color: var(--color-text-tertiary)
│   │   └── :focus → outline: none, border-color: var(--color-accent), box-shadow: 0 0 0 2px var(--color-accent-muted)
│   │
│   ├── .searchahead-clear
│   │   ├── position: absolute, right: var(--space-2)
│   │   ├── width: 20px / height: 20px
│   │   ├── background: var(--color-surface-sunken)
│   │   ├── border: 1px solid var(--color-border)
│   │   ├── border-radius: var(--radius-xs)
│   │   ├── font: var(--font-mono) 10px, color: var(--color-text-tertiary)
│   │   ├── opacity: 0 / pointer-events: none
│   │   ├── .visible → opacity: 1, pointer-events: auto
│   │   └── :hover → color: var(--color-text)
│   │
│   └── .searchahead-dropdown
│       ├── position: absolute, top: calc(100% + 2px)
│       ├── background: var(--color-surface)
│       ├── border: 1px solid var(--color-border)
│       ├── box-shadow: var(--shadow-md)
│       ├── max-height: 240px / overflow-y: auto
│       ├── opacity: 0 / visibility: hidden / transform: translateY(-4px)
│       ├── transition: opacity var(--duration-fast), visibility var(--duration-fast), transform var(--duration-fast) var(--ease-out-expo)
│       ├── z-index: 50
│       ├── .is-open → opacity: 1, visibility: visible, transform: translateY(0)
│       │
│       ├── .searchahead-dropdown-header
│       │   ├── position: sticky, top: 0
│       │   ├── background: var(--color-surface-raised)
│       │   ├── padding: var(--space-2) var(--space-3)
│       │   ├── font: var(--font-mono) 9px, letter-spacing: 0.1em, text-transform: uppercase
│       │   ├── color: var(--color-text-tertiary)
│       │   └── border-bottom: 1px solid var(--color-border)
│       │
│       ├── .searchahead-option
│       │   ├── display: flex / align-items: center / justify-content: space-between
│       │   ├── padding: var(--space-2) var(--space-3)
│       │   ├── cursor: pointer
│       │   ├── transition: background var(--duration-fast) ease
│       │   ├── :hover / .is-highlighted → background: var(--color-accent-muted)
│       │   ├── .searchahead-option-left → display: flex / align-items: center / gap: var(--space-3)
│       │   ├── .searchahead-option-icon → font: var(--font-mono) 9px, color: var(--color-accent), width: 16px, text-align: center
│       │   ├── .searchahead-option-text → font: var(--font-mono) var(--text-sm), color: var(--color-text)
│       │   │   └── mark → background: none, color: var(--color-accent), font-weight: 700
│       │   └── .searchahead-option-meta → font: var(--font-mono) 9px, color: var(--color-text-tertiary), letter-spacing: 0.02em
│       │
│       └── .searchahead-empty
│           ├── padding: var(--space-6) var(--space-3), text-align: center
│           ├── font: var(--font-mono) var(--text-xs), color: var(--color-text-tertiary), letter-spacing: 0.04em
│           └── ::before → content: '// ', color: var(--color-border-strong)
│
└── .searchahead-selection
    ├── margin-top: var(--space-4)
    ├── display: flex / flex-wrap: wrap / gap: var(--space-2) / min-height: 24px
    │
    └── .searchahead-chip
        ├── display: inline-flex / align-items: center / gap: var(--space-1)
        ├── padding: 2px var(--space-2)
        ├── background: var(--color-accent-muted)
        ├── border: 1px solid oklch(0.82 0.03 41)
        ├── border-radius: var(--radius-xs)
        ├── font: var(--font-mono) 9px, letter-spacing: 0.06em, text-transform: uppercase
        ├── color: var(--color-accent-dark)
        ├── cursor: pointer
        ├── :hover → background: oklch(0.88 0.03 41)
        └── ::after → content: '×', font-size: 11px, margin-left: var(--space-1), color: var(--color-accent-light)

Data (saData):
├── { id: 'stu-01', label: 'Emma Bakker',     meta: 'Group 7B',    icon: '◉' }
├── { id: 'stu-02', label: 'Jan de Vries',    meta: 'Group 7B',    icon: '◉' }
├── { id: 'stu-03', label: 'Sophie Jansen',   meta: 'Group 7A',    icon: '◉' }
├── { id: 'stu-04', label: 'Lucas Visser',    meta: 'Group 8A',    icon: '◉' }
├── { id: 'sub-wis', label: 'Wiskunde',       meta: 'WIS-301',     icon: '◆' }
├── { id: 'sub-tal', label: 'Taal',           meta: 'TAL-201',     icon: '◆' }
├── { id: 'sub-lez', label: 'Lezen',          meta: 'LEZ-102',     icon: '◆' }
├── { id: 'sub-snk', label: 'Snappet',        meta: 'SNK-401',     icon: '◆' }
├── { id: 'grp-7a',  label: 'Group 7A',       meta: '28 students', icon: '▨' }
├── { id: 'grp-7b',  label: 'Group 7B',       meta: '24 students', icon: '▨' }
├── { id: 'grp-8a',  label: 'Group 8A',       meta: '26 students', icon: '▨' }
├── { id: 'grp-8b',  label: 'Group 8B',       meta: '22 students', icon: '▨' }
├── { id: 'tea-01',  label: 'Marie Kosters',  meta: 'Teacher',     icon: '◈' }
└── { id: 'tea-02',  label: 'Peter Hendriks', meta: 'Teacher',     icon: '◈' }
```

### Toast

```
component-group-label: "Notification System"
component-group-title: "Toast"

Description: "Ephemeral notifications that slide in, persist briefly, then dismiss. Stacks vertically. No interruption to workflow."
Trigger buttons:
├── .btn-primary "Notify"  → data-toast="default"
├── .btn-secondary (border/success) "Success"  → data-toast="success"
├── .btn-secondary (border/warning) "Warning"  → data-toast="warning"
└── .btn-secondary (border/danger)  "Error"    → data-toast="danger"

.toast-container
├── position: fixed
├── bottom: var(--space-6) / right: var(--space-6)
├── display: flex / flex-direction: column-reverse / gap: var(--space-2)
├── z-index: 1100
└── pointer-events: none

.toast
├── display: flex / align-items: flex-start / gap: var(--space-3)
├── padding: var(--space-3) var(--space-4)
├── background: var(--color-surface)
├── border: 1px solid var(--color-border)
├── box-shadow: var(--shadow-md)
├── min-width: 300px / max-width: 420px
├── pointer-events: auto
├── transform: translateX(110%) / opacity: 0
├── transition: transform var(--duration-slow) var(--ease-out-expo), opacity var(--duration-slow) var(--ease-out-expo)
├── position: relative / overflow: hidden
├── .is-visible → transform: translateX(0), opacity: 1
└── .is-dismissing → transform: translateX(110%), opacity: 0

.toast-indicator
├── width: 3px
├── align-self: stretch / flex-shrink: 0
└── colors by variant:
    ├── .toast-default → background: var(--color-accent)
    ├── .toast-success → background: var(--color-success)
    ├── .toast-warning → background: var(--color-warning)
    └── .toast-danger  → background: var(--color-danger)

.toast-body
├── flex: 1
├── display: flex / flex-direction: column / gap: 2px
├── .toast-title → font: var(--font-body) var(--text-xs) weight 600, color: var(--color-text)
└── .toast-message → font-size: var(--text-sm), line-height: var(--leading-normal), color: var(--color-text-secondary)

.toast-close
├── width: 20px / height: 20px
├── display: flex / align-items: center / justify-content: center
├── background: transparent / border: none / cursor: pointer
├── color: var(--color-text-tertiary) / font-size: 12px / line-height: 1
├── flex-shrink: 0
└── :hover → color: var(--color-text)

.toast-progress
├── position: absolute / bottom: 0 / left: 0
├── height: 2px
├── background: var(--color-accent)
├── transform-origin: left
├── transition: transform linear
└── colors by variant:
    ├── .toast-success → background: var(--color-success)
    ├── .toast-warning → background: var(--color-warning)
    └── .toast-danger  → background: var(--color-danger)

Presets:
├── default → title: "Notification", message: "Progress update saved successfully.", duration: 4500ms
├── success → title: "Registration Complete", message: "Assessments for Group 7B in Mathematics registered for 24 students.", duration: 4500ms
├── warning → title: "Incomplete Data", message: "3 students in Group 7A are missing assessments for Wiskunde this term.", duration: 5500ms
└── danger  → title: "Deadline Approaching", message: "Report deadline is tomorrow. 12 students still have pending registrations.", duration: 6000ms
```

---

## Section 05 — Motion (`#motion`)

```
.section-number: "05"
.section-title: "Motion"

.motion-demos
├── display: grid
├── grid-template-columns: repeat(auto-fit, minmax(240px, 1fr))
├── gap: 1px
├── background: var(--color-border)

.motion-demo
├── background: var(--color-surface)
├── padding: var(--space-8)
├── display: flex / flex-direction: column / gap: var(--space-4)
│
├── .motion-demo-header → display: flex / justify-content: space-between / align-items: baseline
├── .motion-demo-label → font: var(--font-mono) 9px, letter-spacing: 0.1em, text-transform: uppercase, color: var(--color-text-tertiary)
├── .motion-demo-name → font: var(--font-display) var(--text-md) weight 700, color: var(--color-text)
│
├── .motion-stage
│   ├── height: 40px / display: flex / align-items: center
│   ├── position: relative / overflow: hidden
│   ├── background: var(--color-surface-sunken)
│   ├── cursor: pointer
│   └── .motion-block
│       ├── width: 24px / height: 24px
│       ├── background: var(--color-accent)
│       ├── position: absolute / left: 8px
│       └── transition-property: transform
│           :hover / .active → transform: translateX(calc(100% * 5))
│
└── .motion-curve → font: var(--font-mono) 10px, color: var(--color-text-tertiary), letter-spacing: 0.02em

Easing Instances:
├── Out Expo
│   ├── .motion-demo-label: "Easing"
│   ├── .motion-demo-name: "Out Expo"
│   ├── .motion-block: transition-duration: 700ms, transition-timing-function: var(--ease-out-expo)
│   └── .motion-curve: "cubic-bezier(0.16, 1, 0.3, 1)"
│
├── Out Quart
│   ├── .motion-demo-label: "Easing"
│   ├── .motion-demo-name: "Out Quart"
│   ├── .motion-block: transition-duration: 500ms, transition-timing-function: var(--ease-out-quart)
│   └── .motion-curve: "cubic-bezier(0.25, 1, 0.5, 1)"
│
└── Industrial
    ├── .motion-demo-label: "Easing"
    ├── .motion-demo-name: "Industrial"
    ├── .motion-block: transition-duration: 600ms, transition-timing-function: var(--ease-industrial)
    └── .motion-curve: "cubic-bezier(0.22, 0.61, 0.36, 1)"

Duration Reference Grid:
.duration-grid
├── display: grid
├── grid-template-columns: repeat(auto-fit, minmax(200px, 1fr))
├── gap: 1px
├── background: var(--color-border)
├── margin-top: var(--space-8)

.duration-item
├── background: var(--color-surface)
├── padding: var(--space-6)
├── display: flex / flex-direction: column / gap: var(--space-2)
├── .duration-token → font: var(--font-mono) 10px weight 700, letter-spacing: 0.06em, color: var(--color-accent)
├── .duration-value → font: var(--font-mono) 10px, color: var(--color-text-tertiary)
└── .duration-usage → font-size: var(--text-sm), color: var(--color-text-secondary), line-height: var(--leading-normal)

Durations:
├── --duration-fast  → 120ms → "Hover feedback, focus rings, micro state"
├── --duration-base  → 250ms → "Toggles, inline transitions, panel swaps"
├── --duration-slow  → 450ms → "Page regions, expand/collapse, modals"
└── --duration-enter → 600ms → "Page loads, stagger reveals, hero entrance"

Stagger Reveal Demo:
.stagger-demo
├── margin-top: var(--space-12)
├── component-group-label: "Interaction Demo"
├── component-group-title: "Stagger Reveal"
├── description: "Click the grid to trigger a staggered fill. Elements fill sequentially, creating rhythm without chaos — like indicators lighting up on a control panel."
│
└── .stagger-grid
    ├── display: grid
    ├── grid-template-columns: repeat(6, 1fr)
    ├── gap: 2px
    ├── margin-top: var(--space-4)
    ├── role: button / tabindex: 0
    │
    └── .stagger-cell (18 total, 6×3)
        ├── aspect-ratio: 2
        ├── background: var(--color-surface-sunken)
        ├── position: relative / overflow: hidden / cursor: pointer
        └── ::after
            ├── content: ''
            ├── position: absolute / inset: 0
            ├── background: var(--color-accent)
            ├── transform: scaleX(0) / transform-origin: left
            ├── transition: transform var(--duration-base) var(--ease-out-expo)
            └── .visible → transform: scaleX(1)

Behavior:
├── Click triggers staggered fill with 50ms delay per cell
├── Auto-triggers 800ms after page load
└── Keyboard: Enter/Space triggers
```

---

## Section 06 — Design Principles (`#principles`)

```
.section-number: "06"
.section-title: "Design Principles"

.principles-grid
├── display: grid
├── grid-template-columns: repeat(auto-fit, minmax(280px, 1fr))
├── gap: 1px
├── background: var(--color-border)

.principle
├── background: var(--color-surface)
├── padding: var(--space-8)
│
├── .principle-ref
│   ├── font: var(--font-mono) 10px
│   ├── letter-spacing: 0.1em
│   ├── color: var(--color-accent)
│   └── margin-bottom: var(--space-4)
│
├── .principle-title
│   ├── font: var(--font-display) var(--text-md) weight 700
│   ├── letter-spacing: -0.01em
│   ├── color: var(--color-text)
│   └── margin-bottom: var(--space-3)
│
└── .principle-desc
    ├── font-size: var(--text-sm)
    ├── line-height: var(--leading-relaxed)
    ├── color: var(--color-text-secondary)
    └── max-width: 38ch

Principles:
├── GL-P01: "Structure enables focus"
│   └── "A consistent interface lets teachers focus on teaching and students on learning. When every pattern is predictable — buttons, colors, layouts — cognitive load drops and attention stays on what matters: student progress."
│
├── GL-P02: "Clarity is care"
│   └── "Teachers work under time pressure with many students. Ambiguous labels, unclear states, and visual noise aren't minor flaws — they steal time from the people who need it most. Precision in the interface is respect for the educator."
│
├── GL-P03: "Motion conveys progress"
│   └── "Every transition answers a question: what just changed? When a registration is saved or a report generated, the motion confirms the action. Industrial easing. No bounce. No decoration. Progress feels deliberate."
│
└── GL-P04: "Tinted neutrality"
    └── "Pure gray is a missed opportunity. Neutrals tinted toward the system's hue create cohesion below conscious perception. The cold blue undertone gives the interface a calm, focused character suited for long sessions of data entry and review."
```

---

## Footer

```
.footer
├── max-width: var(--max-width)
├── margin: 0 auto
├── padding: var(--space-12) var(--gutter) var(--space-8)
├── display: flex / justify-content: space-between / align-items: center / flex-wrap: wrap
├── gap: var(--space-4)
├── border-top: 1px solid var(--color-border)
│
├── .footer-brand
│   ├── font: var(--font-display) var(--text-md) weight 600
│   ├── letter-spacing: -0.02em
│   ├── color: var(--color-text-tertiary)
│   └── content: "LRS Design System"
│
├── .footer-meta
│   ├── font: var(--font-mono) 9px
│   ├── letter-spacing: 0.08em
│   ├── color: var(--color-text-tertiary)
│   ├── display: flex / gap: var(--space-6)
│   └── content: ["SEED: DS2-GLACIER-7742", "v2.0", "OKLCH"]
│
└── .footer-links
    ├── display: flex / gap: var(--space-4) / list-style: none
    ├── a → font: var(--font-body) var(--text-sm), color: var(--color-text-tertiary), text-decoration: none
    ├── a:hover → color: var(--color-accent)
    └── content: ["Colors", "Type", "Components"]
```

---

## Dividers

```
.divider
├── border: none
├── height: 1px
├── background: var(--color-border)
├── max-width: var(--max-width)
└── margin: 0 auto

.divider-dashed
├── border: none
├── height: 0
├── border-top: 1px dashed var(--color-border)
├── max-width: var(--max-width)
└── margin: 0 auto
```

---

## Responsive Breakpoints (`@media max-width: 640px`)

```
├── .nav-links → display: none
├── .nav-seed → display: none
├── .hero → padding-top: var(--space-16)
├── .hero-specs → grid-template-columns: repeat(2, 1fr)
├── .type-row → grid-template-columns: 1fr, gap: var(--space-2)
├── .type-meta → flex-direction: row, gap: var(--space-3)
├── .spacing-label → min-width: 56px, font-size: 9px
├── .spacing-rem → display: none
├── .font-pair-grid → grid-template-columns: 1fr
├── .stagger-grid → grid-template-columns: repeat(3, 1fr)
├── .principles-grid → grid-template-columns: 1fr
├── .footer → flex-direction: column, align-items: flex-start
├── .footer-meta → flex-direction: column, gap: var(--space-2)
├── .toast-container → left: var(--space-4), right: var(--space-4), bottom: var(--space-4)
├── .toast → min-width: 0, max-width: 100%
└── .searchahead → max-width: 100%
```

---

\_Extracted from design-system.html — DS2-GLACIER-7742 — LRS Edition
