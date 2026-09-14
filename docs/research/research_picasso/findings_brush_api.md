# Picasso.js Brush API Research Findings

Sources: GitHub source code, test files, official API docs, and build output.

---

## 1. `brush.toggleValue()` vs `brush.toggleValues()`

**There is no `brush.toggle()` method.** The API exposes `toggleValue` (singular) and `toggleValues` (plural).

### `brush.toggleValue(key, value)`

Toggles a single primitive value. If it exists in the brush, it's removed; if not, it's added.

```js
// Source: brush.js (JSDoc)
brush.toggleValue('countries', 'Sweden');
```

**Parameters:**
- `key` (string) — An identifier that represents the data source
- `value` (string|number) — The value to toggle

Internally delegates to `toggleValues([{ key, value }])`.

### `brush.toggleValues(items)`

Toggles multiple values in a single operation. Only fires one `update` event.

```js
// Source: brush.js
fn.toggleValues = (items) => {
  const its = intercept(interceptors.toggleValues, items, aliases);
  const toggled = toggle({ items: its, values, vc });
  fn.emit('toggle-values', its);
  if (toggled[0].length > 0 || toggled[1].length > 0) {
    if (!activated) {
      activated = true;
      fn.emit('start');
    }
    fn.emit('update', toggled[0], toggled[1]);
    links.updateValues();
  }
};
```

**Parameters:**
- `items` (Array<{key: string, value: string|number}>) — Items to toggle

**What it returns internally (from `toggle()` helper):**
```js
// Source: brush.js exported toggle function
// Returns a tuple: [added, removed]
// added:  Array<{id: string, values: Array}>  — values that were added
// removed: Array<{id: string, values: Array}> — values that were removed
```

### Toggle behavior from tests (brush.spec.js):

```js
// Toggling new values → they get added
it('should toggle on new values', () => {
  const items = [
    { key: 'products', value: 'Bike' },
    { key: 'products', value: 0 },
  ];
  const toggled = toggle({ items, vc: vcoll, values: {} });
  expect(toggled[0]).to.eql([{ id: 'products', values: ['Bike', 0] }]);
});

// Toggling existing values → they get removed
it('should toggle off existing values', () => {
  const items = [
    { key: 'products', value: 'Bike' },
    { key: 'products', value: 'Existing' },
    { key: 'products', value: 'Car' },
  ];
  v.contains.withArgs('Existing').returns(true);
  const toggled = toggle({ items, vc: vcoll, values: {} });
  expect(toggled[1]).to.eql([{ id: 'products', values: ['Existing'] }]);
});

// Duplicate values in input → deduplicated
it('should toggle duplicate values', () => {
  const items = [
    { key: 'products', value: 'Bike' },
    { key: 'regions', value: 'south' },
    { key: 'regions', value: 'south' },
    { key: 'products', value: 'Bike' },
  ];
  const toggled = toggle({ items, vc: vcoll, values: {} });
  expect(toggled).to.deep.equal([
    [{ id: 'products', values: ['Bike'] }, { id: 'regions', values: ['south'] }],
    [],
  ]);
});
```

**Key insight:** `toggleValue` auto-starts the brush if not active (emits `start`), then emits `update` with `(added, removed)`.

---

## 2. `brush.start()` and `brush.end()`

### `brush.start(...args)`

```js
// Source: brush.js
fn.start = (...args) => {
  if (!activated) {
    activated = true;
    fn.emit('start', ...args);
    links.start();
  }
};
```

- Accepts **any variadic arguments** (`...args`) — all args are forwarded to `start` event listeners
- Only emits `start` event **once** — subsequent calls are no-ops while activated
- Also calls `links.start()` to propagate to linked brushes

**From tests:**
```js
it('should emit a start event with parameters when started', () => {
  const cb = sandbox.spy();
  b.on('start', cb);
  b.start(1, '2', false);
  expect(cb.withArgs(1, '2', false).calledOnce).to.be.true;
});

it('should not emit a start event when already started', () => {
  const cb = sandbox.spy();
  b.on('start', cb);
  b.start();
  b.start();
  b.start();
  expect(cb.callCount).to.equal(1);
});
```

### `brush.end(...args)`

```js
// Source: brush.js
fn.end = (...args) => {
  if (!activated) {
    return;
  }
  activated = false;
  ranges = {};
  values = {};
  fn.emit('end', ...args);
  links.end();
};
```

