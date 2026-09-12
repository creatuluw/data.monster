# picasso.js Click Event & Brush Interaction API Research

## 1. `chart.component('key').on('click', callback)` - How It Works

### Key Finding: This pattern does NOT exist as a standard picasso.js API

The `chart.component(key)` returns a context object with only `type` and `key` properties. There is **no `.on()` method** on the returned Component object. The picasso.js event system works differently from what you might expect.

**Source: `packages/picasso.js/src/core/chart/index.js` (line `instance.component`):**
```js
instance.component = (key) => {
  const component = componentsC.findComponentByKey(key);
  return component?.instance.ctx;
};
```

The returned `ctx` (instance context) has `.emit()` bound to an internal EventEmitter, but **`.on()` is NOT exposed** on the context. The `emit` is set up in `component-factory.js`:

```js
// component-factory.js - setUpEmitter
function setUpEmitter(ctx, emitter, settings) {
  Object.keys(settings.on || {}).forEach((event) => {
    const listener = settings.on[event].bind(ctx);
    ctx.eventListeners.push({ event, listener });
    emitter.on(event, listener);
  });
  ctx.emit = (name, ...event) => emitter.emit(name, ...event);
}
```

### How to register component event handlers instead

Use the `on` property in the **component definition** or **component settings** (chart config):

**Option A: In the component definition (when creating custom components):**
```js
picasso.component('myComponent', {
  on: {
    customEvent: function(e) {
      // 'this' is bound to the definition context
      console.log('Custom event received', e);
    }
  },
  // ... other lifecycle methods
});
```

**Option B: In the chart settings component config:**
```js
{
  type: 'box',
  key: 'myBars',
  on: {
    customEvent: function(e) {
      // 'this' is bound to the instance context
      console.log('Custom event received', e);
    }
  }
}
```

Then emit from elsewhere:
```js
chart.component('myBars').emit('customEvent', { some: 'data' });
```

### Event object properties available

When using custom `emit()`, the event payload is whatever you pass. There is **no standard click event object** from picasso.js — the library does NOT emit 'click' events natively on components. Click handling is done through the brush trigger system or native interactions.

---

## 2. `brush.trigger` with `on: 'tap'` vs `component.on('click')`

### `brush.trigger` with `on: 'tap'` — The Recommended Approach

This is the **primary and recommended mechanism** for click-to-select interactions in picasso.js. It is declarative, built-in, and handles all the complexity for you.

**How it works (source: `brushing.js` + `chart/index.js`):**

1. The chart registers DOM event listeners on the chart element:
   - `mousedown` → `mouseup` (mouse taps)
   - `touchstart` → `touchend` (touch taps)
2. A valid "tap" is validated: `button === 0`, movement < 12px, duration < 300ms
3. For each visible component under the tap point (top-to-bottom order), `onBrushTap(e)` is called
4. Each trigger's `resolveTapEvent()` runs collision detection via `renderer.itemsAt(point)`
5. Colliding scene nodes' `data` properties are extracted
6. The brush action (`toggle`/`add`/`set`/`remove`) is applied to the specified contexts

**Source: `packages/picasso.js/src/core/chart/index.js` — `addDefaultEventListeners`:**
```js
const onTapDown = (e) => {
  if (e.touches) {
    eventInfo.x = e.touches[0].clientX;
    eventInfo.y = e.touches[0].clientY;
    eventInfo.multiTouch = e.touches.length > 1;
  } else {
    eventInfo.x = e.clientX;
    eventInfo.y = e.clientY;
    eventInfo.multiTouch = false;
  }
  eventInfo.time = Date.now();
  eventInfo.comps = componentsFromPoint(eventInfo);
};

const onBrushTap = (e) => {
  const comps = eventInfo.comps || componentsFromPoint(e);
  if (comps.every((c) => c.instance.def.disableTriggers)) {
    return;
  }
  if (e.type === 'touchend') {
    e.preventDefault();
  }
  if (!isValidTapEvent(e, eventInfo)) {
    return;
  }
  for (let i = comps.length - 1; i >= 0; i--) {
    const comp = comps[i];
    comp.instance.onBrushTap(e);
    if (stopBrushing) {
      stopBrushing = false;
      break;
    }
  }
};
```

