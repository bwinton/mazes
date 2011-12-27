#!/usr/bin/env node

var util = require("./mazeutils");

// Recursive Division algorithm functions.

var maze = exports;

maze.name = "recursive division algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2011/1/12/maze-generation-recursive-division-algorithm'>explanation</a>";
maze.handlesOwnEnd = true;

function chooseOrientation(width, height) {
  if (width < height)
    return util.HORIZONTAL;
  else if (height < width)
    return util.VERTICAL;
  else
    return [util.HORIZONTAL, util.VERTICAL][util.randint(2)];
}

function divide(l, grid, x, y, width, height, orientation) {
  if ((width < 2) || (height < 2))
    return;

  var horizontal = (orientation == util.HORIZONTAL);

  // where will the wall be drawn from?
  var wx = x + (horizontal ? 0 : util.randint(width-2));
  var wy = y + (horizontal ? util.randint(height-2) : 0);

  // where will the passage through the wall exist?
  var px = wx + (horizontal ? util.randint(width) : 0);
  var py = wy + (horizontal ? 0 : util.randint(height));

  // what direction will the wall be drawn?
  var dx = horizontal ? 1 : 0;
  var dy = horizontal ? 0 : 1;

  // how long will the wall be?
  var length = horizontal ? width : height;

  // what direction is perpendicular to the wall?
  var dir = horizontal ? util.S : util.E;

  for (var i = 0; i < length; i++) {
    if ((wx != px) || (wy != py))
      grid[wy][wx] &= ~dir;
    wx += dx;
    wy += dy;
  }

  var nx = x;
  var ny = y;
  var w = horizontal ? width : wx-x+1;
  var h = horizontal ? wy-y+1 : height;
  divide(l+1, grid, nx, ny, w, h, chooseOrientation(w, h));

  nx = horizontal ? x : wx+1;
  ny = horizontal ? wy+1 : y;
  w = horizontal ? width : x+width-wx-1;
  h = horizontal ? y+height-wy-1 : height;
  divide(l+1, grid, nx, ny, w, h, chooseOrientation(w, h));
}

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var grid = util.newGrid(size, util.S|util.E);
  for (var y = 0; y < size; y++) {
    grid[y][size-1] &= ~util.E;
  }
  for (var x = 0; x < size; x++) {
    grid[size-1][x] &= ~util.S;
  }

  divide(1, grid, 0, 0, size, size, chooseOrientation(size, size));
  util.draw_grid(grid, maze.name, res);
}
