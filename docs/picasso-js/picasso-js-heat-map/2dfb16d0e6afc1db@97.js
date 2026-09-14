// https://observablehq.com/@miralemd/picasso-js-heat-map@97
function _1(md){return(
md`# Heat map`
)}

function _2(html){return(
html`<div id='container' style="height:400px;position:relative;"></div>`
)}

function _data()
{
  var arr = [
    ['Day', 'Hour', 'Density']
  ];

  for (var i = 0; i < 10; i++) {
    const d = new Date(2018, 1, i + 1);
    for (var m = 0; m < 24; m++) {
      arr.push([
		d.toLocaleString('en-US', { day: 'numeric', month: 'long' }),
        String(m),
        Math.random() * 30
      ]);
    }
  }
  return [{
    type: 'matrix',
    data: arr
  }];
}


function _4(picasso,data){return(
picasso.chart({
  element: document.querySelector('#container'),
  data,
  settings: {
    scales: {
      days: {
        data: {
          extract: { field: 'Day' }
        }
      },
      hours: {
        data: {
          extract: { field: 'Hour' }
        }
      },
      col: {
        data: { field: 'Density' },
        type: 'color',
        range: ['#304D2A','#53763E','#7DA050','#AECC61','#E6F871', '#eee'].reverse(),
        nice: true,
        type: 'threshold-color'
      }
    },
    components: [{
      type: 'legend-cat',
      dock: 'top',
      scale: 'col'
    },{
      key: 'y-axis',
      type: 'axis',
      scale: 'days',
      dock: 'left'
    }, {
      key: 'x-axis',
      type: 'axis',
      scale: 'hours',
      dock: 'bottom'
    }, {
      key: 'p',
      type: 'point',
      data: {
        extract: {
          field: 'Hour',
          props: {
            d: { field: 'Density' },
            group: { field: 'Day' }
          }
        }
      },
      settings: {
        x: { scale: 'hours' },
        y: { scale: 'days', ref: 'group' },
        fill: { scale: 'col', ref: 'd' },
        shape: 'square'
      }
    }]
  }
})
)}

function _picasso(require){return(
require("picasso.js")
)}

export default function define(runtime, observer) {
  const main = runtime.module();
  main.variable(observer()).define(["md"], _1);
  main.variable(observer()).define(["html"], _2);
  main.variable(observer("data")).define("data", _data);
  main.variable(observer()).define(["picasso","data"], _4);
  main.variable(observer("picasso")).define("picasso", ["require"], _picasso);
  return main;
}
