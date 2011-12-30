#!/usr/bin/env node

var util = require("./mazeutils");

// Hunt and kill algorithm functions.

var maze = exports;

maze.name = "Hunt and kill algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2011/1/24/maze-generation-hunt-and-kill-algorithm'>explanation</a>";
maze.handlesOwnEnd = true;

function walk(grid, x, y) {
  var directions = [util.N, util.S, util.E, util.W];
  util.shuffle(directions);
  for (var i in directions) {
    var direction = directions[i];
    var nx = x + util.DX[direction];
    var ny = y + util.DY[direction];
    if (0 <= ny && ny <= grid.length-1 &&
        0 <= nx && nx <= grid[ny].length-1 &&
        grid[ny][nx] == 0) {
      grid[y][x] |= direction;
      grid[ny][nx] |= util.OPPOSITE[direction];
      return [nx, ny];
    }
  }
  return [null, null];
}

function hunt(grid) {
  for (var y = 0; y < grid.length; y++) {
    for (var x = 0; x < grid[y].length; x++) {
      if (grid[y][x] != 0)
        continue;
      var neighbors = [];
      if (y > 0 && grid[y-1][x] != 0)
        neighbors.push(util.N);
      if (x > 0 && grid[y][x-1] != 0)
        neighbors.push(util.W);
      if (x+1 < grid[y].length && grid[y][x+1] != 0)
        neighbors.push(util.E);
      if (y+1 < grid.length && grid[y+1][x] != 0)
        neighbors.push(util.S);
      if (!neighbors.length)
        continue;
      var direction = neighbors[util.randint(neighbors.length)];
      var nx = x + util.DX[direction];
      var ny = y + util.DY[direction];
      grid[y][x] |= direction;
      grid[ny][nx] |= util.OPPOSITE[direction];
      return [x, y];
    }
  }
  return [null, null];
}


maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var grid = util.newGrid(size, 0);

  var x = util.randint(size);
  var y = util.randint(size);

  while (x !== null) {
    var rv = walk(grid, x, y);
    x = rv[0]; y = rv[1];
    if (x === null) {
      rv = hunt(grid);
      x = rv[0]; y = rv[1];
    }
  }

  util.draw_grid(grid, maze.name, res);
}