**Source: `packages/picasso.js/src/core/component/brushing.js` — `resolveTapEvent`:**
```js
export function resolveTapEvent({ e, t, config }) {
  const collisions = resolveCollisions(e, t, config.renderer);
  return resolveEvent({
    collisions,
    t,
    config,
    action: resolveAction(t.action, e, 'toggle'),
  });
}

function resolveCollisions(e, t, renderer) {
  const rect = renderer.element().getBoundingClientRect();
  let p = isTouchEvent(e) ? touchSingleContactPoint(e, rect) : singleContactPoint(e, rect);
  if (p === null || p.x < 0 || p.y < 0 || p.x > rect.width || p.y > rect.height) {
    return [];
  }
  if (t.touchRadius > 0 && isTouchEvent(e)) {
    p = { cx: p.x, cy: p.y, r: t.touchRadius };
  } else if (t.mouseRadius > 0 && !isTouchEvent(e)) {
    p = { cx: p.x, cy: p.y, r: t.mouseRadius };
  }
  return renderer.itemsAt(p);
}
```

### `on: 'over'` trigger

The `on: 'over'` trigger fires on `mousemove` and uses action `'set'` by default (instead of `'toggle'`). This is used for hover interactions like tooltips.

### Comparison Table

| Feature | `brush.trigger` `on: 'tap'` | `component.on('click')` |
|---|---|---|
| Built-in | Yes | No (not a standard API) |
| Collision detection | Automatic via `renderer.itemsAt()` | N/A |
| Data extraction | Automatic from scene node `data` | N/A |
| Brush integration | Automatic (toggle/add/set/remove) | Manual |
| Touch support | Built-in with configurable radius | N/A |
| Propagation control | Yes (`propagation`, `globalPropagation`) | N/A |
| Works with SVG renderer | Yes | N/A |
| Works with Canvas renderer | Yes | N/A |
| Recommended for selection | **Yes** | No |

### Native interaction events (alternative)

For raw DOM events without brush integration, use the `interactions` array:

```js
settings: {
  interactions: [{
    type: 'native',
    key: 'myClickHandler',
    events: {
      click: function(e) {
        // Raw DOM event - 'this.chart' gives chart access
        const shapes = this.chart.shapesAt(
          { x: e.offsetX, y: e.offsetY, width: 1, height: 1 },
          { propagation: 'stop' }
        );
        shapes.forEach((shape) => {
          console.log('Clicked shape data:', shape.data);
        });
      }
    }
  }]
}
```

**Source URL:** https://qlik.dev/extend/create-viz-picasso/main-concepts/interaction

---

## 3. Working Example: Bar Chart Click-to-Select/Brush

### Complete bar chart with click-to-select using `box` component

```js
const chart = picasso.chart({
  element: document.querySelector('#container'),
  data: [{
    type: 'matrix',
    data: [
      ['Product', 'Sales'],
      ['Widget A', 150],
      ['Widget B', 280],
      ['Widget C', 95],
      ['Widget D', 310],
      ['Widget E', 220],
    ]
  }],
  settings: {
    scales: {
      x: { data: { field: 'Product' }, type: 'band' },
      y: { data: { field: 'Sales' }, include: [0] },
      color: { data: { field: 'Product' }, type: 'color' },
    },
    components: [
      {
        type: 'box',
        key: 'barChart',
        data: {
          extract: {
            field: 'Product',
            props: {
              start: { field: 'Sales' },
              end: { field: 'Sales' },
              fill: { field: 'Product' },
            }
          }
        },
        settings: {
          major: { scale: 'x' },
          minor: { scale: 'y' },
          box: {
            fill: { scale: 'color', ref: 'fill' },
            strokeWidth: 0,
          }
        },
        brush: {
          trigger: [{
            on: 'tap',
            action: 'toggle',
            contexts: ['selection'],
            data: [''],
            propagation: 'stop',
          }],
          consume: [{
            context: 'selection',
            style: {
              active: {
                fill: '#4477AA',
                stroke: '#224466',
                strokeWidth: 2,
              },
              inactive: {
                opacity: 0.3,
              },
            }
          }]
        }
      },
      {
        type: 'axis',
        scale: 'x',
        dock: 'bottom',
      },
      {
        type: 'axis',
        scale: 'y',
        dock: 'left',
      }
    ]
  }
});

// Listen for brush updates
chart.brush('selection').on('update', function(added, removed) {
  console.log('Added:', added);     // [{ id: 'fieldName', values: [...] }]
  console.log('Removed:', removed); // [{ id: 'fieldName', values: [...] }]
});
```

