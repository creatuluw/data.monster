// https://observablehq.com/@miralemd/picasso-js-bar-chart@62
function _1(md){return(
md`# Bar chart`
)}

function _2(html){return(
html`<div id='container' style="height:400px;position:relative;"></div>`
)}

function _3(picasso,data){return(
picasso.chart({
  element: document.querySelector('#container'),
  data,
  settings: {
    scales: {
      y: {
        data: { field: 'Sales' },
        invert: true,
        include: [0]
      },
      c: {
        data: { field: 'Sales' },
        type: 'color'
      },
      t: { data: { extract: { field: 'Month' } }, padding: 0.3 },
    },
    components: [{
      type: 'axis',
      dock: 'left',
      scale: 'y'
    },{
      type: 'axis',
      dock: 'bottom',
      scale: 't'
    },{
      key: 'bars',
      type: 'box',
      data: {
        extract: {
          field: 'Month',
          props: {
            start: 0,
            end: { field: 'Sales' }
          }
        }
      },
      settings: {
        major: { scale: 't' },
        minor: { scale: 'y' },
        box: {
          fill: { scale: 'c', ref: 'end' }
        }
      }
    }]
  }
})
)}

function _picasso(require){return(
require("picasso.js")
)}

function _data()
{
  var arr = [
    ['Month', 'Sales']
  ];

  var months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'June', 'July', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
  for (var m = 0; m < months.length; m++) {
    arr.push([
      months[m],
      parseFloat((Math.random() * 10000).toFixed(0))
    ]);
  }
  return [{
    type: 'matrix',
    data: arr
  }];
}


export default function define(runtime, observer) {
  const main = runtime.module();
  main.variable(observer()).define(["md"], _1);
  main.variable(observer()).define(["html"], _2);
  main.variable(observer()).define(["picasso","data"], _3);
  main.variable(observer("picasso")).define("picasso", ["require"], _picasso);
  main.variable(observer("data")).define("data", _data);
  return main;
}
