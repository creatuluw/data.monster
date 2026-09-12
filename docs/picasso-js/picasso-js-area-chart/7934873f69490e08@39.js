// https://observablehq.com/@miralemd/picasso-js-area-chart@39
function _1(md){return(
md`# Area chart`
)}

function _2(html){return(
html`<div id='container' style="height:400px;position:relative;"></div>`
)}

function _data()
{
  var arr = [
    ['Year', 'Sales']
  ];

  for (var i = 0; i < 12; i++) {
    arr.push([
      String(2000 + i),
      parseFloat((Math.random() * 1000).toFixed(0))
    ]);
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
      y: {
        data: { field: 'Sales' },
        invert: true,
        expand: 0.2,
        min: 0
      },
      t: { data: { extract: { field: 'Year' } } }
    },
    components: [{
      type: 'axis',
      dock: 'left',
      scale: 'y'
    },{
      type: 'axis',
      dock: 'bottom',
      scale: 't'
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
          curve: 'monotone',
          line: {
            show: false
          },
          area: {}
        }
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
