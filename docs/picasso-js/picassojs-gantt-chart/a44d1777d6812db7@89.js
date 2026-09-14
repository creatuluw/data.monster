// https://observablehq.com/@miralemd/picassojs-gantt-chart@89
function _1(md){return(
md`# Gantt chart`
)}

function _2(html){return(
html`<div id='container' style="height:400px;position:relative;"></div>`
)}

function _data()
{
  var arr = [
    ['Category', 'Started', 'Ended']
  ];
  
  let start = new Date(2017, 0).valueOf();
  for (var i = 0; i < 12; i++) {
	start = new Date(start + Math.random() * 10 * 864e5).valueOf();
    let end = new Date(start + Math.random() * 90 * 864e5).valueOf();
    arr.push([
      `Task ${i+1}`,
      start,
      end
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
        data: { extract: { field: 'Category' } }
      },
      t: {
        data: { fields: ['Started', 'Ended'] },
        expand: 0.1
      }
    },
    components: [{
      type: 'grid-line',
      x: 't'
    },{
      type: 'axis',
      dock: 'left',
      scale: 'y'
    },{
      type: 'axis',
      dock: 'bottom',
      scale: 't',
      formatter: {
        type: 'd3-time',
        format: '%Y-%m'
      }
    },{
      key: 'bars',
      type: 'box',
      data: {
        extract: {
          field: 'Category',
          props: {
            start: { field: 'Started' },
            end: { field: 'Ended' }
          }
        }
      },
      settings: {
        orientation: 'horizontal',
        major: { scale: 'y' },
        minor: { scale: 't' },
        box: {
          width: 0.8
        }
      }
    }, {
    type: 'labels',
    displayOrder: 2,
    settings: {
      sources: [{
        component: 'bars',
        selector: 'rect',
        strategy: {
          type: 'bar',
          settings: {
            direction: 'right',
            labels: [{
              placements: [{
                position: 'outside'
              }],
              label: node => `${Math.round((node.data.end.value - node.data.start.value) / 864e5)} days`
            }]
          }
        }
      }]
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
