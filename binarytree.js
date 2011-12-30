#!/usr/bin/env node

var util = require("./mazeutils");

// Binary tree algorithm functions.

var maze = exports;

maze.name = "Binary tree algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2011/2/1/maze-generation-binary-tree-algorithm'>explanation</a>";
maze.handlesOwnEnd = true;

var directionFunctions = {
  "NW": function(grid, x, y) {
    var directions = [];
    if (y > 0)
      directions.push(util.N);
    if (x > 0)
      directions.push(util.W);
    return directions;
  },
  "NE": function(grid, x, y) {
    var directions = [];
    if (y > 0)
      directions.push(util.N);
    if (x < grid[0].length - 1)
      directions.push(util.E);
    return directions;
  },
  "SW": function(grid, x, y) {
    var directions = [];
    if (y < grid.length - 1)
      directions.push(util.S);
    if (x > 0)
      directions.push(util.W);
    return directions;
  },
  "SE": function(grid, x, y) {
    var directions = [];
    if (y < grid.length - 1)
      directions.push(util.S);
    if (x < grid[0].length - 1)
      directions.push(util.E);
    return directions;
  },
}

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var type = req.params[1];
  if (!(type in directionFunctions)) {
    type = "NW";
  }
  var getDirections = directionFunctions[type];
  var grid = util.newGrid(size, 0);

  for (var y = 0; y < size; y++) {
    for (var x = 0; x < size; x++) {
      var directions = getDirections(grid, x, y);
      if (!directions.length)
        continue;

      var direction = directions[util.randint(directions.length)];
      var nx = x + util.DX[direction];
      var ny = y + util.DY[direction];
      grid[y][x] |= direction;
      grid[ny][nx] |= util.OPPOSITE[direction];
    }
  }

  util.draw_grid(grid, maze.name, res);
}
