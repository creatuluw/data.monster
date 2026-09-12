// https://observablehq.com/@miralemd/parallel-coordinates-plot@71
function _1(md){return(
md`# Parallel coordinates plot`
)}

function _2(html){return(
html`<div id='container' style="height:400px;position:relative;"></div>`
)}

function _data()
{
  var arr = [
    ['Month', 'High', 'Low', 'Medium', 'Sales', 'Quarter']
  ];

  var months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'June', 'July', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
  for (var m = 0; m < months.length; m++) {
    arr.push([
      months[m],
      parseFloat((500 + Math.random() * 100).toFixed(0)),
      parseFloat((300 + Math.random() * 100).toFixed(0)),
      parseFloat((300 + Math.random() * 100).toFixed(0)),
      parseFloat((300 + Math.random() * 100).toFixed(0)),
      ['Q1', 'Q2', 'Q3', 'Q4'][m % 4]
    ]);
  }
  return [{
    type: 'matrix',
    data: arr
  }];
}


function _fields(data){return(
data[0].data[0]
)}

function _scales(fields)
{
  const s = {};
  fields.forEach(f => {
    s[f] = { data: { field: f } };
  });
  
  return s;
}


function _picasso(require){return(
require("picasso.js")
)}

function _7(picasso,data,fields,scales){return(
picasso.chart({
  element: document.querySelector('#container'),
  data,
  settings: {
    scales: {
      fields: {
        data: fields,
        type: 'band'
      },
      color: {
        data: { extract: { field: 'Month' } },
        type: 'color'
      },
      ...scales
    },
    components: [
      {
        type: 'axis',
        dock: 'bottom',
        scale: 'fields'
      },
      {
        type: 'grid-line',
        x: { scale: 'fields' },
        minorTicks: {
          show: true
        }
      },
      {
        key: 'lines',
        type: 'line',
        data: {
          extract: fields.map(f => ({
            field: f,
            props: {
              name: f,
              color: { field: 'Month' },
              row(a, b, c) {
                return a;
              }
            }
          }))
        },
        require: ['chart'],
        settings: {
          coordinates: {
            major(d) {
              return (
                d.resources.scale('fields')(d.datum.name.label) +
                d.resources.scale('fields').bandwidth() * 0.5
              );
            },
            minor(d) {
              return d.resources.scale(d.datum.name.label)(d.datum.value);
            },
            layerId(a, b, c) {
              return b % 12;
            }
          },
          layers: {
            // curve: 'monotone',
            line: {
              show: true,
              stroke: {
                scale: 'color',
                ref: 'color'
              }
            }
          }
        }
      }
    ]
  }
})
)}

export default function define(runtime, observer) {
  const main = runtime.module();
  main.variable(observer()).define(["md"], _1);
  main.variable(observer()).define(["html"], _2);
  main.variable(observer("data")).define("data", _data);
  main.variable(observer("fields")).define("fields", ["data"], _fields);
  main.variable(observer("scales")).define("scales", ["fields"], _scales);
  main.variable(observer("picasso")).define("picasso", ["require"], _picasso);
  main.variable(observer()).define(["picasso","data","fields","scales"], _7);
  return main;
}
