#!/usr/bin/env node

const N = 1;
const S = 2;
const E = 4;
const W = 8;
const DX = {1: 0, 2: 0, 4: 1, 8: -1}
const DY = {1: -1, 2: 1, 4: 0, 8: 0}
const OPPOSITE = {1: S, 2: N, 4: W, 8: E}
const NAMES = ["X", "N", "S", "NS", "E", "NE", "SE", "NSE", "W",
               "NW", "SW", "NSW", "EW", "NEW", "SEW", "NSEW"];

var mu = require("./lib/mu");
mu.templateRoot = "./templates";

// Aldous-Broder algorithm functions.

var maze = exports;

maze.name = "Aldous-Broder algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2011/1/17/maze-generation-aldous-broder-algorithm'>explanation</a>";
maze.handlesOwnEnd = true;


function randint(startOrEnd, end) {
  var start = 0;
  if (end)
    start = startOrEnd;
  else
    end = startOrEnd;
  return Math.floor(Math.random() * end) + start;
}

function chooseOrientation(width, height) {
  if (width < height)
    return HORIZONTAL;
  else if (height < width)
    return VERTICAL;
  else
    return [HORIZONTAL, VERTICAL][randint(2)];
}

maze.asciify_grid = function(grid) {
  var size = grid.length;
  var rv = "<pre>\n " + new Array(size * 2).join("_") + "\n";
  for (var y = 0; y < size; y++) {
    rv += "|";
    for (var x = 0; x < size; x++) {
      var temp = ((grid[y][x] & S) != 0) ? " " : "_";
      if (grid[y][x] & E) {
        temp += (((grid[y][x] | grid[y][x+1]) & S) != 0) ? " " : "_";
      }
      else {
        temp += "|";
      }
      rv += temp;
    }
    rv += "\n";
  }
  rv += "</pre>";
  return rv;
};

maze.draw_grid = function(grid, res) {
  const size = 25;
  var context = {
    "name": maze.name,
    "length": grid.length,
    "width": (grid.length + 1) * size,
    "height": (grid.length + 1) * size,
    "size": size,
    "offset": size / 2,
    "grid": grid,
    "S": S,
    "E": E,
    "ascii": maze.asciify_grid(grid),
  };
  mu.render("nsew.html", context, {}, function (err, output) {
    if (err) {
      throw err;
    }
    output.addListener("data", function(c) { res.write(c); })
          .addListener("end", function() { res.end(); });
  });
};

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var grid = [];
  for (var y = 0; y < size; y++) {
    grid[y] = [];
    for (var x = 0; x < size; x++) {
      grid[y][x] = 0;
    }
  }

  var y = randint(grid.length);
  var x = randint(grid[y].length);
  var remaining = size * size - 1
  var directions = [N, S, E, W];

  while (remaining > 0) {
    directions.sort(function() {return 0.5 - Math.random()});
    for (var i in directions) {
      var direction = directions[i];
      var nx = x + DX[direction];
      var ny = y + DY[direction];
      if (0 <= ny && ny <= grid.length-1 &&
          0 <= nx && nx <= grid[ny].length-1){
        if (grid[ny][nx] == 0) {
          grid[y][x] |= direction;
          grid[ny][nx] |= OPPOSITE[direction];
          remaining -= 1;
        }
        x = nx;
        y = ny;
        break;
      }
    }
  }
  maze.draw_grid(grid, res);
}