- Accepts **any variadic arguments** — forwarded to `end` event listeners
- **Clears all state** (resets both `ranges` and `values` to empty objects)
- No-op if not already activated
- Propagates to linked brushes

**From tests:**
```js
it('should emit an end event with parameters when ended', () => {
  const cb = sandbox.spy();
  b.on('end', cb);
  b.start();
  b.end(1, '2', false);
  expect(cb.withArgs(1, '2', false).calledOnce).to.be.true;
  b.end(); // second call is no-op
  expect(cb.callCount).to.equal(1);
});
```

**IMPORTANT:** `addValue`, `toggleValue`, `setValues` etc. all auto-start the brush if not already active. You don't need to call `start()` manually before using these methods.

---

## 3. Reading Brush State — `brush.brushes()`

Yes, `brush.brushes()` is the correct method. It returns an array of objects describing all active brushes.

```js
// Source: brush.js
fn.brushes = () => {
  let result = [];
  result = result.concat(
    Object.keys(ranges).map((key) => ({
      type: 'range',
      id: key,
      brush: ranges[key],
    }))
  );
  result = result.concat(
    Object.keys(values).map((key) => ({
      type: 'value',
      id: key,
      brush: values[key],
    }))
  );
  return result;
};
```

**Return type:** `Array<{type: 'range'|'value', id: string, brush: object}>`

- `type: 'value'` — brush is a `valueCollection` with `.values()`, `.contains()`, `.add()`, `.remove()` methods
- `type: 'range'` — brush is a `rangeCollection` with `.ranges()`, `.containsValue()`, `.containsRange()` methods
- `id` — The key identifier (e.g. `'products'`, `'sales'`)
- `brush` — The actual collection instance

**From tests:**
```js
it('should return all created brushes', () => {
  b.addValue('products');
  b.addRange('sales');
  b.addRange('_aliased');
  expect(b.brushes()).to.eql([
    { type: 'range', id: 'sales', brush: rc },
    { type: 'range', id: 'region', brush: rc },  // aliased key resolved
    { type: 'value', id: 'products', brush: vc },
  ]);
});

it('should return empty after brush is ended', () => {
  b.addValue('products');
  b.addRange('sales');
  expect(b.brushes().length).to.eql(2);
  b.end();
  expect(b.brushes().length).to.eql(0);
});
```

### Getting actual values from a brush entry:

```js
const allBrushes = chart.brush('selection').brushes();

allBrushes.forEach((b) => {
  if (b.type === 'value') {
    const values = b.brush.values(); // returns array of values, e.g. ['Sweden', 'Norway']
  } else if (b.type === 'range') {
    const ranges = b.brush.ranges(); // returns [{min: 10, max: 50}, ...]
  }
});
```

### Additional state query methods:

```js
// Check if a specific value is brushed
brush.containsValue('products', 'Bike'); // → boolean

// Check if a value falls within a brushed range
brush.containsRangeValue('sales', 30); // → boolean

// Check if a specific range segment is contained
brush.containsRange('sales', { min: 15, max: 20 }); // → boolean

// Check if a brush is active at all
brush.isActive(); // → boolean

// Get full state snapshot (internal method)
brush._state(); // → { values: {key: [...]}, ranges: {key: [...]} }
```

---

## 4. Subscribing to Brush Events

Yes, `brush.on('update', callback)` exists. The brush uses an EventEmitter mixin.

### Available events:

| Event | Callback Signature | When Fired |
|-------|-------------------|------------|
| `'start'` | `(...args)` | When brush is activated (once) |
| `'end'` | `(...args)` | When brush is deactivated and cleared |
| `'update'` | `(added, removed)` | When brush state changes |
| `'add-values'` | `(items)` | Before values are added (raw items) |
| `'remove-values'` | `(items)` | Before values are removed |
| `'toggle-values'` | `(items)` | Before values are toggled |
| `'set-values'` | `(items)` | Before values are set |

### The `update` callback parameters:

```js
brush.on('update', (added, removed) => {
  // added:  Array<{id: string, values: Array<string|number>}>
  // removed: Array<{id: string, values: Array<string|number>}>
});
```

**From source:**
```js
// When values are added:
fn.emit('update', added, []);

// When values are removed:
fn.emit('update', [], removed);

// When toggling:
fn.emit('update', toggled[0], toggled[1]); // [added, removed]

// When setting (replaces):
fn.emit('update', changed[0], changed[1]); // [added, removed]
```