### Working example: Multiple linked charts with cross-brushing

```js
const barChart = picasso.chart({
  element: document.querySelector('#barContainer'),
  data: [/* bar data */],
  settings: {
    components: [{
      type: 'box',
      key: 'bars',
      brush: {
        trigger: [{ on: 'tap', action: 'toggle', contexts: ['highlight'] }],
        consume: [{
          context: 'highlight',
          style: { inactive: { opacity: 0.3 } }
        }]
      }
    }]
  }
});

const scatterChart = picasso.chart({
  element: document.querySelector('#scatterContainer'),
  data: [/* scatter data */],
  settings: {
    components: [{
      type: 'point',
      key: 'points',
      brush: {
        trigger: [{ on: 'tap', action: 'toggle', contexts: ['highlight'] }],
        consume: [{
          context: 'highlight',
          style: { inactive: { opacity: 0.3 } }
        }]
      }
    }]
  }
});

// Link brushes between charts
barChart.brush('highlight').link(scatterChart.brush('highlight'));
```

**Source URL:** https://qlik.dev/extend/create-viz-picasso/main-concepts/brushing

### Working example from hammer plugin repo (custom component with events)

**Source: `examples/hammer/index.js`**
```js
picasso.component('draw', {
  require: ['chart', 'renderer'],
  created() {
    this.state = { points: [] };
  },
  render() {
    this.drawing = {
      el: this.renderer.element().getBoundingClientRect(),
      sx: this.chart.scale('x'),
      sy: this.chart.scale('y'),
    };
    return render(this, this.state);
  },
  on: {
    start: function(e) {
      // Custom event handler
      var p = getPoint(e, this);
      this.state.points = [p, p];
      render(this, this.state);
    },
    move: function(e) {
      this.state.points[1] = getPoint(e, this);
      render(this, this.state, true);
    },
    end: function(e) {
      this.state.points[1] = getPoint(e, this);
      render(this, this.state, true);
    },
  },
});

// Emitting to the custom component from a hammer interaction:
interactions: [{
  type: 'hammer',
  gestures: [{
    type: 'Pan',
    events: {
      drawstart: function(e) {
        var hitComp = this.chart.componentsFromPoint({ x: e.center.x, y: e.center.y })
          .filter(function(c) { return c.settings.key === 'drawOnMe'; })[0];
        if (!hitComp) return;
        this.chart.component('drawOnMe').emit('start', e);
      },
      drawmove: function(e) {
        this.chart.component('drawOnMe').emit('move', e);
      },
      drawend: function(e) {
        this.chart.component('drawOnMe').emit('end', e);
      },
    }
  }]
}]
```

**Source URL:** https://github.com/qlik-oss/picasso.js/blob/master/examples/hammer/index.js

---

## 4. Using `brush.toggleValue` / `brush.toggleValues` with Datum from Click Events

### The automatic way (recommended)

When using `brush.trigger`, the toggle is handled automatically. You don't need to call `toggleValue` manually. The `resolveTapEvent` → `resolveEvent` → `brushFromSceneNodes` → `brushDataPoints` pipeline handles everything:

**Source: `packages/picasso.js/src/core/component/brushing.js`:**
```js
export function brushDataPoints({ dataPoints, action, chart, trigger }) {
  if (!trigger) return;

  const dataProps = trigger.data || [''];
  let valueBrush = { items: [], actionFn: 'toggleValues' };

  if (['add', 'remove', 'set', 'toggle'].indexOf(action) !== -1) {
    valueBrush.actionFn = `${action}Values`;
  }

  for (let i = 0; i < dataPoints.length; i++) {
    const dataPoint = dataPoints[i];
    if (!dataPoint) continue;
    dataProps.forEach((p) => {
      let d = dataPoint && !p ? dataPoint : dataPoint[p];
      if (d) {
        let it = { key: d.source.field };
        if (typeof d.source.key !== 'undefined') {
          it.key = `${d.source.key}/${d.source.field}`;
        }
        if (Array.isArray(d.value)) {
          it.range = { min: d.value[0], max: d.value[1] };
          rangeBrush.items.push(it);
        } else {
          it.value = d.value;
          valueBrush.items.push(it);
        }
      }
    });
  }

  trigger.contexts.forEach((c) => {
    chart.brush(c)[valueBrush.actionFn](valueBrush.items);
  });
}
```

