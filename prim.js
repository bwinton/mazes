#!/usr/bin/env node

// Prim’s algorithm functions.

const S = 2;
const E = 4;
const IN = 0x10;
const FRONTIER = 0x20;

var mu = require("./lib/mu");
mu.templateRoot = "./templates";

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
    grid[y][x] |= E;
  else if (nx < x)
    grid[ny][nx] |= E;
  else if (y < ny)
    grid[y][x] |= S;
  else
    grid[ny][nx] |= S;
}

maze.carve_passages = function(grid) {
  var size = grid.length;
  var frontiers = [];
  var frontier = [Math.floor(Math.random() * size),
                  Math.floor(Math.random() * size)];
  mark(frontier, grid, frontiers);
  while (frontiers.length) {
    // Pick a random frontier.
    var i = Math.floor(Math.random() * frontiers.length);
    frontier = frontiers[i];
    frontiers.splice(i, 1);

    // Pick an in-set neighbour of that frontier.
    var n = neighbours(frontier, grid);
    i = Math.floor(Math.random() * n.length);
    var neighbour = n[i];

    // Make a path from the frontier to the neighbour.
    carve(frontier, neighbour, grid);

    // And make the frontier part of the in-set.
    mark(frontier, grid, frontiers);
  }
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
  maze.carve_passages(grid);
  maze.draw_grid(grid, res);
}
