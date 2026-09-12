// https://observablehq.com/@miralemd/picasso-js-point-matrix@63
function _1(md){return(
md`# Point matrix`
)}

function _2(html){return(
html`<div id='container' style="height:400px;position:relative;"></div>`
)}

function _data()
{
  var arr = [
    ['Year', 'Month', 'Sales', 'Margin']
  ];

  var months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'June', 'July', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
  for (var i = 0; i < 5; i++) {
    for (var m = 0; m < months.length; m++) {
      arr.push([
        String(2010 + i),
        months[m],
        parseFloat((Math.random() * 10000).toFixed(0)),
        parseFloat((Math.random() * 100).toFixed(0))]);
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
      years: {
        data: {
          extract: { field: 'Year' }
        }
      },
      months: {
        data: {
          extract: { field: 'Month' }
        }
      },
      s: {
        data: {
          field: 'Sales'
        }
      },
      col: {
        data: { field: 'Sales' },
        type: 'color'
      }
    },
    components: [{
      key: 'y-axis',
      type: 'axis',
      scale: 'years',
      dock: 'left'
    }, {
      key: 'x-axis',
      type: 'axis',
      scale: 'months',
      dock: 'bottom'
    }, {
      key: 'p',
      type: 'point',
      data: {
        extract: {
          field: 'Month',
          props: {
            mm: { field: 'Month' },
            size: { field: 'Sales' },
            group: { field: 'Year' }
          }
        }
      },
      settings: {
        x: { scale: 'months' },
        y: { scale: 'years', ref: 'group' },
        size: { scale: 's' },
        fill: { scale: 'col', ref: 'size' }
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