**From tests:**
```js
it('should emit an "update" event when state changes', () => {
  const cb = sandbox.spy();
  b.on('update', cb);
  vc.add.returns(true);
  b.addValues([
    { key: 'products', value: 'cars' },
    { key: '_aliased', value: 'sweden' },
  ]);
  expect(cb).to.have.been.calledWith(
    [
      { id: 'products', values: ['cars'] },
      { id: 'region', values: ['sweden'] },  // alias resolved
    ],
    []
  );
});
```

### Removing listeners:

```js
brush.removeListener('update', callback);
```

### Interceptors (middleware pattern):

You can intercept brush operations before they happen:

```js
brush.intercept('add-values', (items) => {
  console.log('about to add:', items);
  return items; // must return items (can modify them)
});

brush.intercept('toggle-values', (items) => {
  // Transform items before toggle
  return items;
});
```

Valid interceptor names: `'add-values'`, `'remove-values'`, `'toggle-values'`, `'set-values'`, `'add-ranges'`, `'set-ranges'`, `'remove-ranges'`, `'toggle-ranges'`.

---

## 5. Component Interaction: Trigger/Consume Pattern

The brush interaction with components uses a **trigger** (input) and **consume** (output/display) pattern configured in component settings.

### Trigger Configuration (brush.trigger)

Makes a component **produce** brush events on user interaction:

```js
// Source: BrushTriggerSettings (official API docs)
{
  on: 'tap',                    // 'tap' or 'over'
  action: 'toggle',             // 'add', 'remove', 'set', 'toggle'
  contexts: ['selection'],      // brush context names to affect
  data: ['x'],                  // mapped data properties to brush
  propagation: 'stop',          // 'stop' => stop at first shape
  globalPropagation: 'stop',    // 'stop' => stop at first component
  touchRadius: 24,              // extend touch contact area
  mouseRadius: 10,              // extend mouse contact area
}
```

### Consume Configuration (brush.consume)

Makes a component **react** to brush state changes (e.g., highlighting):

```js
// Source: BrushConsumeSettings (official API docs)
{
  context: 'selection',         // brush context to observe
  data: ['x'],                  // mapped data properties to observe
  mode: 'and',                  // 'and', 'or', 'xor'
  filter: (shape) => shape.type === 'circle',
  style: {
    active: {
      fill: 'red',
      stroke: '#333',
      strokeWidth: (shape) => shape.strokeWidth * 2,
    },
    inactive: {
      opacity: 0.3,
    },
  },
}
```

### Full bar chart example with trigger + consume:

```js
const chart = picasso.chart({
  element: document.querySelector('#container'),
  data: [
    { product: 'A', sales: 100 },
    { product: 'B', sales: 200 },
    { product: 'C', sales: 150 },
  ],
  settings: {
    scales: {
      x: { data: { field: 'product' }, type: 'band' },
      y: { data: { field: 'sales' }, expand: 0.1 },
    },
    components: [
      {
        type: 'axis',
        scale: 'x',
        layout: { dock: 'bottom' },
      },
      {
        type: 'axis',
        scale: 'y',
        layout: { dock: 'left' },
      },
      {
        type: 'box',
        key: 'bars',
        data: {
          extract: {
            field: 'product',
            props: {
              x: { field: 'product' },
              y: { field: 'sales' },
            },
          },
        },
        settings: {
          major: { scale: 'x' },
          minor: { scale: 'y' },
        },
        brush: {
          trigger: [
            {
              on: 'tap',
              action: 'toggle',
              contexts: ['selection'],
              data: ['x'],
            },
          ],
          consume: [
            {
              context: 'selection',
              data: ['x'],
              style: {
                active: {
                  fill: '#4287f5',
                  stroke: '#1a5ab8',
                },
                inactive: {
                  opacity: 0.3,
                },
              },
            },
          ],
        },
      },
    ],
  },
});
```

### How trigger works internally (from build source):

