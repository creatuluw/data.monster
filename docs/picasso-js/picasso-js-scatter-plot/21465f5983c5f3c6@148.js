// https://observablehq.com/@miralemd/picasso-js-example@148
function _1(md){return(
md`# Scatter plot`
)}

function _2(html){return(
html`<div id='container' style="height:400px;position:relative;"></div>`
)}

function _data(){return(
[{
  type: 'matrix',
  data: [
    ['Year', 'Month', 'Sales', 'Margin'],
    ['2010', 'Jan', 1106, 7],
    ['2010', 'Feb', 5444, 53],
    ['2010', 'Mar', 147, 64],
    ['2010', 'Apr', 7499, 47],
    ['2010', 'May', 430, 62],
    ['2010', 'June', 9735, 13],
    ['2010', 'July', 7435, 15],
    ['2011', 'Jan', 1482, 45],
    ['2011', 'Feb', 2659, 76],
    ['2011', 'Mar', 1261, 73],
    ['2011', 'Apr', 3085, 56],
    ['2011', 'May', 3035, 91],
    ['2011', 'June', 7691, 88],
    ['2011', 'July', 3012, 81],
    ['2012', 'Jan', 7980, 61],
    ['2012', 'Feb', 2564, 22],
    ['2012', 'Mar', 7957, 98],
    ['2012', 'Apr', 5809, 1],
    ['2012', 'May', 429, 2],
    ['2012', 'June', 6757, 77],
    ['2012', 'July', 9415, 92]
  ]
}]
)}

function _picasso(require){return(
require("picasso.js")
)}

function _pic(picasso){return(
picasso({
  style: {
    '$font-size': '12px',
    '$font-size--l': '18px',
    '$font-family': 'Source Sans Pro'
  }
})
)}

function _c(pic,data){return(
pic.chart({
  element: document.querySelector('#container'),
  data,
  settings: {
    scales: {
      s: {
        data: {
          field: 'Sales'
        },
        expand: 0.2,
        invert: true
      },
      m: {
        data: {
          field: 'Margin'
        },
        expand: 0.1
      },
      col: {
        data: { extract: { field: 'Year' } },
        type: 'color'
      }
    },
    components: [{
      key: 'y-axis',
      type: 'axis',
      scale: 's',
      dock: 'left'
    }, {
      type: 'legend-cat',
      dock: 'right',
      scale: 'col'
    }, {
      key: 'x-axis',
      type: 'axis',
      scale: 'm',
      dock: 'bottom'
    }, {
      key: 'p',
      type: 'point',
      data: {
        extract: {
          field: 'Month',
          props: {
            y: { field: 'Sales' },
            x: { field: 'Margin' },
            group: { field: 'Year' }
          }
        }
      },
      settings: {
        x: { scale: 'm' },
        y: { scale: 's' },
        shape: 'circle',
        size: () => Math.random(),
        strokeWidth: 2,
        stroke: '#fff',
        opacity: 0.8,
        fill: { scale: 'col', ref: 'group' }
      }
    }]
  }
})
)}

export default function define(runtime, observer) {
  const main = runtime.module();
  main.variable(observer()).define(["md"], _1);
  main.variable(observer()).define(["html"], _2);
  main.variable(observer("data")).define("data", _data);
  main.variable(observer("picasso")).define("picasso", ["require"], _picasso);
  main.variable(observer("pic")).define("pic", ["picasso"], _pic);
  main.variable(observer("c")).define("c", ["pic","data"], _c);
  return main;
}
