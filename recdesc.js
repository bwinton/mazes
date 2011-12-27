#!/usr/bin/env node

var util = require("./mazeutils");

// Recursive Descent functions.

var maze = exports;

maze.name = "Recursive Descent";
maze.link = "<a href='http://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking'>explanation</a>";
maze.handlesOwnEnd = true;

maze.carve_passages_from = function(cx, cy, grid) {
  // work, work, work
  var directions = [util.N, util.S, util.E, util.W];
  util.shuffle(directions);
  for (var i in directions) {
    var direction = directions[i];
    var nx = cx + util.DX[direction];
    var ny = cy + util.DY[direction];
    if (0 <= ny && ny <= grid.length-1 &&
        0 <= nx && nx <= grid[ny].length-1 &&
        grid[ny][nx] == 0) {
      grid[cy][cx] |= direction
      grid[ny][nx] |= util.OPPOSITE[direction]
      maze.carve_passages_from(nx, ny, grid)
    }
  }
};

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var grid = util.newGrid(size, 0);
  maze.carve_passages_from(0, 0, grid);
  util.draw_grid(grid, maze.name, res);
}
