// https://observablehq.com/@miralemd/picasso-js-line-chart@192
function _1(md){return(
md`# Line chart`
)}

function _2(html){return(
html`<div id='container' style="height:400px;position:relative;"></div>`
)}

function _data()
{
  var arr = [
    ['Year', 'Sales']
  ];
  let s = 0.5;
  for (var i = 0; i < 500; i++) {
    s = s - 2 + 4 * Math.random();
    arr.push([
      new Date(2017, 0, i).valueOf(),
      10000 + s * 10000,
    ]);
  }

  return [{
    type: 'matrix',
    data: arr
  }];
}


function _picasso(require){return(
require("picasso.js")
)}

function _5(picasso,data){return(
picasso.chart({
  element: document.querySelector('#container'),
  data,
  settings: {
    scales: {
      y: {
        data: { field: 'Sales' },
        invert: true,
        expand: 0.2
      },
      t: { data: { extract: { field: 'Year' } } }
    },
    components: [{
      type: 'axis',
      dock: 'left',
      scale: 'y',
      formatter: {
        type: 'd3-number',
        format: '$,.1r'
      },
    },{
      type: 'axis',
      dock: 'bottom',
      scale: 't',
      formatter: {
        type: 'd3-time',
        format: '%Y-%m'
      }
    }, {
      key: 'lines',
      type: 'line',
      data: {
        extract: {
          field: 'Year',
          props: {
            v: { field: 'Sales' }
          }
        }
      },
      settings: {
        coordinates: {
          major: { scale: 't' },
          minor: { scale: 'y', ref: 'v' }
        },
        layers: {
          line: {}
        }
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
  main.variable(observer()).define(["picasso","data"], _5);
  return main;
}