### Scene node `data` (datum) structure

When you use `data.extract`, each rendered shape gets a `data` property with this structure:

```js
// For a node created by data.extract with props: { x: {field: 'Sales'}, fill: {field: 'Product'} }
node.data = {
  value: 'Widget A',          // Primary field value (from extract.field)
  source: { field: 'Product' },  // Primary field info
  label: 'Widget A',
  // Additional props:
  x: {
    value: 150,
    source: { field: 'Sales' }
  },
  fill: {
    value: 'Widget A',
    source: { field: 'Product' }
  }
};
```

### Manual brush toggling

If you need to manually toggle values (e.g., from a native interaction click handler):

```js
// Method 1: toggleValue - single key/value pair
chart.brush('selection').toggleValue('Product', 'Widget A');

// Method 2: toggleValues - array of items
chart.brush('selection').toggleValues([
  { key: 'Product', value: 'Widget A' }
]);

// Method 3: If using dataset keys
chart.brush('selection').toggleValues([
  { key: 'myDataset/Product', value: 'Widget A' }
]);
```

### Using `shapesAt()` to get datum from a click coordinate

```js
settings: {
  interactions: [{
    type: 'native',
    key: 'clickHandler',
    events: {
      click: function(e) {
        const rect = this.chart.element.getBoundingClientRect();
        const shapes = this.chart.shapesAt(
          { x: e.clientX - rect.left, y: e.clientY - rect.top, width: 1, height: 1 },
          { components: [{ key: 'barChart', propagation: 'stop' }] }
        );
        
        shapes.forEach((shape) => {
          const datum = shape.data;
          // datum.value, datum.source.field, datum.x.value, etc.
          this.chart.brush('selection').toggleValue(datum.source.field, datum.value);
        });
      }
    }
  }]
}
```

### Brush API complete reference

**Source: `packages/picasso.js/src/core/brush/brush.js`:**

```js
const b = chart.brush('myContext');

// Value operations
b.addValue(key, value);           // Add a single value
b.addValues([{ key, value }]);    // Add multiple values
b.removeValue(key, value);        // Remove a single value
b.removeValues([{ key, value }]); // Remove multiple values
b.toggleValue(key, value);        // Toggle a single value
b.toggleValues([{ key, value }]); // Toggle multiple values
b.setValues([{ key, value }]);    // Replace all values

// Range operations
b.addRange(key, { min, max });
b.toggleRange(key, { min, max });
b.setRange(key, { min, max });
b.removeRange(key, { min, max });

// Lifecycle
b.start();                        // Activate brush (emits 'start')
b.end();                          // Deactivate and clear (emits 'end')
b.clear();                        // Clear without deactivating

// Query
b.isActive();                     // Is brush started?
b.containsValue(key, value);      // Is value in brush?
b.brushes();                      // Get all active brushes/values

// Events
b.on('start', () => { });
b.on('update', (added, removed) => {
  // added: [{ id: 'key', values: [...] }]
  // removed: [{ id: 'key', values: [...] }]
});
b.on('end', () => { });

// Interceptors
b.intercept('add-values', (items) => {
  // Transform items before adding
  return items;
});
```

---

## 5. SVG Renderer vs Canvas Renderer for Click Events

### Both renderers support identical click/brush interaction

The brush trigger system is renderer-agnostic. Both SVG and Canvas renderers implement the same `itemsAt()` interface used by `resolveCollisions()`:

```js
// brushing.js - resolveCollisions
function resolveCollisions(e, t, renderer) {
  const rect = renderer.element().getBoundingClientRect();
  let p = isTouchEvent(e)
    ? touchSingleContactPoint(e, rect)
    : singleContactPoint(e, rect);
  // ...
  return renderer.itemsAt(p);  // Same API for both renderers
}
```

