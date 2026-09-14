// https://observablehq.com/@miralemd/picasso-js-dumbbell-plot@89
function _1(md){return(
md`# Dumbbell plot`
)}

function _2(html){return(
html`<div id='container' style="height:500px;position:relative;"></div>`
)}

function _data()
{
  var arr = [
    ['Dim', 'Low', 'High']
  ];

  var low = 50;
  var high;
  for (var m = 0; m < 24; m++) {
    low = low + Math.random();
    high = low + 1 + Math.random() * 5;
    arr.push([
      String.fromCharCode(65 + m),
      low,
      high
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
    collections: [{
	    key: 'd',
      	data: {
          extract: {
            field: 'Dim',
            props: {
              start: { field: 'Low' },
              end: { field: 'High' }
            }
          }
        }
    }],
    scales: {
      y: {
        data: { extract: { field: 'Dim' } }
      },
      v: {
        data: { fields: ['Low', 'High'] },
        expand: 0.1
      }
    },
    components: [{
      type: 'grid-line',
      y: 'y'
    },{
      type: 'axis',
      dock: 'left',
      scale: 'y'
    },{
      type: 'axis',
      dock: 'bottom',
      scale: 'v'
    },{
      key: 'bars',
      type: 'box',
      data: {
        collection: 'd'
      },
      settings: {
        orientation: 'horizontal',
        major: { scale: 'y' },
        minor: { scale: 'v' },
        box: {
          width: 0.1,
          fill: '#ccc'
        }
      }
    }, {
      type: 'point',
      data: {
        collection: 'd'
      },
      settings: {
        x: { scale: 'v', ref: 'start' },
        y: { scale: 'y' },
        fill: '#fa0',
        size: 0.8
      }
    }, {
      type: 'point',
      data: {
        collection: 'd'
      },
      settings: {
        x: { scale: 'v', ref: 'end' },
        y: { scale: 'y' },
        fill: '#bdf700',
        size: 0.8
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