```js
// Source: picasso.js build (brushDataPoints function)
function brushDataPoints({ dataPoints, action, chart, trigger }) {
  const dataProps = trigger.data || [''];
  let valueBrush = { items: [], actionFn: 'toggleValues' };

  if (['add', 'remove', 'set', 'toggle'].indexOf(action) !== -1) {
    valueBrush.actionFn = `${action}Values`;
  }

  for (let i = 0; i < dataPoints.length; i++) {
    const dataPoint = dataPoints[i];
    dataProps.forEach((p) => {
      let d = !p ? dataPoint : dataPoint[p];
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

### How consume works internally (styler):

```js
// Source: picasso.js build (styler function)
// The styler subscribes to brush events and applies styles to nodes
brusher.on('start', onStart);   // Apply inactive styles to all nodes
brusher.on('end', onEnd);       // Restore original styles
brusher.on('update', onUpdate); // Apply active styles to matching nodes

// Cleanup on unmount:
function cleanUp() {
  brusher.removeListener('start', onStart);
  brusher.removeListener('end', onEnd);
  brusher.removeListener('update', onUpdate);
}
```

### Multi-component interaction (bar + legend):

```js
components: [
  {
    type: 'box',
    key: 'myBars',
    data: { extract: { field: 'Region', props: { x: { field: 'Region' }, y: { field: 'Sales' } } } },
    settings: { major: { scale: 'x' }, minor: { scale: 'y' } },
    brush: {
      trigger: [{ on: 'tap', action: 'toggle', contexts: ['selection'], data: ['x'] }],
      consume: [{ context: 'selection', data: ['x'], style: { active: { fill: '#4287f5' }, inactive: { opacity: 0.3 } } }],
    },
  },
  {
    type: 'legend-cat',
    key: 'legend',
    data: { extract: { field: 'Region' } },
    brush: {
      consume: [{ context: 'selection', data: [''], style: { active: { font: 'bold 14px Arial' }, inactive: { opacity: 0.3 } } }],
    },
  },
],
```

---

## 6. Relationship Between `data` Property in Trigger and Brush Storage

The `data` property in a trigger config maps to **data property names** from the component's `data.extract.props` configuration.

### How it works:

1. **Component data extraction** defines named props:
```js
data: {
  extract: {
    field: 'Product',
    props: {
      x: { field: 'Product' },       // prop name = 'x'
      y: { field: 'Sales' },         // prop name = 'y'
      size: { field: 'Customers' },  // prop name = 'size'
    },
  },
},
```

2. **Trigger `data`** references these prop names:
```js
brush: {
  trigger: [{
    data: ['x'],  // references the 'x' prop → brushes the Product field value
  }],
}
```

3. **What gets stored in the brush** depends on the prop's data:

```js
// For each data point the user interacts with:
const d = dataPoint['x'];  // get the mapped prop
// d = { value: 'Cars', source: { field: 'Product' } }

// The brush stores:
// key = d.source.field   → 'Product'
// value = d.value        → 'Cars'
```

4. **If `data` is `['']` (empty string)**, it uses the root datum:
```js
const d = dataPoint;  // the entire datum
// d = { value: 'Cars', source: { field: 'Product' }, ... }
```

5. **Range values** (arrays) are stored as range brushes:
```js
// If d.value is an array like [5, 10]:
it.range = { min: d.value[0], max: d.value[1] };
// Stored via brush.addRange / brush.toggleRange

// If d.value is a scalar like 'Cars':
it.value = d.value;
// Stored via brush.addValue / brush.toggleValue
```

6. **Source key composition**: If the data source has a `key`, the brush key is `{key}/{field}`:
```js
if (typeof d.source.key !== 'undefined') {
  it.key = `${d.source.key}/${d.source.field}`;
}
```

### Practical example:

```js
// Setup:
{
  type: 'box',
  data: {
    extract: {
      field: 'Region',
      props: {
        x: { field: 'Region' },     // 'x' → Region values like 'North', 'South'
        y: { field: 'Sales' },      // 'y' → numeric Sales values
      },
    },
  },
  brush: {
    trigger: [{ data: ['x'], action: 'toggle', contexts: ['selection'] }],
  },
}

// User clicks bar for 'North':
// 1. Trigger fires with dataPoint.x = { value: 'North', source: { field: 'Region' } }
// 2. brush.toggleValues([{ key: 'Region', value: 'North' }]) is called
// 3. Brush now contains: { values: { 'Region': ['North'] } }

