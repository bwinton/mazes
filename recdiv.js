#!/usr/bin/env node

// recursive division algorithm functions.

const S = 2;
const E = 4;
const HORIZONTAL = 1;
const VERTICAL = 2;

var mu = require("./lib/mu");
mu.templateRoot = "./templates";

var maze = exports;

maze.name = "recursive division algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2011/1/12/maze-generation-recursive-division-algorithm'>explanation</a>";
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

function divide(l, grid, x, y, width, height, orientation) {
  if ((width < 2) || (height < 2))
    return;

  var horizontal = (orientation == HORIZONTAL);

  // where will the wall be drawn from?
  var wx = x + (horizontal ? 0 : randint(width-2))
  var wy = y + (horizontal ? randint(height-2) : 0)

  // where will the passage through the wall exist?
  var px = wx + (horizontal ? randint(width) : 0)
  var py = wy + (horizontal ? 0 : randint(height))

  // what direction will the wall be drawn?
  var dx = horizontal ? 1 : 0
  var dy = horizontal ? 0 : 1

  // how long will the wall be?
  var length = horizontal ? width : height

  // what direction is perpendicular to the wall?
  var dir = horizontal ? S : E

  for (var i = 0; i < length; i++) {
    if ((wx != px) || (wy != py))
      grid[wy][wx] &= ~dir;
    wx += dx
    wy += dy
  }

  var nx = x;
  var ny = y;
  var w = horizontal ? width : wx-x+1;
  var h = horizontal ? wy-y+1 : height;
  divide(l+1, grid, nx, ny, w, h, chooseOrientation(w, h))

  nx = horizontal ? x : wx+1;
  ny = horizontal ? wy+1 : y;
  w = horizontal ? width : x+width-wx-1;
  h = horizontal ? y+height-wy-1 : height;
  divide(l+1, grid, nx, ny, w, h, chooseOrientation(w, h))
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
      grid[y][x] = S|E;
    }
  }
  for (var y = 0; y < size; y++) {
    grid[y][size-1] &= ~E;
  }
  for (var x = 0; x < size; x++) {
    grid[size-1][x] &= ~S;
  }

  divide(1, grid, 0, 0, size, size, chooseOrientation(size, size))
  maze.draw_grid(grid, res);
}
