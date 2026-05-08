# Chart Library Candidates — Research Report

**Project:** data.monster  
**Stack:** Svelte 5 (runes) · SvelteKit · Tauri v2 · DuckDB (Rust) · Tailwind CSS v4  
**Date:** 2025-05-14  
**Goal:** Select a chart library for an interactive desktop analytics/visualization app

---

## Requirements Summary

| # | Requirement | Priority |
|---|-------------|----------|
| 1 | Svelte wrapper or easily wrappable | Must |
| 2 | Click events on chart elements (bars, segments, points) for interactive filtering | Must |
| 3 | Renders in Tauri WebView2 (Windows) without issues | Must |
| 4 | Chart types: bar, line, area, scatter, heatmap, pie/donut, treemap | Must |
| 5 | Large datasets (thousands of rows) without lag | High |
| 6 | Lightweight bundle | High |
| 7 | Svelte 5 runes API compatibility (`$props`, `$state`, `$derived`, `$effect`) | High |
| 8 | Deep customization (axes, legends, tooltips, colors, labels) | High |

---

## Existing Prototypes (In-Use)

The app has draft implementations with three libraries already. These are evaluated alongside new candidates below.

---

## Candidate 1: `@carbon/charts-svelte`

**npm:** [`@carbon/charts-svelte`](https://www.npmjs.com/package/@carbon/charts-svelte) v1.22.18  
**GitHub:** [carbon-design-system/carbon-charts](https://github.com/carbon-design-system/carbon-charts) — ⭐ 1,000  
**License:** Apache-2.0  
**Last Published:** May 9, 2025 (5 days ago — actively maintained)

### Overview

IBM's Carbon Charts — a D3-based charting library with official wrappers for Svelte, React, Vue, and Angular. Provides 26 pre-built chart types with Carbon Design System styling. The Svelte wrapper is maintained by community members (@nstuyvesant, @metonym).

### Bundle Size

~250–350 KB minified (core `@carbon/charts` + D3 dependencies + Svelte wrapper). Heavy for a desktop analytics tool. The Carbon design tokens and theme CSS add additional weight.

### Svelte 5 Compatibility

⚠️ **Partial.** The README states:

> "A release in the near future will move components to runes mode for Svelte 5 and beyond. As this would be a breaking change because event handlers will change to component properties, this version will be published with the distribution tag `next`."

Current version uses Svelte 4 patterns (`on:event` dispatchers, `bind:chart`). The runes migration is planned but not shipped. Using it in a Svelte 5 runes project works but requires mixing Svelte 4 event patterns.

### Interactive Event Handling

✅ **Supported.** Two mechanisms:

1. **Component dispatch events:** `on:load`, `on:update`, `on:destroy`
2. **Chart service events:** Access via `bind:chart` then `chart.services.events.addEventListener('bar-mouseover', handler)`. Supports `bar-mouseover`, `bar-click`, `bar-mouseout`, and similar events for other chart types.

Click-to-filter is achievable but requires wiring through the imperative event API rather than declarative Svelte patterns.

### Chart Types Supported

✅ 26 types: bar (simple, grouped, stacked, floating), line, area, scatter, pie, donut, radar, gauge, treemap, heatmap, wordcloud, histogram, alluvial, boxplot, lollipop, bubble, meter, bullet, sparkline, combo charts, and more.

### Customization Depth

**Moderate.** Configurable via an `options` prop object (axes, title, legend, theme, grid, etc.). The Carbon Design System aesthetic is baked in — deviating significantly from IBM's visual language requires effort. Theming supports `white`, `g90`, `g100` (dark modes). Custom colors require CSS overrides or custom theme objects.

### Performance with Large Datasets

⚠️ **Moderate.** SVG-based rendering (via D3). Struggles with >5,000–10,000 data points due to DOM node proliferation. No Canvas rendering fallback. For analytics with DuckDB returning aggregated results, this is usually fine, but raw row-level scatter plots can lag.

### Tauri/WebView2 Issues

No known WebView2-specific issues. SVG rendering works well in WebView2. IBM Telemetry collection is a concern for a desktop app (can be opted out, but requires configuration).

### Known Issues

- **IBM Telemetry** included by default — must opt out for privacy-sensitive desktop apps
- **Heavy D3 dependency chain** — many transitive dependencies
- **Svelte 5 runes migration not yet shipped** — uses legacy `on:` event syntax
- **Carbon Design System aesthetic** — overwriting IBM's visual language for a custom analytics tool is extra work
- **SSR configuration** requires `noExternal` Vite config for `@carbon/charts`

### Verdict

| Criteria | Rating |
|----------|--------|
| Svelte wrapper quality | ⭐⭐⭐ (official but Svelte 4 patterns) |
| Event handling | ⭐⭐⭐ (imperative API, works) |
| Chart variety | ⭐⭐⭐⭐⭐ (26 types) |
| Customization | ⭐⭐⭐ (decent, but Carbon-themed) |
| Performance | ⭐⭐⭐ (SVG-only, moderate) |
| Bundle size | ⭐⭐ (heavy) |
| Svelte 5 ready | ⭐⭐ (migration pending) |

**Recommendation:** ⚠️ **Consider replacing.** Heavy, IBM-oriented, and not yet runes-native. Good chart variety but the Carbon design system overhead doesn't align with a lightweight analytics tool.

---

## Candidate 2: `svelteplot`

**npm:** [`svelteplot`](https://www.npmjs.com/package/svelteplot) v0.14.2  
**GitHub:** [svelteplot/svelteplot](https://github.com/svelteplot/svelteplot) — ⭐ 387  
**License:** ISC  
**Created by:** Gregor Aisch (data visualization expert, former NYT Graphics)  
**Last Published:** May 3, 2025 (11 days ago)

### Overview

A Svelte-native visualization framework built on the layered grammar of graphics principles. API heavily inspired by Observable Plot. Components are native Svelte — marks like `<Line>`, `<Bar>`, `<AreaX>`, `<Dot>` compose together inside a `<Plot>` container.

### Bundle Size

~60–80 KB minified (core svelteplot + D3 dependencies). Moderate — D3 is still in the dependency tree but it's selective (18 deps total, mostly d3 sub-modules). Significantly lighter than Carbon Charts.

### Svelte 5 Compatibility

✅ **Good.** The project was built with modern Svelte in mind. Uses Svelte component patterns throughout. As of recent versions, it targets Svelte 5 compatibility. The API uses Svelte props and reactive patterns naturally. Still pre-1.0, so some API churn expected.

### Interactive Event Handling

⚠️ **Limited but workable.** The grammar-of-graphics approach means marks are Svelte components. You can attach standard Svelte event handlers (`onclick`, `on:pointerdown`) to mark elements. However, the interaction model is less polished than dedicated interaction systems. Click-to-filter requires manual wiring:

```svelte
<Plot>
  <BarX
    data={filteredData}
    onclick={(e) => {
      selectedCategory = e.detail?.group;
    }}
  />
</Plot>
```

The granularity of click targets depends on the mark type — bars and points work well, but continuous marks (lines, areas) require custom hit detection.

### Chart Types Supported

✅ Most required types via composable marks:
- Bar (`BarX`, `BarY`)
- Line (`Line`, `LineX`, `LineY`)
- Area (`AreaX`, `AreaY`)
- Scatter/Dot (`Dot`, `DotX`, `DotY`)
- Cell/Heatmap (`Cell`)
- Pie/Donut (via `Arc` marks — less polished than dedicated pie components)
- Treemap — via `Rect` marks with hierarchical layout (manual)

Missing: First-class treemap and dedicated heatmap components. These require more manual composition.

### Customization Depth

**High.** Since it's a grammar of graphics, you compose marks, scales, axes, and facets. Full control over colors, scales, axes, labels. The Svelte-native approach means you can interleave any Svelte markup inside chart layers (custom tooltips, annotations, interactive overlays).

### Performance with Large Datasets

⚠️ **Moderate.** SVG-based rendering via D3 scales and generators. Handles ~5,000–10,000 points well for bar/line charts. Scatter plots with >10K points will feel sluggish. No built-in Canvas rendering mode (though one could be built since SveltePlot supports `Canvas` layers).

### Tauri/WebView2 Issues

No known issues. SVG rendering in WebView2 is solid. The Svelte-native approach means no Canvas/WebGL dependencies that might conflict with WebView2.

### Known Issues

- **Pre-1.0** — API may change between minor versions
- **Treemap/Heatmap** not first-class — require manual composition
- **Interaction model** is basic compared to Carbon Charts or ECharts
- **Pie/Donut** charts are possible but not as polished as dedicated implementations
- **Documentation** is growing but still thin compared to mature libraries
- **Small community** — 387 stars, single primary maintainer (though Gregor Aisch is experienced)

### Verdict

| Criteria | Rating |
|----------|--------|
| Svelte wrapper quality | ⭐⭐⭐⭐⭐ (native Svelte) |
| Event handling | ⭐⭐⭐ (Svelte events, manual wiring) |
| Chart variety | ⭐⭐⭐ (core types yes, treemap/heatmap weak) |
| Customization | ⭐⭐⭐⭐⭐ (grammar of graphics) |
| Performance | ⭐⭐⭐ (SVG, moderate) |
| Bundle size | ⭐⭐⭐⭐ (moderate) |
| Svelte 5 ready | ⭐⭐⭐⭐ (good, modern Svelte) |

**Recommendation:** ✅ **Strong contender for composition-heavy charts.** Best Svelte-native experience, but weak on treemap/heatmap and interaction polish. Keep for declarative/analytical charts where customization matters.

---

## Candidate 3: `picasso.js`

**npm:** [`picasso.js`](https://www.npmjs.com/package/picasso.js) v2.11.3  
**GitHub:** [qlik-oss/picasso.js](https://github.com/qlik-oss/picasso.js) — ⭐ 190  
**License:** MIT  
**Maintained by:** Qlik (analytics platform company)  
**Last Published:** May 13, 2025 (1 day ago)

### Overview

A charting library built for Qlik's analytics platform. Framework-agnostic imperative API — you call `picasso.chart({ element, settings })` and get a chart instance. Renders to SVG and Canvas. Zero runtime dependencies (impressive!).

### Bundle Size

~90–120 KB minified, **zero dependencies**. This is remarkably lean for the feature set. The zero-dep approach means no D3 — everything is custom-built.

### Svelte 5 Compatibility

⚠️ **Requires wrapping.** Picasso.js is imperative and framework-agnostic. You'd need to build a Svelte wrapper:

```svelte
<script>
  import picasso from 'picasso.js';
  import { onMount } from 'svelte';

  let chartEl;
  let chart;

  $effect(() => {
    if (chartEl) {
      chart = picasso.chart({
        element: chartEl,
        data: [{ type: 'matrix', data: myData }],
        settings: chartSettings
      });
    }
    return () => chart?.destroy();
  });
</script>

<div bind:this={chartEl}></div>
```

This is straightforward but requires maintenance of the wrapper layer.

### Interactive Event Handling

✅ **Strong.** Picasso.js has a built-in interaction/brushing system designed for Qlik's associative analytics engine:

```js
chart.on('tap', (event) => {
  console.log(event); // Contains data about the tapped element
});
```

Supports tap, hover, brush (range selection), and custom interactions. The brushing system is specifically designed for the click-to-filter analytics pattern. This is one of its strongest features for an analytics app.

### Chart Types Supported

✅ Most required types:
- Bar (grouped, stacked)
- Line
- Area
- Scatter
- Pie/Donut
- Heatmap (via `rect` component + color scale)
- Treemap (via dedicated `treemap` component type)

26 component types including axes, legends, labels, ref-lines, and more.

### Customization Depth

**Very High.** The settings-driven API provides granular control over every visual aspect: scales, components, docking, styling, interactions. Since there's no framework opinion on styling, you can match any design system. Supports both SVG and Canvas rendering per-component.

### Performance with Large Datasets

✅ **Strong.** Canvas rendering mode handles 50,000+ data points smoothly. The zero-dependency core is optimized for performance. Qlik's use case (enterprise analytics with millions of rows) means performance was a primary design goal.

### Tauri/WebView2 Issues

No known issues. Both SVG and Canvas modes work in WebView2. The zero-dependency approach minimizes compatibility surface area.

### Known Issues

- **No Svelte wrapper** — must build and maintain your own
- **Imperative API** — doesn't feel "Svelte-native"; requires `$effect` and `onMount` wrapping
- **Small community** — 190 GitHub stars, primarily Qlik-internal usage
- **Documentation** is good but Qlik-oriented (qlik.dev)
- **Verbose settings** — the configuration objects can be deeply nested
- **Designed for Qlik's data model** — some patterns assume Qlik's hypercube data format (but works with plain arrays)

### Verdict

| Criteria | Rating |
|----------|--------|
| Svelte wrapper quality | ⭐⭐ (manual wrapping required) |
| Event handling | ⭐⭐⭐⭐⭐ (built-in brushing/interaction system) |
| Chart variety | ⭐⭐⭐⭐ (all required types) |
| Customization | ⭐⭐⭐⭐⭐ (settings-driven, full control) |
| Performance | ⭐⭐⭐⭐⭐ (Canvas mode, zero deps) |
| Bundle size | ⭐⭐⭐⭐⭐ (zero deps, ~100 KB) |
| Svelte 5 ready | ⭐⭐⭐ (wrappable via $effect) |

**Recommendation:** ✅ **Strong for performance-critical analytics charts.** The brushing system and Canvas rendering are perfect for an analytics tool. Requires investment in a Svelte wrapper but the performance payoff is significant.

---

## Candidate 4: `layerchart` 🆕

**npm:** [`layerchart`](https://www.npmjs.com/package/layerchart) v1.0.13  
**GitHub:** [techniq/layerchart](https://github.com/techniq/layerchart) — ⭐ 1,200  
**License:** MIT  
**Maintained by:** techniq (Sean Lynch)  
**Last Published:** May 7, 2025 (7 days ago)

### Overview

A large collection of **composable Svelte chart components** — the closest thing to a "full-featured chart library built for Svelte." Provides ready-to-use components for a wide range of visualization types. Companion library to Svelte UX (another library by the same author for interactive app components).

### Bundle Size

~80–120 KB minified (depends on D3 modules — 27 dependencies listed, though many are small d3 sub-packages). Tree-shakeable since it's component-based — you only import what you use.

### Svelte 5 Compatibility

✅ **Excellent.** Built for modern Svelte. The component API uses Svelte's prop-based patterns. The author (techniq) is an active Svelte community member and maintains Svelte UX alongside LayerChart. The component patterns align with Svelte 5's approach.

### Interactive Event Handling

✅ **Strong.** Built-in support for:
- **Tooltip** — hover interactions with data display
- **Highlights** — hover/click highlighting of data elements
- **Pan/Zoom** — interactive navigation of chart space
- **Click events** — standard Svelte event handling on components

Since components are native Svelte, you can use `onclick`, `on:pointerdown`, etc. on chart elements. The interaction model is well-designed for analytics workflows.

### Chart Types Supported

✅ **Comprehensive:**
- **Cartesian:** Bar, Area, Stack, Scatter
- **Radial:** Pie, Arc, Sunburst
- **Hierarchy:** Pack, Tree, Treemap, Sunburst
- **Graph:** Sankey
- **Geo:** Choropleth, Spike, Bubble, Point, Globe
- Plus: SVG primitives (Arc, Circle, Group, Line, Spline, Text), gradients, patterns, legends, color ramps

Covers all required types (bar, line, area, scatter, heatmap, pie/donut, treemap) and more.

### Customization Depth

**Very High.** Composable component architecture means you:
- Use individual mark components inside container components
- Mix and match layers
- Override any visual property via props
- Interleave custom Svelte markup between chart layers
- Style with Tailwind (compatible — SVG elements accept class props)
- Full control over axes, scales, legends, tooltips

### Performance with Large Datasets

⚠️ **Moderate.** SVG-based rendering (like most Svelte-native libraries). Handles thousands of data points well for bar/line/area charts. Scatter plots with >10K points may need optimization. No built-in Canvas mode. For DuckDB-aggregated results (typically hundreds to low thousands of rows), this should be fine.

### Tauri/WebView2 Issues

No known issues. SVG rendering in WebView2 is well-supported. The Svelte-native approach means no exotic rendering APIs.

### Known Issues

- **27 dependencies** — mostly d3 sub-packages; heavier dependency tree than ideal
- **Relatively young** — v1.0.13, API may still evolve
- **SVG-only** — no Canvas rendering fallback for massive datasets
- **Weekly downloads high (82K)** but much of that may be transitive from Svelte UX
- **Documentation** is primarily examples-based (GitHub + Storybook) rather than comprehensive API docs

### Verdict

| Criteria | Rating |
|----------|--------|
| Svelte wrapper quality | ⭐⭐⭐⭐⭐ (native Svelte components) |
| Event handling | ⭐⭐⭐⭐ (tooltips, highlights, Svelte events) |
| Chart variety | ⭐⭐⭐⭐⭐ (all required types + geo + hierarchy) |
| Customization | ⭐⭐⭐⭐⭐ (composable, Tailwind-friendly) |
| Performance | ⭐⭐⭐ (SVG-only, moderate) |
| Bundle size | ⭐⭐⭐⭐ (tree-shakeable) |
| Svelte 5 ready | ⭐⭐⭐⭐⭐ (built for modern Svelte) |

**Recommendation:** ✅ **Top recommendation.** Best overall fit for the stack. Native Svelte components, comprehensive chart types, built-in interactions, active maintenance, and Tailwind-compatible. The one weakness (SVG performance) is mitigated by DuckDB aggregation producing manageable result sets.

---

## Candidate 5: `layercake`

**npm:** [`layercake`](https://www.npmjs.com/package/layercake) v10.0.2  
**GitHub:** [mhkeller/layercake](https://github.com/mhkeller/layercake) — ⭐ 1,800  
**License:** MIT  
**Last Published:** October 2024 (7 months ago)

### Overview

A **headless** visualization framework for Svelte. Provides the scaffolding (scales, layout, data mapping) but no pre-built chart components. You compose layers of SVG, Canvas, or HTML inside a `<LayerCake>` wrapper. Think of it as a lower-level cousin to LayerChart — more control, more work.

### Bundle Size

~15–25 KB minified (4 dependencies). Very lightweight — the smallest candidate by far. Does not include D3 by default (you bring your own scale/shape functions).

### Svelte 5 Compatibility

✅ **Supported.** README states "Works with version 5." v10.0 is the Svelte 5-compatible line. All examples use Rune syntax.

### Interactive Event Handling

⚠️ **DIY.** Since LayerCake is headless, you add event handlers to your own SVG/Canvas elements. This gives complete control but requires manual implementation:

```svelte
<LayerCake {data} x="x" y="y">
  <Svg>
    {#each data as d}
      <circle
        cx={$xGet(d)}
        cy={$yGet(d)}
        r={5}
        onclick={() => handleFilter(d)}
      />
    {/each}
  </Svg>
</LayerCake>
```

Click-to-filter is straightforward for bar/point charts but requires custom code for line/area hit detection.

### Chart Types Supported

⚠️ **None built-in.** You build everything from scratch using the layer primitives. The example gallery shows bar, line, area, scatter, and more — but these are example implementations you copy into your project, not imported components.

Can build: bar, line, area, scatter, heatmap, pie/donut (manual arc math), treemap (manual layout). Heatmap and treemap require significant custom work.

### Customization Depth

**Maximum.** You control every pixel. No opinions on styling, colors, layout beyond the coordinate system mapping.

### Performance with Large Datasets

✅ **Good to Excellent.** Supports SVG, Canvas, and HTML layers simultaneously. For large datasets, you can use Canvas layers for the data marks and SVG layers for axes/labels. This is a significant advantage.

### Tauri/WebView2 Issues

No known issues. WebView2 handles all three rendering modes (SVG, Canvas, HTML).

### Known Issues

- **Headless = more work** — no pre-built chart components; every chart type requires implementation
- **Not a chart library** — it's a framework for building chart libraries
- **Last publish 7 months ago** — slower release cadence than other candidates
- **No built-in interactions** — tooltip, highlight, zoom must all be custom-built
- **Heatmap/Treemap** require significant effort
- **Learning curve** — grammar of graphics concepts require study

### Verdict

| Criteria | Rating |
|----------|--------|
| Svelte wrapper quality | ⭐⭐⭐⭐⭐ (native Svelte, headless) |
| Event handling | ⭐⭐ (DIY — full control, full effort) |
| Chart variety | ⭐⭐ (none built-in, all possible) |
| Customization | ⭐⭐⭐⭐⭐ (maximum) |
| Performance | ⭐⭐⭐⭐⭐ (Canvas support built-in) |
| Bundle size | ⭐⭐⭐⭐⭐ (~20 KB, 4 deps) |
| Svelte 5 ready | ⭐⭐⭐⭐ (explicit Svelte 5 support) |

**Recommendation:** ⚠️ **Good as a foundation, not as a primary library.** Could pair with LayerChart — use LayerCake's Canvas layers for high-volume scatter plots and LayerChart for standard chart types. Not recommended as the sole chart library due to implementation effort.

---

## Candidate 6: Apache ECharts (via `svelte-echarts`) 🆕

**npm:** [`echarts`](https://www.npmjs.com/package/echarts) v6.0.0 + [`svelte-echarts`](https://www.npmjs.com/package/svelte-echarts) v1.0.0  
**GitHub:** [apache/echarts](https://github.com/apache/echarts) — ⭐ 63,000+  
**License:** Apache-2.0  
**ECharts last published:** August 2024 (stable, mature)  
**svelte-echarts last published:** ~1 year ago

### Overview

Apache ECharts is one of the most feature-rich open-source charting libraries in the world. Used by Alibaba, Baidu, and millions of projects. The `svelte-echarts` wrapper provides a `<Chart>` component for Svelte. Supports both SVG and Canvas rendering.

### Bundle Size

~300–800 KB depending on included modules. **Heavy.** Tree-shakeable via modular imports (`echarts/core` + individual chart/component modules), but even minimal setups are 200+ KB. The `svelte-echarts` wrapper itself is tiny (~2 KB).

### Svelte 5 Compatibility

⚠️ **Adequate but not runes-native.** `svelte-echarts` v1.0.0 works with Svelte 5 but uses basic prop-based reactivity. No runes-specific features. The wrapper is minimal (a single component), so compatibility issues are unlikely, but it's not leveraging Svelte 5 patterns.

### Interactive Event Handling

✅ **Excellent.** ECharts has the most comprehensive event system of any candidate:

```js
chart.on('click', 'series', (params) => {
  console.log(params.name, params.value); // Clicked element data
});
```

Events for every element type: bars, points, pie slices, map regions, legend items, axes. Built-in support for brush selection, data zoom, and tooltip interactions. The click-to-filter pattern is a first-class use case.

### Chart Types Supported

✅ **Most comprehensive of any candidate:**
- Bar (simple, stacked, grouped, waterfall, histogram)
- Line (basic, stacked, step)
- Area (basic, stacked)
- Scatter (basic, bubble)
- Pie/Donut (basic, nested, rose)
- Treemap (first-class)
- Heatmap (first-class)
- Plus: radar, gauge, funnel, sankey, parallel, boxplot, candlestick, map/geo, graph, theme river, sunburst, and 20+ more

Every required type is a first-class, polished component.

### Customization Depth

**Maximum.** The `options` configuration object is massive — thousands of configurable properties. Full control over:
- Every axis property (labels, ticks, grid, ranges)
- Legend (position, orientation, selection mode)
- Tooltip (formatters, triggers, shared mode)
- Colors (palettes, gradients, individual item styles)
- Animations, transitions
- Data zoom (slider, inside/wheel)
- Brush selection tools
- Custom series and mark points

### Performance with Large Datasets

✅ **Excellent.** Canvas rendering mode handles 100,000+ data points. Built-in incremental rendering for large data. WebGL extension (`echarts-gl`) for millions of points. The performance story is unmatched among candidates.

### Tauri/WebView2 Issues

⚠️ **Minor concerns.** ECharts Canvas rendering works in WebView2. Some reports of:
- Initial render flicker in WebView2 when using Canvas mode
- SVG mode is more stable in webview environments
- Font rendering differences between WebView2 and Chrome (CJK characters, icon fonts)
- `echarts-gl` (WebGL) may have issues in some WebView2 configurations

These are generally manageable by using SVG mode or warming up the chart.

### Known Issues

- **Heavy bundle** — even tree-shaken, 200+ KB minimum
- **svelte-echarts wrapper** is minimally maintained (1 year since last publish)
- **Configuration complexity** — the options API has a steep learning curve
- **Chinese ecosystem** — most active community is Chinese-language; English docs are good but community support is mostly Chinese
- **Not Svelte-native** — rendering is entirely ECharts-internal (Canvas/SVG), Svelte just owns the container div
- **Styling isolation** — ECharts manages its own DOM/Canvas; can't use Svelte transitions or Tailwind on chart elements
- **svelte-echarts last updated 1 year ago** — may need forking/maintenance

### Verdict

| Criteria | Rating |
|----------|--------|
| Svelte wrapper quality | ⭐⭐⭐ (minimal wrapper, works) |
| Event handling | ⭐⭐⭐⭐⭐ (best-in-class event system) |
| Chart variety | ⭐⭐⭐⭐⭐ (30+ types, all first-class) |
| Customization | ⭐⭐⭐⭐⭐ (thousands of config options) |
| Performance | ⭐⭐⭐⭐⭐ (Canvas/WebGL, 100K+ points) |
| Bundle size | ⭐⭐ (heavy, even tree-shaken) |
| Svelte 5 ready | ⭐⭐⭐ (works, not runes-native) |

**Recommendation:** ⚠️ **Best for heavy data, worst for Svelte-native feel.** Unmatched performance and chart variety, but the heavy bundle and non-native Svelte experience make it a poor fit for a lightweight desktop analytics tool. Consider only if you need to visualize raw DuckDB results with >50K rows.

---

## Comparison Matrix

| Criteria | Carbon Charts | SveltePlot | Picasso.js | LayerChart | LayerCake | ECharts |
|----------|:---:|:---:|:---:|:---:|:---:|:---:|
| **Svelte native** | ⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐ |
| **Svelte 5 runes** | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Click events** | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Chart variety** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Customization** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Performance** | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Bundle size** | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ |
| **Maintenance** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Tailwind compat** | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ |
| **Tauri compat** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Overall fit** | **5/10** | **7/10** | **7/10** | **9/10** | **6/10** | **6/10** |

---

## Final Recommendation

### Primary Choice: `layerchart` + `svelteplot`

**LayerChart** is the best overall fit for this stack:

1. **Native Svelte components** — idiomatic, composable, works with runes
2. **All required chart types** — bar, line, area, scatter, treemap, pie/donut, heatmap (via Cell)
3. **Built-in interactions** — tooltips, highlights, pan/zoom
4. **Tailwind-compatible** — SVG elements accept class props
5. **Active maintenance** — published 7 days ago, 1.2k stars, growing community
6. **Reasonable bundle** — tree-shakeable, import only what you need

**SveltePlot** complements LayerChart for scenarios where:
- You need a grammar-of-graphics composition model (faceted charts, layered analytical views)
- You want to build custom analytical chart types that LayerChart doesn't cover
- You're composing multiple data layers in a single plot

### For High-Performance Scatter/Large Data: `picasso.js`

If DuckDB returns raw (unaggregated) datasets with >10K rows that need scatter plot visualization, use **picasso.js** in Canvas mode. Wrap it in a Svelte 5 component using `$effect` for lifecycle management. Its zero-dependency profile and built-in brushing system make it ideal for analytics interactions.

### Migration Path from Current Prototypes

| Current | Recommended Action |
|---------|-------------------|
| `@carbon/charts-svelte` | **Replace with LayerChart.** Better Svelte 5 fit, lighter, no IBM design system overhead. |
| `svelteplot` | **Keep.** Complements LayerChart well for analytical compositions. |
| `picasso.js` | **Keep for performance-critical charts.** Wrap in Svelte component for Canvas-mode large data visualization. |

### Implementation Sketch

```
src/lib/charts/
├── index.ts                    # Re-exports
├── layerchart/                 # Primary chart components
│   ├── BarChart.svelte         # Wrapper around LayerChart BarX/BarY
│   ├── LineChart.svelte        # Wrapper around LayerChart Line
│   ├── ScatterChart.svelte     # Wrapper around LayerChart Scatter
│   ├── TreemapChart.svelte     # Wrapper around LayerChart Treemap
│   ├── PieChart.svelte         # Wrapper around LayerChart Arc/Pie
│   └── HeatmapChart.svelte     # Wrapper around LayerChart Cell
├── svelteplot/                 # Analytical compositions
│   ├── FacetedScatter.svelte   # Multi-panel scatter via Plot
│   └── LayeredAnalysis.svelte  # Multi-layer analytical view
├── picasso/                    # High-performance Canvas charts
│   ├── PicassoChart.svelte     # Generic Svelte 5 wrapper
│   └── LargeScatter.svelte     # Canvas-mode scatter for >10K points
└── shared/
    ├── themes.ts               # Shared color palettes, theme config
    ├── interactions.ts         # Click-to-filter event handling
    └── duckdb-adapter.ts       # DuckDB result → chart data transformers
```

---

## Appendix: Quick Reference

| Library | npm Package | Bundle | Svelte Native | Last Publish | ⭐ Stars |
|---------|-------------|--------|:---:|:---:|:---:|
| Carbon Charts | `@carbon/charts-svelte` | ~300 KB | ✅ wrapper | May 9, 2025 | 1,000 |
| SveltePlot | `svelteplot` | ~70 KB | ✅ native | May 3, 2025 | 387 |
| Picasso.js | `picasso.js` | ~100 KB | ❌ wrap needed | May 13, 2025 | 190 |
| **LayerChart** | `layerchart` | ~100 KB | ✅ native | May 7, 2025 | 1,200 |
| LayerCake | `layercake` | ~20 KB | ✅ native | Oct 2024 | 1,800 |
| ECharts | `echarts` + `svelte-echarts` | ~300+ KB | ❌ wrapper | Aug 2024 | 63,000 |