// To read this outside:
const b = chart.brush('selection');
b.brushes();  // → [{ type: 'value', id: 'Region', brush: valueCollection }]
b.containsValue('Region', 'North');  // → true
```

---

## Complete API Reference Summary

### Brush Instance Methods

| Method | Parameters | Returns | Description |
|--------|-----------|---------|-------------|
| `start(...args)` | any | void | Activate brush, emit 'start' |
| `end(...args)` | any | void | Deactivate and clear, emit 'end' |
| `isActive()` | — | boolean | Is brush active? |
| `clear()` | — | void | Clear all values/ranges, emit 'update' |
| `addValue(key, value)` | string, string\|number | void | Add single value |
| `addValues(items)` | Array<{key, value}> | void | Add multiple values |
| `removeValue(key, value)` | string, string\|number | void | Remove single value |
| `removeValues(items)` | Array<{key, value}> | void | Remove multiple values |
| `toggleValue(key, value)` | string, string\|number | void | Toggle single value |
| `toggleValues(items)` | Array<{key, value}> | void | Toggle multiple values |
| `setValues(items)` | Array<{key, values}> | void | Replace all values |
| `addAndRemoveValues(add, remove)` | Array, Array | void | Atomic add+remove |
| `addRange(key, range)` | string, {min, max} | void | Add range |
| `addRanges(items)` | Array<{key, range}> | void | Add multiple ranges |
| `removeRange(key, range)` | string, {min, max} | void | Remove range |
| `removeRanges(items)` | Array<{key, range}> | void | Remove multiple ranges |
| `toggleRange(key, range)` | string, {min, max} | void | Toggle range |
| `toggleRanges(items)` | Array<{key, range}> | void | Toggle multiple ranges |
| `setRange(key, range)` | string, {min, max} | void | Replace range for key |
| `setRanges(items)` | Array<{key, range}> | void | Replace multiple ranges |
| `brushes()` | — | Array<{type, id, brush}> | Get all active brushes |
| `containsValue(key, value)` | string, any | boolean | Check if value brushed |
| `containsRangeValue(key, value)` | string, number | boolean | Check if value in range |
| `containsRange(key, range)` | string, {min,max} | boolean | Check if range contained |
| `containsMappedData(d, props, mode)` | object, Array, string | boolean | Check complex data match |
| `configure(config)` | {ranges} | void | Configure range settings |
| `intercept(name, handler)` | string, function | void | Add interceptor |
| `removeInterceptor(name, handler)` | string, function | void | Remove interceptor |
| `removeAllInterceptors(name?)` | string? | void | Remove interceptors |
| `addKeyAlias(key, alias)` | string, string | void | Map key to alias |
| `removeKeyAlias(key)` | string | void | Remove alias |
| `link(target)` | Brush | void | Link to another brush |
| `on(event, callback)` | string, function | void | Subscribe to event |
| `removeListener(event, callback)` | string, function | void | Unsubscribe from event |

### Brush Events

| Event | Callback | Description |
|-------|----------|-------------|
| `'start'` | `(...args)` | Brush activated |
| `'end'` | `(...args)` | Brush deactivated/cleared |
| `'update'` | `(added, removed)` | State changed |
| `'add-values'` | `(items)` | Values being added |
| `'remove-values'` | `(items)` | Values being removed |
| `'toggle-values'` | `(items)` | Values being toggled |
| `'set-values'` | `(items)` | Values being set |

---

## Source URLs

- **Brush source code:** https://github.com/qlik-oss/picasso.js/blob/master/packages/picasso.js/src/core/brush/brush.js
- **Brush tests:** https://github.com/qlik-oss/picasso.js/blob/master/packages/picasso.js/src/core/brush/__tests__/brush.spec.js
- **Value collection:** https://github.com/qlik-oss/picasso.js/blob/master/packages/picasso.js/src/core/brush/value-collection.js
- **Range collection:** https://github.com/qlik-oss/picasso.js/blob/master/packages/picasso.js/src/core/brush/range-collection.js
- **Official API docs:** https://qlik.dev/apis/javascript/picasso-js
- **BrushTriggerSettings docs:** https://qlik.dev/apis/javascript/picasso-js/#definitions-brushtriggersettings
- **BrushConsumeSettings docs:** https://qlik.dev/apis/javascript/picasso-js/#definitions-brushconsumesettings
- **Brush interface docs:** https://qlik.dev/apis/javascript/picasso-js/#definitions-brush
- **Hammer example:** https://github.com/qlik-oss/picasso.js/blob/master/examples/hammer/index.js
