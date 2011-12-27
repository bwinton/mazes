#!/usr/bin/env node

var util = require("./mazeutils");

// Prim’s algorithm functions.

const IN = 0x10;
const FRONTIER = 0x20;

var maze = exports;

maze.name = "Prim’s algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2011/1/10/maze-generation-prim-s-algorithm'>explanation</a>";
maze.handlesOwnEnd = true;

function addFrontier(x, y, grid, frontiers) {
  var size = grid.length;
  if ((x >= 0) && (x < size) &&
      (y >= 0) && (y < size) &&
      !(grid[y][x] & IN) &&
      !(grid[y][x] & FRONTIER)) {
    grid[y][x] |= FRONTIER;
    frontiers.push([x,y]);
  }
}

function mark(xy, grid, frontiers) {
  var x = xy[0];
  var y = xy[1];
  grid[y][x] &= ~FRONTIER;
  grid[y][x] |= IN;
  addFrontier(x-1, y, grid, frontiers);
  addFrontier(x+1, y, grid, frontiers);
  addFrontier(x, y-1, grid, frontiers);
  addFrontier(x, y+1, grid, frontiers);
}

function neighbours(xy, grid) {
  var size = grid.length;
  var rv = [];
  var x = xy[0];
  var y = xy[1];
  if ((x > 0) && (grid[y][x-1] & IN))
    rv.push([x-1, y]);
  if ((x < size-1) && (grid[y][x+1] & IN))
    rv.push([x+1, y]);
  if ((y > 0) && (grid[y-1][x] & IN))
    rv.push([x, y-1]);
  if ((y < size-1) && (grid[y+1][x] & IN))
    rv.push([x, y+1]);
  return rv;
}

function carve(xy, nxny, grid) {
  var x = xy[0];
  var y = xy[1];
  var nx = nxny[0];
  var ny = nxny[1];

  // We only carve to the south and east.
  if (x < nx)
    grid[y][x] |= util.E;
  else if (nx < x)
    grid[ny][nx] |= util.E;
  else if (y < ny)
    grid[y][x] |= util.S;
  else
    grid[ny][nx] |= util.S;
}

maze.carve_passages = function(grid) {
  var size = grid.length;
  var frontiers = [];
  var frontier = [util.randint(size), util.randint(size)];
  mark(frontier, grid, frontiers);
  while (frontiers.length) {
    // Pick a random frontier.
    var i = util.randint(frontiers.length);
    frontier = frontiers[i];
    frontiers.splice(i, 1);

    // Pick an in-set neighbour of that frontier.
    var n = neighbours(frontier, grid);
    i = util.randint(n.length);
    var neighbour = n[i];

    // Make a path from the frontier to the neighbour.
    carve(frontier, neighbour, grid);

    // And make the frontier part of the in-set.
    mark(frontier, grid, frontiers);
  }
}

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var grid = [];
  for (var y = 0; y < size; y++) {
    grid[y] = [];
    for (var x = 0; x < size; x++) {
      grid[y][x] = 0;
    }
  }
  maze.carve_passages(grid);
  util.draw_grid(grid, maze.name, res);
}
