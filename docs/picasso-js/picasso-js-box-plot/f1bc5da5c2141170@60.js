// https://observablehq.com/@miralemd/picasso-js-box-plot@60
function _1(md){return(
md`# Box plot`
)}

function _2(html){return(
html`<div id='container' style="height:400px;position:relative;"></div>`
)}

function _data()
{
  var arr = [
    ['Dim', 'min', 'low', 'high', 'max']
  ];
  
  let min = 4;
  for (var m = 0; m < 12; m++) {
    min = min - 0.5 + 1 * Math.random();
    let low = min + Math.random();
    let high = low + 2 * Math.random();
    let max = high + Math.random();
    arr.push([
      String.fromCharCode(65 + m),
      min,
      low,
      high,
      max
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
      v: {
        data: { fields: ['min', 'max'] }
      },
      t: { data: { extract: { field: 'Dim' } }, type: 'band' }
    },
    components: [{
      type: 'axis',
      dock: 'left',
      scale: 't'
    },{
      type: 'axis',
      dock: 'bottom',
      scale: 'v'
    }, {
      type: 'box',
      data: {
        extract: {
          field: 'Dim',
          props: {
            min: { field: 'min' },
            start: { field: 'low' },
            end: { field: 'high' },
            max: { field: 'max' }
          }
        }
      },
      settings: {
        major: { scale: 't' },
        minor: { scale: 'v' },
        orientation: 'horizontal',
        box: {
          width: 0.7
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
