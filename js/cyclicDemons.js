
function randint(startOrEnd, end) {
  var start = 0;
  if (end)
    start = startOrEnd;
  else
    end = startOrEnd;
  return Math.floor(Math.random() * end) + start;
}

const colours = [
  {r: 255, g: 0, b: 0},
  {r: 255, g: 130, b: 50},
  {r: 255, g: 255, b: 0},
  {r: 130, g: 255, b: 0},
  {r: 0, g: 255, b: 0},
  {r: 0, g: 255, b: 130},
  {r: 0, g: 255, b: 255},
  {r: 0, g: 130, b: 255},
  {r: 0, g: 0, b: 255},
  {r: 130, g: 0, b: 255},
  {r: 255, g: 0, b: 255},
  {r: 255, g: 0, b: 130},
];
for (var c in colours) {
  colours[c].c = c;
}

const delay = 20;

function getNeighbours(x, y) {
  var rv = [];
  // left
  if (x > 0)
    rv.push({x: x-1, y:y});
  // right
  if (x < canvas.width-1)
    rv.push({x: x+1, y:y});
  // up
  if (y > 0)
    rv.push({x: x, y:y-1});
  // down
  if (y < canvas.height-1)
    rv.push({x: x, y:y+1});
  return rv;
}

var canvas = document.getElementById('space');
var ctx = canvas.getContext('2d');
var imgData = ctx.getImageData(0, 0, canvas.width, canvas.height);
var pix = imgData.data;
var bgPix = ctx.createImageData(imgData).data;
for (var y = 0; y < canvas.height; y++) {
  for (var x = 0; x < canvas.width; x++) {
    setInfo(x, y, colours[randint(colours.length)]);
  }
}
ctx.putImageData(imgData, 0, 0);

function toCoord(index) {
  return {x: (index % canvas.width * 4) / 4,
          y: (index / canvas.width * 4)}
}

function toIndex(x, y) {
  if (y === undefined) {
    y = x.y;
    x = x.x;
  }
  return canvas.width * 4 * y + x * 4;
}

function getInfo(x, y) {
  var index = toIndex(x, y);
  return {r: bgPix[index],
          g: bgPix[index+1],
          b: bgPix[index+2],
          c: 255 - bgPix[index+3]}
}

function setInfo(x, y, info) {
  var index = toIndex(x, y);
  pix[index] = info.r;
  pix[index+1] = info.g;
  pix[index+2] = info.b;
  pix[index+3] = 255 - info.c;
}

function update() {
  // Flip the buffers.
  var temp = bgPix; bgPix = pix; pix = temp;
  for (var y = 0; y < canvas.height; y++) {
    for (var x = 0; x < canvas.width; x++) {
      var info = getInfo(x, y);
      var next = (info.c + 1) % colours.length;
      var neighbours = getNeighbours(x, y);
      for (var j = 0; j < neighbours.length; j++) {
        var nInfo = getInfo(neighbours[j]);
        if (nInfo.c == next) {
          info = colours[next];
          break;
        }
      }
      setInfo(x, y, info);
    }
  }
  ctx.putImageData(imgData, 0, 0);
  setTimeout(update, delay);
}

setTimeout(update, delay);