### Practical differences

| Aspect | SVG Renderer | Canvas Renderer |
|---|---|---|
| Hit testing | DOM-based (native SVG events) + geometric collision | Geometric collision only |
| `itemsAt()` implementation | Uses scene graph collider index | Uses scene graph collider index |
| Mouse hover effects | CSS `:hover` works naturally | No native hover — must use `on: 'over'` trigger |
| Screen reader | Supported (disable with `rendererSettings.disableScreenReader`) | Not supported |
| Performance (many shapes) | Slower (DOM overhead) | Faster (single element) |
| `touchRadius` / `mouseRadius` | Supported | Supported |
| Progressive rendering | Not supported | Supported via `rendererSettings.progressive` |
| Transform without re-render | Supported | Supported |

### Renderer selection

```js
// Default: SVG first, then Canvas fallback
picasso().chart({ ... });

// Force canvas renderer
picasso({ renderer: { prio: ['canvas'] } }).chart({ ... });

// Force SVG renderer
picasso({ renderer: { prio: ['svg'] } }).chart({ ... });
```

### Note on `componentsFromPoint`

The chart-level `componentsFromPoint()` uses geometric bounding rect checks, not renderer-specific logic. It works identically for both renderers:

```js
// chart/index.js
const componentsFromPoint = (p) => {
  const br = element.getBoundingClientRect();
  const x = 'clientX' in p ? p.clientX : p.x;
  const y = 'clientY' in p ? p.clientY : p.y;
  const tp = { x: x - br.left, y: y - br.top };
  const ret = [];
  visibleComponents.forEach((c) => {
    const r = c.instance.getRect();
    if (testRectPoint(r.computedPhysical || {...}, tp)) {
      ret.push(c);
    }
  });
  return ret;
};
```

---

## Summary: Recommended Patterns

### For bar/box chart click-to-select:

```js
{
  type: 'box',
  key: 'myBars',
  brush: {
    trigger: [{
      on: 'tap',
      action: 'toggle',
      contexts: ['selection'],
      data: [''],          // All mapped data props
      propagation: 'stop', // Only topmost shape
    }],
    consume: [{
      context: 'selection',
      style: {
        active: { fill: '#4477AA', stroke: '#333', strokeWidth: 2 },
        inactive: { opacity: 0.3 },
      }
    }]
  }
}
```

### For programmatic brush control:

```js
chart.brush('selection').on('update', (added, removed) => {
  // React to selection changes
});
```

### For cross-chart linking:

```js
chart1.brush('selection').link(chart2.brush('highlight'));
```

---

## Source URLs

- **picasso.js GitHub repo:** https://github.com/qlik-oss/picasso.js
- **Brush trigger source:** https://github.com/qlik-oss/picasso.js/blob/master/packages/picasso.js/src/core/component/brushing.js
- **Component factory (event handling):** https://github.com/qlik-oss/picasso.js/blob/master/packages/picasso.js/src/core/component/component-factory.js
- **Chart instance (tap handling):** https://github.com/qlik-oss/picasso.js/blob/master/packages/picasso.js/src/core/chart/index.js
- **Brush API source:** https://github.com/qlik-oss/picasso.js/blob/master/packages/picasso.js/src/core/brush/brush.js
- **Event type utilities:** https://github.com/qlik-oss/picasso.js/blob/master/packages/picasso.js/src/core/utils/event-type.js
- **Hammer example:** https://github.com/qlik-oss/picasso.js/blob/master/examples/hammer/index.js
- **Brush lasso component:** https://github.com/qlik-oss/picasso.js/blob/master/packages/picasso.js/src/core/chart-components/brush-lasso/brush-lasso.js
- **Official brushing docs:** https://qlik.dev/extend/create-viz-picasso/main-concepts/brushing
- **Official interaction docs:** https://qlik.dev/extend/create-viz-picasso/main-concepts/interaction
- **API reference:** https://qlik.dev/apis/javascript/picasso-js/
- **BrushTriggerSettings type definition:** (in chart/index.js JSDoc)
