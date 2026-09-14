// https://observablehq.com/@miralemd/picasso-js-donut-chart@45
function _1(md){return(
md`# Donut chart`
)}

function _2(html){return(
html`<div id='container' style="height:400px;position:relative;"></div>`
)}

function _data()
{
  var arr = [
    ['Year', 'Sales', 'Budget']
  ];

  for (var i = 0; i < 8; i++) {
    arr.push([
      String(2010 + i),
      parseFloat((Math.random() * 1000).toFixed(0)),
      parseFloat((Math.random() * 1000).toFixed(0))]);
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
      c: {
        data: { extract: { field: 'Year' } }, type: 'color'
      }
    },
    components: [{
      type: 'legend-cat',
      scale: 'c'
    }, {
      key: 'p',
      type: 'pie',
      data: {
        extract: {
          field: 'Year',
          props: {
            num: { field: 'Sales' }
          }
        }
      },
      settings: {
        padAngle: 0.01,
        slice: {
          cornerRadius: 4,
          innerRadius: 0.4,
          arc: { ref: 'num' },
          fill: { scale: 'c' },
          strokeWidth: 1,
          stroke: 'rgba(255, 255, 255, 0.5)'
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
