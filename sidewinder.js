#!/usr/bin/env node

var util = require("./mazeutils");

// Sidewinder algorithm functions.

var maze = exports;

maze.name = "Sidewinder algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2011/2/3/maze-generation-sidewinder-algorithm'>explanation</a>";
maze.handlesOwnEnd = true;

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var grid = util.newGrid(size, 0);

  for (var y = 0; y < grid.length; y++) {
    var runStart = 0;
    for (var x = 0; x < grid[y].length; x++) {
      if (y > 0 && (x+1 == grid[y].length || util.randint(2) == 0)) {
        // end current run and carve north
        var cell = runStart + util.randint(x - runStart + 1);
        grid[y][cell] |= util.N;
        grid[y-1][cell] |= util.S;
        runStart = x+1;
      } else if (x+1 < grid[y].length) {
        // carve east
        grid[y][x] |= util.E;
        grid[y][x+1] |= util.W;
      }
    }
  }

  util.draw_grid(grid, maze.name, res);
}
